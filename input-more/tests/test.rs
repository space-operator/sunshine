#[test]
fn test_chain() {
    use core::fmt::Debug;

    use input_core::*;
    use input_more::*;

    type DurationMs = u64;
    type TimestampMs = u64;
    type Coords = (u64, u64);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Switch {
        Keyboard(KeyboardSwitch),
        Mouse(MouseSwitch),
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct KeyboardSwitch(&'static str);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct MouseSwitch(&'static str);

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum AppEvent {
        Undo(u32),
        CreateNode(u32, Coords),
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum BasicAppEventBuilder {
        Undo(u32),
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum PointerAppEventBuilder {
        CreateNode(u32),
        Undo(u32),
    }

    /*#[derive(Clone, Debug, Default)]
    pub struct Context<St, Ma> {
        state: St,
        mapping: Ma,
    }

    impl<St, Ma> Context<St, Ma> {
        pub fn new(state: St, mapping: Ma) -> Self {
            Self { state, mapping }
        }
    }*/

    pub type KeyboardPressMapping = Mapping<KeyboardSwitch, Switch, (), BasicAppEventBuilder>;
    pub type KeyboardReleaseMappingMarker =
        Mapping<KeyboardSwitch, Switch, Option<TimedReleaseEventData>, BasicAppEventBuilder>;
    pub type KeyboardLongPressMappingMarker =
        Mapping<KeyboardSwitch, Switch, TimedLongPressEventData, BasicAppEventBuilder>;
    pub type KeyboardClickExactMappingMarker =
        Mapping<KeyboardSwitch, Switch, TimedClickExactEventData, BasicAppEventBuilder>;

    pub type GlobalMapping = input_more::GlobalMapping<
        KeyboardPressMapping,
        KeyboardReleaseMappingMarker,
        KeyboardLongPressMappingMarker,
        KeyboardClickExactMappingMarker,
    >;

    pub type KeyboardSwitchEvent = SwitchEvent<TimestampMs, KeyboardSwitch, (), (), ()>;
    pub type MouseSwitchEvent = SwitchEvent<TimestampMs, MouseSwitch, Coords, (), ()>;

    pub type Modifiers = input_core::Modifiers<Switch>;
    pub type KeyboardTimedState = TimedState<KeyboardSwitch>;
    pub type MouseTimedState = TimedState<MouseSwitch>;

    pub type CustomState<Ts, Sh> = State<Modifiers, Ts, Sh>;
    pub type CustomScheduler<Sw, Re> =
        SchedulerState<TimestampMs, SwitchEvent<TimestampMs, Sw, (), Modifiers, ()>, Re>;
    pub type KeyboardLongPressScheduler = CustomScheduler<KeyboardSwitch, LongPressHandleRequest>;
    pub type KeyboardClickExactScheduler = CustomScheduler<KeyboardSwitch, ClickExactHandleRequest>;
    pub type MouseLongPressScheduler = CustomScheduler<MouseSwitch, LongPressHandleRequest>;
    pub type MouseClickExactScheduler = CustomScheduler<MouseSwitch, ClickExactHandleRequest>;

    pub type KeyboardPressState = CustomState<KeyboardTimedState, KeyboardLongPressScheduler>;
    pub type KeyboardReleaseState = CustomState<KeyboardTimedState, KeyboardClickExactScheduler>;
    pub type KeyboardLongPressState = CustomState<MouseTimedState, ()>;
    pub type KeyboardClickExactState = CustomState<MouseTimedState, ()>;

    pub type GlobalState = input_more::GlobalState<
        Modifiers,
        KeyboardTimedState,
        MouseTimedState,
        KeyboardLongPressScheduler,
        KeyboardClickExactScheduler,
        MouseLongPressScheduler,
        MouseClickExactScheduler,
    >;

    pub trait Process<Ev>: Sized {
        fn with_event(self, event: Ev, mapping: &GlobalMapping) -> (Self, Option<TimestampMs>);
        fn with_timeout(self, time: TimestampMs) -> Self;
    }

    impl Process<KeyboardSwitchEvent> for KeyboardPressState {
        fn with_event(
            self,
            event: KeyboardSwitchEvent,
            mapping: &GlobalMapping,
        ) -> (Self, Option<TimestampMs>) {
            //let (mapping, modifiers, timed_state, scheduler) = self.split();
            //let filtered_mapping = self.mapping.to_ref_mapping();

            let (modifiers, result) = self
                .modifiers
                .with_press_event(Switch::Keyboard(event.switch));
            result.unwrap();

            let event = event.with_modifiers(modifiers.clone());
            let (timed_state, result) = self.timed_state.with_press_event(event.switch);
            let request = result.unwrap();

            let scheduler = self.scheduler.schedule(event.time, event.clone(), request);
            let next_scheduled = scheduler.next_scheduled().copied();

            //let filtered_mapping = filtered_mapping.filter_by_switch(&event.switch);
            //if filtered_mapping.is_empty() {
            //    return (
            //        Self::new(mapping, modifiers, timed_state, scheduler),
            //        next_scheduled,
            //    );
            //}
            //let filtered_mapping = filtered_mapping.filter_by_modifiers(&modifiers);
            //if filtered_mapping.is_empty() {
            //    return (
            //        Self::new(mapping, modifiers, timed_state, scheduler),
            //        next_scheduled,
            //    );
            //}

            println!("Ev1: {:?}", event);

            (Self::new(modifiers, timed_state, scheduler), next_scheduled)
        }

        fn with_timeout(self, time: TimestampMs) -> Self {
            // TODO: Add filtering
            const KEYBOARD_LONG_PRESS_DURATION: DurationMs = 1000;

            let mut timed_state = self.timed_state;
            let (scheduler, requests) = self
                .scheduler
                .take_scheduled(&(time - KEYBOARD_LONG_PRESS_DURATION));
            for (time, requests) in requests {
                for (event, request) in requests {
                    let (new_timed_state, result) =
                        timed_state.with_long_press_event(event.switch, request);
                    timed_state = new_timed_state;

                    let timed_data = result.unwrap();
                    let event = event.with_timed_data(timed_data);

                    println!("Ev2: {:?}", event);
                }
            }
            Self::new(self.modifiers, timed_state, scheduler)
        }
    }

    impl Process<KeyboardSwitchEvent> for KeyboardReleaseState {
        fn with_event(
            self,
            event: KeyboardSwitchEvent,
            mapping: &GlobalMapping,
        ) -> (Self, Option<TimestampMs>) {
            let (modifiers, result) = self
                .modifiers
                .with_release_event(&Switch::Keyboard(event.switch));
            result.unwrap();

            let event = event.with_modifiers(modifiers.clone());
            let (timed_state, result) = self.timed_state.with_release_event(event.switch);
            let data = result.unwrap();

            let (scheduler, event) = if let Some((timed_event_data, request)) = data {
                (
                    self.scheduler.schedule(event.time, event.clone(), request),
                    event.with_timed_data(Some(timed_event_data)),
                )
            } else {
                (self.scheduler, event.with_timed_data(None))
            };
            let next_scheduled = scheduler.next_scheduled().copied();

            //let filtered_mapping = filtered_mapping.filter_by_switch(&event.switch);
            //if filtered_mapping.is_empty() {
            //    return (Self::new(mapping, modifiers, timed_state, scheduler), None);
            //}
            //let filtered_mapping = filtered_mapping.filter_by_modifiers(&modifiers);
            //if filtered_mapping.is_empty() {
            //    return (Self::new(mapping, modifiers, timed_state, scheduler), None);
            //}
            //let mapping = mapping.filter_by_timed_data(&event.timed_data);
            //if mapping.is_empty() {
            //    return (
            //        Self::new(mapping, modifiers, timed_state, scheduler),
            //        next_scheduled,
            //    );
            //}

            println!("Ev3: {:?}", event);

            (Self::new(modifiers, timed_state, scheduler), next_scheduled)
        }

        fn with_timeout(self, time: TimestampMs) -> Self {
            // TODO: Add filtering
            const KEYBOARD_CLICK_EXACT_DURATION: DurationMs = 300;

            let mut timed_state = self.timed_state;
            let (scheduler, requests) = self
                .scheduler
                .take_scheduled(&(time - KEYBOARD_CLICK_EXACT_DURATION));
            for (time, requests) in requests {
                for (event, request) in requests {
                    let (new_timed_state, result) =
                        timed_state.with_click_exact_event(event.switch, request);
                    timed_state = new_timed_state;

                    let timed_data = result.unwrap();
                    let event = event.with_timed_data(timed_data);

                    println!("Ev4: {:?}", event);
                }
            }
            Self::new(self.modifiers, timed_state, scheduler)
        }
    }

    let mut global_state = GlobalState::default();
    let mut mapping = GlobalMapping::new(
        Mapping::default(),
        Mapping::new(
            [(
                KeyboardSwitch("LeftCtrl"),
                [(
                    Modifiers::new(),
                    [(
                        Some(TimedReleaseEventData {
                            kind: TimedReleaseEventKind::Click,
                            num_possible_clicks: 2,
                        }),
                        vec![BasicAppEventBuilder::Undo(10)],
                    )]
                    .into_iter()
                    .collect(),
                )]
                .into_iter()
                .collect(),
            )]
            .into_iter()
            .collect(),
        ),
        Mapping::default(),
        Mapping::default(),
    );

    let (state, global_state): (KeyboardPressState, _) = global_state.take_state();
    let state = state.with_timeout(1000);
    let global_state = global_state.with_state(state);
    let (state, global_state): (KeyboardReleaseState, _) = global_state.take_state();
    let state = state.with_timeout(1000);
    let global_state = global_state.with_state(state);

    let (state, global_state): (KeyboardPressState, _) = global_state.take_state();
    let (state, scheduled) = state.with_event(
        KeyboardSwitchEvent::new(1000, KeyboardSwitch("LeftCtrl"), ()),
        &mapping,
    );
    let global_state = global_state.with_state(state);
    println!("St: {:?}", global_state);
    println!("Sh: {:?}", scheduled);
    println!();

    let (state, global_state): (KeyboardPressState, _) = global_state.take_state();
    let state = state.with_timeout(1100);
    let global_state = global_state.with_state(state);
    let (state, global_state): (KeyboardReleaseState, _) = global_state.take_state();
    let state = state.with_timeout(1100);
    let global_state = global_state.with_state(state);

    let (state, global_state): (KeyboardReleaseState, _) = global_state.take_state();
    let (state, scheduled) = state.with_event(
        KeyboardSwitchEvent::new(1100, KeyboardSwitch("LeftCtrl"), ()),
        &mapping,
    );
    let global_state = global_state.with_state(state);
    println!("St: {:?}", global_state);
    println!("Sh: {:?}", scheduled);
    println!();

    let (state, global_state): (KeyboardPressState, _) = global_state.take_state();
    let state = state.with_timeout(1200);
    let global_state = global_state.with_state(state);
    let (state, global_state): (KeyboardReleaseState, _) = global_state.take_state();
    let state = state.with_timeout(1200);
    let global_state = global_state.with_state(state);

    let (state, global_state): (KeyboardPressState, _) = global_state.take_state();
    let (state, scheduled) = state.with_event(
        KeyboardSwitchEvent::new(1200, KeyboardSwitch("LeftCtrl"), ()),
        &mapping,
    );
    let global_state = global_state.with_state(state);
    println!("St: {:?}", global_state);
    println!("Sh: {:?}", scheduled);
    println!();

    let (state, global_state): (KeyboardPressState, _) = global_state.take_state();
    let state = state.with_timeout(1300);
    let global_state = global_state.with_state(state);
    let (state, global_state): (KeyboardReleaseState, _) = global_state.take_state();
    let state = state.with_timeout(1300);
    let global_state = global_state.with_state(state);

    let (state, global_state): (KeyboardReleaseState, _) = global_state.take_state();
    let (state, scheduled) = state.with_event(
        KeyboardSwitchEvent::new(1300, KeyboardSwitch("LeftCtrl"), ()),
        &mapping,
    );
    let global_state = global_state.with_state(state);
    println!("St: {:?}", global_state);
    println!("Sh: {:?}", scheduled);
    println!();

    panic!();
}
