#[cfg(feature = "disabled")]
#[test]
fn test_chain() {
    use core::fmt::Debug;

    use input_core::*;
    use input_more::State;

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

    #[derive(Clone, Debug, Default)]
    pub struct Context<St, Ma> {
        state: St,
        mapping: Ma,
    }

    impl<St, Ma> Context<St, Ma> {
        pub fn new(state: St, mapping: Ma) -> Self {
            Self { state, mapping }
        }
    }

    impl Context<State<Modifieirs<Switch>, KeyboardSwitch, LongPressHandleRequest>, GlobalMapping> {
        pub fn with_event(self, event: KeyboardSwitchEvent) -> (Self, Option<TimestampMs>) {
            let (mapping, modifiers, timed_state, scheduler) = self.split();
            let filtered_mapping = self.mapping.to_ref_mapping();

            let (modifiers, result) = self
                .modifiers
                .with_press_event(Switch::Keyboard(event.switch));
            result.unwrap();

            let event = event.with_modifiers(modifiers.clone());
            let (timed_state, result) = self.timed_state.with_press_event(event.switch);
            let request = result.unwrap();

            let scheduler = scheduler.schedule(event.time, event.clone(), request);
            let next_scheduled = self.scheduler.next_scheduled().copied();

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

            (
                Self::new(mapping, modifiers, timed_state, scheduler),
                next_scheduled,
            )
        }

        pub fn with_timeout(self, time: TimestampMs) -> Self {
            // TODO: Add filtering
            const KEYBOARD_LONG_PRESS_DURATION: DurationMs = 1000;

            let (mapping, modifiers, timed_state, scheduler) = self.split();
            let mut timed_state = timed_state;
            let (scheduler, (_, requests)) =
                scheduler.take_scheduled(time - KEYBOARD_LONG_PRESS_DURATION);
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
            Self::new(mapping, modifiers, timed_state, scheduler)
        }
    }

    impl Context<State<KeyboardSwitch, ClickExactHandleRequest>, Mappings> {
        pub fn with_event(self, event: KeyboardSwitchEvent) -> (Self, Option<TimestampMs>) {
            let (mapping, modifiers, timed_state, scheduler) = self.split();
            let filtered_mapping = mapping.to_ref_mapping();

            let (modifiers, (_, result)) =
                modifiers.with_release_event(Switch::Keyboard(event.switch));
            result.unwrap();

            let event = event.with_modifiers(modifiers.clone());
            let (timed_state, result) = timed_state.with_release_event(event.switch);
            let data = result.unwrap();

            let (scheduler, event) = if let Some((timed_event_data, request)) = data {
                (
                    scheduler.schedule(event.time, event.clone(), request),
                    event.with_timed_data(Some(timed_event_data)),
                )
            } else {
                (scheduler, event.with_timed_data(None))
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

            (
                Self::new(mapping, modifiers, timed_state, scheduler),
                next_scheduled,
            )
        }

        pub fn with_timeout(self, time: TimestampMs) -> Self {
            // TODO: Add filtering
            const KEYBOARD_CLICK_EXACT_DURATION: DurationMs = 300;

            let (mapping, modifiers, timed_state, scheduler) = self.split();
            let mut timed_state = timed_state;
            let (scheduler, (_, requests)) =
                scheduler.take_scheduled(time - KEYBOARD_CLICK_EXACT_DURATION);
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
            Self::new(mapping, modifiers, timed_state, scheduler)
        }
    }

    let mut state = GlobalState::default();
    state.mappings.keyboard_release.0 = vec![BindingData {
        switch: KeyboardSwitch("LeftCtrl"),
        modifiers: Modifiers::new(),
        timed_data: Some(TimedReleaseEventData {
            kind: TimedReleaseEventKind::Click,
            num_possible_clicks: 2,
        }),
        event_builder: BasicAppEventBuilder::Undo(10),
    }];

    let state = state.with_event(RawEvent::KeyboardPress(SwitchEvent::new(
        1000,
        KeyboardSwitch("LeftCtrl"),
        (),
    )));
    println!("St: {:?}", state.keyboard_timed_state);
    println!();

    let state = state.with_event(RawEvent::KeyboardRelease(SwitchEvent::new(
        1100,
        KeyboardSwitch("LeftCtrl"),
        (),
    )));
    println!("St: {:?}", state.keyboard_timed_state);
    println!();

    let state = state.with_event(RawEvent::KeyboardPress(SwitchEvent::new(
        1200,
        KeyboardSwitch("LeftCtrl"),
        (),
    )));
    println!("St: {:?}", state.keyboard_timed_state);
    println!();

    let state = state.with_event(RawEvent::KeyboardRelease(SwitchEvent::new(
        1300,
        KeyboardSwitch("LeftCtrl"),
        (),
    )));
    println!("St: {:?}", state.keyboard_timed_state);
    println!();

    //

    panic!();
}
