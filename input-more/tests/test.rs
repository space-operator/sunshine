#[test]
fn test_chain() {
    /*
    TODO:
        1:
            Modifiers -> [AppEventBinding]
            filter event+data(like coords) by events in app
                from Modifiers -> [Event + Data]
                into Modifiers -> [Option<AppEvent>]
                into Modifiers -> [AppEvent]
            filter events by modifiers in input
                find longest Modifiers
                if different modifiers then drop
                if equal modifiers
            emit AppEvent
        2:
            We do not need to store anything except Modifiers in Events,
            because we copy events only for timed processing,
            but use data separated from events for mapping.
        3:
            PointerState
        4:
            State::with_press_event, State::with_release_event
        5:
            Mouse
        6:
            Move smth to library
        7:
            hooray
    */

    use core::fmt::Debug;
    use core::hash::Hash;
    use std::collections::HashMap;

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
        CreateNode(u32, &'static str),
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum BasicAppEventBuilder {
        Undo(u32),
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum PointerAppEventBuilder {
        Undo(u32),
        CreateNode(u32),
    }

    pub trait BuildAppEvent<Co> {
        fn build(&self, coords: &Co) -> Option<AppEvent>;
    }

    impl BuildAppEvent<()> for BasicAppEventBuilder {
        fn build(&self, _: &()) -> Option<AppEvent> {
            match self {
                Self::Undo(id) => Some(AppEvent::Undo(*id)),
            }
        }
    }

    impl BuildAppEvent<Coords> for PointerAppEventBuilder {
        fn build(&self, coords: &Coords) -> Option<AppEvent> {
            match self {
                Self::Undo(id) => Some(AppEvent::Undo(*id)),
                Self::CreateNode(id) => {
                    if (100..=200).contains(&coords.0) && (100..=200).contains(&coords.1) {
                        Some(AppEvent::CreateNode(*id, "first"))
                    } else if (300..=400).contains(&coords.0) && (300..=400).contains(&coords.1) {
                        Some(AppEvent::CreateNode(*id, "second"))
                    } else {
                        None
                    }
                }
            }
        }
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

    pub type KeyboardPressMapping =
        Mapping<KeyboardSwitch, Switch, (), Option<PointerChangeEventKind>, BasicAppEventBuilder>;
    pub type KeyboardReleaseMapping = Mapping<
        KeyboardSwitch,
        Switch,
        Option<TimedReleaseEventData>,
        Option<PointerChangeEventKind>,
        BasicAppEventBuilder,
    >;
    pub type KeyboardLongPressMapping =
        Mapping<KeyboardSwitch, Switch, TimedLongPressEventData, (), BasicAppEventBuilder>;
    pub type KeyboardClickExactMapping =
        Mapping<KeyboardSwitch, Switch, TimedClickExactEventData, (), BasicAppEventBuilder>;

    pub type KeyboardPressMappingCache = MappingCache<
        KeyboardSwitch,
        Switch,
        (),
        Option<PointerChangeEventKind>,
        BasicAppEventBuilder,
    >;
    pub type KeyboardReleaseMappingCache = MappingCache<
        KeyboardSwitch,
        Switch,
        Option<TimedReleaseEventData>,
        Option<PointerChangeEventKind>,
        BasicAppEventBuilder,
    >;
    pub type KeyboardLongPressMappingCache =
        MappingCache<KeyboardSwitch, Switch, TimedLongPressEventData, (), BasicAppEventBuilder>;
    pub type KeyboardClickExactMappingCache =
        MappingCache<KeyboardSwitch, Switch, TimedClickExactEventData, (), BasicAppEventBuilder>;

    /*pub type GlobalMapping = input_more::GlobalMapping<
        KeyboardPressMapping,
        KeyboardReleaseMapping,
        KeyboardLongPressMapping,
        KeyboardClickExactMapping,
    >;*/

    pub type KeyboardSwitchEvent = SwitchEvent<TimestampMs, KeyboardSwitch, (), (), ()>;
    pub type MouseSwitchEvent = SwitchEvent<TimestampMs, MouseSwitch, Coords, (), ()>;

    pub type Modifiers = input_core::Modifiers<Switch>;
    pub type KeyboardTimedState = TimedState<KeyboardSwitch>;
    pub type MouseTimedState = TimedState<MouseSwitch>;

    pub type CustomState<Ts, Sh, Ps> = State<Modifiers, Ts, Sh, Ps>;
    pub type CustomScheduler<Sw, Re> =
        SchedulerState<TimestampMs, SwitchEvent<TimestampMs, Sw, (), Modifiers, ()>, Re>;
    pub type KeyboardLongPressScheduler = CustomScheduler<KeyboardSwitch, LongPressHandleRequest>;
    pub type KeyboardClickExactScheduler = CustomScheduler<KeyboardSwitch, ClickExactHandleRequest>;
    pub type MouseLongPressScheduler = CustomScheduler<MouseSwitch, LongPressHandleRequest>;
    pub type MouseClickExactScheduler = CustomScheduler<MouseSwitch, ClickExactHandleRequest>;
    pub type KeyboardPointerState = PointerState<KeyboardSwitch, ()>;
    pub type MousePointerState = PointerState<MouseSwitch, Coords>;

    pub type KeyboardPressState =
        CustomState<KeyboardTimedState, KeyboardLongPressScheduler, KeyboardPointerState>;
    pub type KeyboardReleaseState =
        CustomState<KeyboardTimedState, KeyboardClickExactScheduler, KeyboardPointerState>;
    pub type KeyboardLongPressState = CustomState<MouseTimedState, (), ()>;
    pub type KeyboardClickExactState = CustomState<MouseTimedState, (), ()>;

    pub type GlobalState = input_more::GlobalState<
        Modifiers,
        KeyboardTimedState,
        MouseTimedState,
        KeyboardLongPressScheduler,
        KeyboardClickExactScheduler,
        MouseLongPressScheduler,
        MouseClickExactScheduler,
        KeyboardPointerState,
        MousePointerState,
    >;

    pub trait WithEvent<Ev>: Sized {
        type EventBuilder;
        type Coords;

        fn with_event<'a>(
            self,
            event: Ev,
            mapping: &'a GlobalMappingCache,
        ) -> (
            Self,
            Option<TimestampMs>,
            Option<(Bindings<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        );
    }

    pub trait WithTimeout: Sized {
        type EventBuilder;
        type Coords;

        fn with_timeout<'a>(
            self,
            time: TimestampMs,
            mapping: &'a GlobalMappingCache,
        ) -> (
            Self,
            Vec<(Bindings<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        );
    }

    impl WithEvent<KeyboardSwitchEvent> for KeyboardPressState {
        type EventBuilder = BasicAppEventBuilder;
        type Coords = ();

        fn with_event<'a>(
            self,
            event: KeyboardSwitchEvent,
            mapping: &'a GlobalMappingCache,
        ) -> (
            Self,
            Option<TimestampMs>,
            Option<(Bindings<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        ) {
            let State {
                modifiers,
                timed_state,
                scheduler,
                pointer_state,
            } = self;

            let keyboard_press_mapping = mapping.keyboard_press.filter_by_switch(&event.switch);
            let keyboard_release_mapping = mapping.keyboard_release.filter_by_switch(&event.switch);
            let keyboard_long_press_mapping =
                mapping.keyboard_long_press.filter_by_switch(&event.switch);
            let keyboard_click_exact_mapping =
                mapping.keyboard_click_exact.filter_by_switch(&event.switch);

            let is_used_as_modifier = mapping
                .modifiers
                .switches()
                .contains(&Switch::Keyboard(event.switch));

            if keyboard_press_mapping.is_none()
                && keyboard_release_mapping.is_none()
                && keyboard_long_press_mapping.is_none()
                && keyboard_click_exact_mapping.is_none()
                && !is_used_as_modifier
            {
                return (
                    Self::new(modifiers, timed_state, scheduler, pointer_state),
                    None,
                    None,
                );
            }

            let modifiers = if is_used_as_modifier {
                let (modifiers, result) =
                    modifiers.with_press_event(Switch::Keyboard(event.switch));
                result.unwrap();
                modifiers
            } else {
                modifiers
            };
            let event = event.with_modifiers(modifiers.clone());

            let keyboard_press_mapping = keyboard_press_mapping
                .and_then(|mapping| mapping.filter_by_modifiers(&event.modifiers));
            let keyboard_release_mapping = keyboard_release_mapping
                .and_then(|mapping| mapping.filter_by_modifiers(&event.modifiers));
            let keyboard_long_press_mapping = keyboard_long_press_mapping
                .and_then(|mapping| mapping.filter_by_modifiers(&event.modifiers));
            let keyboard_click_exact_mapping = keyboard_click_exact_mapping
                .and_then(|mapping| mapping.filter_by_modifiers(&event.modifiers));

            if keyboard_press_mapping.is_none()
                && keyboard_release_mapping.is_none()
                && keyboard_long_press_mapping.is_none()
                && keyboard_click_exact_mapping.is_none()
            {
                return (
                    Self::new(modifiers, timed_state, scheduler, pointer_state),
                    None,
                    None,
                );
            }

            let (timed_state, result) = timed_state.with_press_event(event.switch);
            let request = result.unwrap();

            /*let (scheduler, next_scheduled) = if keyboard_release_mapping.is_some()
                && keyboard_long_press_mapping.is_some()
                && keyboard_click_exact_mapping.is_some()
            {*/
            let scheduler = scheduler.schedule(event.time, event.clone(), request);
            let next_scheduled = scheduler.next_scheduled().copied();
            //(scheduler, next_scheduled)
            /*} else {
                (scheduler, None)
            };*/

            let mapping =
                keyboard_press_mapping.and_then(|mapping| mapping.filter_by_timed_data(&()));

            let mapping = if let Some(mapping) = mapping {
                mapping
            } else {
                return (
                    Self::new(modifiers, timed_state, scheduler, pointer_state),
                    next_scheduled,
                    None,
                );
            };

            let (pointer_state, pointer_data) = pointer_state.with_change_event(event.switch, ());
            let mapping = mapping.filter_by_pointer_data(&pointer_data);
            let mapping = if let Some(mapping) = mapping {
                mapping
            } else {
                return (
                    Self::new(modifiers, timed_state, scheduler, pointer_state),
                    next_scheduled,
                    None,
                );
            };

            (
                Self::new(modifiers, timed_state, scheduler, pointer_state),
                next_scheduled,
                Some((mapping, event.coords)),
            )
        }
    }

    impl WithEvent<KeyboardSwitchEvent> for KeyboardReleaseState {
        type EventBuilder = BasicAppEventBuilder;
        type Coords = ();

        fn with_event<'a>(
            self,
            event: KeyboardSwitchEvent,
            mapping: &'a GlobalMappingCache,
        ) -> (
            Self,
            Option<TimestampMs>,
            Option<(Bindings<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        ) {
            let State {
                modifiers,
                timed_state,
                scheduler,
                pointer_state,
            } = self;

            let keyboard_press_mapping = mapping.keyboard_press.filter_by_switch(&event.switch);
            let keyboard_release_mapping = mapping.keyboard_release.filter_by_switch(&event.switch);
            let keyboard_long_press_mapping =
                mapping.keyboard_long_press.filter_by_switch(&event.switch);
            let keyboard_click_exact_mapping =
                mapping.keyboard_click_exact.filter_by_switch(&event.switch);

            let is_used_as_modifier = mapping
                .modifiers
                .switches()
                .contains(&Switch::Keyboard(event.switch));

            if keyboard_press_mapping.is_none()
                && keyboard_release_mapping.is_none()
                && keyboard_long_press_mapping.is_none()
                && keyboard_click_exact_mapping.is_none()
                && !is_used_as_modifier
            {
                return (
                    Self::new(modifiers, timed_state, scheduler, pointer_state),
                    None,
                    None,
                );
            }

            let modifiers = if is_used_as_modifier {
                let (modifiers, result) =
                    modifiers.with_release_event(&Switch::Keyboard(event.switch));
                result.unwrap();
                modifiers
            } else {
                modifiers
            };

            let event = event.with_modifiers(modifiers.clone());

            let keyboard_press_mapping = keyboard_press_mapping
                .and_then(|mapping| mapping.filter_by_modifiers(&event.modifiers));
            let keyboard_release_mapping = keyboard_release_mapping
                .and_then(|mapping| mapping.filter_by_modifiers(&event.modifiers));
            let keyboard_long_press_mapping = keyboard_long_press_mapping
                .and_then(|mapping| mapping.filter_by_modifiers(&event.modifiers));
            let keyboard_click_exact_mapping = keyboard_click_exact_mapping
                .and_then(|mapping| mapping.filter_by_modifiers(&event.modifiers));

            if keyboard_press_mapping.is_none()
                && keyboard_release_mapping.is_none()
                && keyboard_long_press_mapping.is_none()
                && keyboard_click_exact_mapping.is_none()
            {
                return (
                    Self::new(modifiers, timed_state, scheduler, pointer_state),
                    None,
                    None,
                );
            }

            let (timed_state, result) = timed_state.with_release_event(event.switch);
            let data = result.unwrap();

            let (timed_data, scheduler, next_scheduled) = if let Some((timed_data, request)) = data
            {
                let scheduler = scheduler.schedule(event.time, event.clone(), request);
                let next_scheduled = scheduler.next_scheduled().copied();
                (Some(timed_data), scheduler, next_scheduled)
            } else {
                (None, scheduler, None)
            };

            let mapping = keyboard_release_mapping
                .and_then(|mapping| mapping.filter_by_timed_data(&timed_data));

            let mapping = if let Some(mapping) = mapping {
                mapping
            } else {
                return (
                    Self::new(modifiers, timed_state, scheduler, pointer_state),
                    next_scheduled,
                    None,
                );
            };

            let (pointer_state, pointer_data) = pointer_state.with_change_event(event.switch, ());
            let mapping = mapping.filter_by_pointer_data(&pointer_data);
            let mapping = if let Some(mapping) = mapping {
                mapping
            } else {
                return (
                    Self::new(modifiers, timed_state, scheduler, pointer_state),
                    next_scheduled,
                    None,
                );
            };

            (
                Self::new(modifiers, timed_state, scheduler, pointer_state),
                next_scheduled,
                Some((mapping, event.coords)),
            )
        }
    }

    fn with_timeout_event<'a, Rq, Td>(
        mapping: &'a MappingCache<
            KeyboardSwitch,
            Switch,
            TimedEventData<Td>,
            (),
            BasicAppEventBuilder,
        >,
        timed_state: TimedState<KeyboardSwitch>,
        event: SwitchEvent<TimestampMs, KeyboardSwitch, (), Modifiers, ()>,
        request: Rq,
        timed_processing: impl FnOnce(
            TimedState<KeyboardSwitch>,
            KeyboardSwitch,
            Rq,
        )
            -> (TimedState<KeyboardSwitch>, Option<TimedEventData<Td>>),
    ) -> (
        TimedState<KeyboardSwitch>,
        Option<(Bindings<'a, Switch, BasicAppEventBuilder>, ())>,
    )
    where
        Td: 'a + Eq + Hash,
    {
        let mapping =
            unwrap_or_return!(mapping.filter_by_switch(&event.switch), (timed_state, None));

        let mapping = unwrap_or_return!(
            mapping.filter_by_modifiers(&event.modifiers),
            (timed_state, None)
        );
        //let (new_timed_state, result) = timed_state.with_long_press_event(event.switch, request);
        let (timed_state, result) = timed_processing(timed_state, event.switch, request);

        let timed_data = unwrap_or_return!(result, (timed_state, None));
        let mapping = unwrap_or_return!(
            mapping.filter_by_timed_data(&timed_data),
            (timed_state, None)
        );
        let bindings = unwrap_or_return!(mapping.filter_by_pointer_data(&()), (timed_state, None));
        let event = event.with_timed_data(timed_data);

        (timed_state, Some((bindings, event.coords)))
    }

    impl WithTimeout for KeyboardPressState {
        type EventBuilder = BasicAppEventBuilder;
        type Coords = ();

        fn with_timeout<'a>(
            self,
            time: TimestampMs,
            mapping: &'a GlobalMappingCache,
        ) -> (
            Self,
            Vec<(Bindings<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        ) {
            const KEYBOARD_LONG_PRESS_DURATION: DurationMs = 1000;

            let mut timed_state = self.timed_state;
            let (scheduler, requests) = self
                .scheduler
                .take_scheduled(&(time - KEYBOARD_LONG_PRESS_DURATION));
            let pointer_state = self.pointer_state;
            let mut delayed_bindings = Vec::new();
            for (_, requests) in requests {
                for (event, request) in requests {
                    let (new_timed_state, result) = with_timeout_event(
                        &mapping.keyboard_long_press,
                        timed_state,
                        event,
                        request,
                        |timed_state, switch, request| {
                            let (timed_state, result) =
                                timed_state.with_long_press_event(switch, request);
                            (timed_state, result.unwrap())
                        },
                    );
                    timed_state = new_timed_state;
                    if let Some((bindings, coords)) = result {
                        delayed_bindings.push((bindings, coords));
                    }
                }
            }
            (
                Self::new(self.modifiers, timed_state, scheduler, pointer_state),
                delayed_bindings,
            )
        }
    }

    impl WithTimeout for KeyboardReleaseState {
        type EventBuilder = BasicAppEventBuilder;
        type Coords = ();

        fn with_timeout<'a>(
            self,
            time: TimestampMs,
            mapping: &'a GlobalMappingCache,
        ) -> (
            Self,
            Vec<(Bindings<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        ) {
            // TODO: Better filtering

            const KEYBOARD_CLICK_EXACT_DURATION: DurationMs = 300;

            let mut timed_state = self.timed_state;
            let (scheduler, requests) = self
                .scheduler
                .take_scheduled(&(time - KEYBOARD_CLICK_EXACT_DURATION));
            let pointer_state = self.pointer_state;
            let mut delayed_bindings = Vec::new();
            for (_, requests) in requests {
                for (event, request) in requests {
                    let (new_timed_state, result) = with_timeout_event(
                        &mapping.keyboard_click_exact,
                        timed_state,
                        event,
                        request,
                        |timed_state, switch, request| {
                            let (timed_state, result) =
                                timed_state.with_click_exact_event(switch, request);
                            (timed_state, result.unwrap())
                        },
                    );
                    timed_state = new_timed_state;
                    if let Some((bindings, coords)) = result {
                        delayed_bindings.push((bindings, coords));
                    }
                    /*
                    let mapping = mapping.keyboard_click_exact.filter_by_switch(&event.switch);
                    let mapping = if let Some(mapping) = mapping {
                        mapping
                    } else {
                        continue;
                    };

                    let mapping = mapping.filter_by_modifiers(&event.modifiers);
                    let mapping = if let Some(mapping) = mapping {
                        mapping
                    } else {
                        continue;
                    };

                    let (new_timed_state, result) =
                        timed_state.with_click_exact_event(event.switch, request);
                    timed_state = new_timed_state;

                    let timed_data = result.unwrap();
                    let timed_data = if let Some(timed_data) = timed_data {
                        timed_data
                    } else {
                        continue;
                    };

                    let mapping = mapping.filter_by_timed_data(&timed_data);

                    let event = event.with_timed_data(timed_data);

                    //println!("Ev4: {:?}", event);
                    if let Some(mapping) = mapping {
                        //println!("Ma4: {:?}", mapping.inner());
                        bindings.push((mapping, event.coords));
                    }*/
                }
            }
            (
                Self::new(self.modifiers, timed_state, scheduler, pointer_state),
                delayed_bindings,
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct GlobalMapping {
        keyboard_press: KeyboardPressMapping,
        keyboard_release: KeyboardReleaseMapping,
        keyboard_long_press: KeyboardLongPressMapping,
        keyboard_click_exact: KeyboardClickExactMapping,
    }

    #[derive(Clone, Debug)]
    pub struct GlobalMappingCache<'a> {
        keyboard_press: KeyboardPressMappingCache,
        keyboard_release: KeyboardReleaseMappingCache,
        keyboard_long_press: KeyboardLongPressMappingCache,
        keyboard_click_exact: KeyboardClickExactMappingCache,
        modifiers: ModifiersCache<&'a Switch>,
    }

    impl<'a> GlobalMappingCache<'a> {
        fn contains(&self, switch: &Switch) -> bool {
            self.modifiers.switches().contains(switch)
        }
    }

    let keyboard_press_mapping = Mapping::new(
        [
            Binding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new(),
                timed_data: (),
                pointer_data: None,
                event: BasicAppEventBuilder::Undo(10),
            },
            Binding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new()
                    .with_press_event(Switch::Keyboard(KeyboardSwitch("LeftShift")))
                    .0,
                timed_data: (),
                pointer_data: None,
                event: BasicAppEventBuilder::Undo(110),
            },
            Binding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new()
                    .with_press_event(Switch::Keyboard(KeyboardSwitch("LeftAlt")))
                    .0,
                timed_data: (),
                pointer_data: None,
                event: BasicAppEventBuilder::Undo(120),
            },
        ]
        .into_iter()
        .collect(),
    );

    let keyboard_release_mapping = Mapping::new(
        [
            Binding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: None,
                event: BasicAppEventBuilder::Undo(20),
            },
            Binding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new(),
                timed_data: Some(TimedReleaseEventData {
                    kind: TimedReleaseEventKind::Click,
                    num_possible_clicks: 1,
                }),
                pointer_data: None,
                event: BasicAppEventBuilder::Undo(30),
            },
            Binding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new(),
                timed_data: Some(TimedReleaseEventData {
                    kind: TimedReleaseEventKind::Click,
                    num_possible_clicks: 2,
                }),
                pointer_data: None,
                event: BasicAppEventBuilder::Undo(40),
            },
        ]
        .into_iter()
        .collect(),
    );

    let mapping = GlobalMapping {
        keyboard_press: keyboard_press_mapping,
        keyboard_release: keyboard_release_mapping,
        keyboard_long_press: Mapping::default(),
        keyboard_click_exact: Mapping::default(),
    };

    let mapping_cache = GlobalMappingCache {
        keyboard_press: MappingCache::from(mapping.keyboard_press.clone()),
        keyboard_release: MappingCache::from(mapping.keyboard_release.clone()),
        keyboard_long_press: MappingCache::from(mapping.keyboard_long_press.clone()),
        keyboard_click_exact: MappingCache::from(mapping.keyboard_click_exact.clone()),
        modifiers: [
            ModifiersCache::from(&mapping.keyboard_press).switches(),
            ModifiersCache::from(&mapping.keyboard_release).switches(),
            ModifiersCache::from(&mapping.keyboard_long_press).switches(),
            ModifiersCache::from(&mapping.keyboard_click_exact).switches(),
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect(),
    };

    let mut global_state = GlobalState::default();

    enum RawEvent {
        KeyboardPress(KeyboardSwitchEvent),
        KeyboardRelease(KeyboardSwitchEvent),
    }

    impl RawEvent {
        fn time(&self) -> TimestampMs {
            match self {
                RawEvent::KeyboardPress(event) => event.time,
                RawEvent::KeyboardRelease(event) => event.time,
            }
        }
    }

    fn build_bindings<'a, Bu, Co>(
        bindings: Bindings<'a, Switch, Bu>,
        coords: &Co,
    ) -> Option<HashMap<&'a Modifiers, Vec<AppEvent>>>
    where
        Bu: BuildAppEvent<Co>,
    {
        let bindings: HashMap<_, _> = bindings
            .into_inner()
            .into_iter()
            .filter_map(|(modifiers, events)| {
                let events: Vec<_> = events
                    .into_iter()
                    .filter_map(|binding| binding.build(coords))
                    .collect();
                if events.is_empty() {
                    None
                } else {
                    Some((modifiers, events))
                }
            })
            .collect();
        if bindings.is_empty() {
            None
        } else {
            Some(bindings)
        }
    }

    trait GlobalStateExt: Sized {
        fn with_timeout<'a>(
            self,
            time: TimestampMs,
            mapping: &'a GlobalMappingCache,
        ) -> (Self, Vec<HashMap<&'a Modifiers, Vec<AppEvent>>>);

        fn with_event<'a>(
            self,
            event: RawEvent,
            mapping: &'a GlobalMappingCache,
        ) -> (
            Self,
            Option<DurationMs>,
            Vec<HashMap<&'a Modifiers, Vec<AppEvent>>>,
            Option<HashMap<&'a Modifiers, Vec<AppEvent>>>,
        );
    }

    impl GlobalStateExt for GlobalState {
        fn with_timeout<'a>(
            self,
            time: TimestampMs,
            mapping: &'a GlobalMappingCache,
        ) -> (Self, Vec<HashMap<&'a Modifiers, Vec<AppEvent>>>) {
            let global_state = self;

            let (state, global_state): (KeyboardPressState, _) = global_state.take_state();
            let (state, press_bindings) = state.with_timeout(time, mapping);
            let global_state = global_state.with_state(state);
            let press_bindings = press_bindings
                .into_iter()
                .filter_map(|(bindings, coords)| build_bindings(bindings, &coords));

            let (state, global_state): (KeyboardReleaseState, _) = global_state.take_state();
            let (state, release_bindings) = state.with_timeout(time, mapping);
            let global_state = global_state.with_state(state);
            let release_bindings = release_bindings
                .into_iter()
                .filter_map(|(bindings, coords)| build_bindings(bindings, &coords));

            (
                global_state,
                press_bindings.chain(release_bindings).collect(),
            )
        }

        fn with_event<'a>(
            self,
            event: RawEvent,
            mapping: &'a GlobalMappingCache,
        ) -> (
            Self,
            Option<DurationMs>,
            Vec<HashMap<&'a Modifiers, Vec<AppEvent>>>,
            Option<HashMap<&'a Modifiers, Vec<AppEvent>>>,
        ) {
            let (global_state, delayed_bindings) = self.with_timeout(event.time(), &mapping);
            let (global_state, scheduled, bindings) = match event {
                RawEvent::KeyboardPress(event) => {
                    let (state, global_state): (KeyboardPressState, _) = global_state.take_state();
                    let (state, scheduled, bindings) = state.with_event(event, &mapping);
                    let bindings =
                        bindings.and_then(|(bindings, coords)| build_bindings(bindings, &coords));
                    (global_state.with_state(state), scheduled, bindings)
                }
                RawEvent::KeyboardRelease(event) => {
                    let (state, global_state): (KeyboardReleaseState, _) =
                        global_state.take_state();
                    let (state, scheduled, bindings) = state.with_event(event, &mapping);
                    let bindings =
                        bindings.and_then(|(bindings, coords)| build_bindings(bindings, &coords));
                    (global_state.with_state(state), scheduled, bindings)
                }
            };
            (global_state, scheduled, delayed_bindings, bindings)
        }
    }

    pub fn filter_events_with_longest_modifiers(
        events: HashMap<&Modifiers, Vec<AppEvent>>,
    ) -> Vec<AppEvent> {
        let events: Vec<_> = events.into_iter().collect();

        let events_mask: Vec<_> = events
            .iter()
            .map(|(modifiers, _)| {
                events.iter().all(|(other_modifiers, _)| {
                    modifiers.switches().is_superset(other_modifiers.switches())
                })
            })
            .collect();

        events
            .into_iter()
            .enumerate()
            .filter_map(|(j, event)| if events_mask[j] { Some(event) } else { None })
            .flat_map(|(_, events)| events)
            .collect()
    }

    let events = [
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            1000,
            KeyboardSwitch("LeftShift"),
            (),
        )),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            1100,
            KeyboardSwitch("LeftAlt"),
            (),
        )),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            2000,
            KeyboardSwitch("LeftCtrl"),
            (),
        )),
        RawEvent::KeyboardRelease(KeyboardSwitchEvent::new(
            2100,
            KeyboardSwitch("LeftCtrl"),
            (),
        )),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            2200,
            KeyboardSwitch("LeftCtrl"),
            (),
        )),
        RawEvent::KeyboardRelease(KeyboardSwitchEvent::new(
            2300,
            KeyboardSwitch("LeftCtrl"),
            (),
        )),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            3000,
            KeyboardSwitch("LeftShift"),
            (),
        )),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            3100,
            KeyboardSwitch("LeftAlt"),
            (),
        )),
    ];

    for event in events {
        let (new_global_state, scheduled, delayed_bindings, bindings) =
            global_state.with_event(event, &mapping_cache);
        global_state = new_global_state;
        println!("St: {:?}", global_state);
        println!("Sh: {:?}", scheduled);
        for bindings in delayed_bindings {
            println!("Bi: {:?}", bindings);
            let events = filter_events_with_longest_modifiers(bindings);
            println!("Ev: {:?}", events);
        }
        if let Some(bindings) = bindings {
            println!("Bi: {:?}", bindings);
            let events = filter_events_with_longest_modifiers(bindings);
            println!("Ev: {:?}", events);
        }
        println!();
    }

    panic!();
}
