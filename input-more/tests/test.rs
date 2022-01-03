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
            Mouse
        3:
            Move smth to library
        4:
            hooray
    */

    use core::fmt::Debug;
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

    pub type KeyboardPressMapping = Mapping<KeyboardSwitch, Switch, (), BasicAppEventBuilder>;
    pub type KeyboardReleaseMapping =
        Mapping<KeyboardSwitch, Switch, Option<TimedReleaseEventData>, BasicAppEventBuilder>;
    pub type KeyboardLongPressMapping =
        Mapping<KeyboardSwitch, Switch, TimedLongPressEventData, BasicAppEventBuilder>;
    pub type KeyboardClickExactMapping =
        Mapping<KeyboardSwitch, Switch, TimedClickExactEventData, BasicAppEventBuilder>;

    pub type KeyboardPressMappingCache =
        MappingCache<KeyboardSwitch, Switch, (), BasicAppEventBuilder>;
    pub type KeyboardReleaseMappingCache =
        MappingCache<KeyboardSwitch, Switch, Option<TimedReleaseEventData>, BasicAppEventBuilder>;
    pub type KeyboardLongPressMappingCache =
        MappingCache<KeyboardSwitch, Switch, TimedLongPressEventData, BasicAppEventBuilder>;
    pub type KeyboardClickExactMappingCache =
        MappingCache<KeyboardSwitch, Switch, TimedClickExactEventData, BasicAppEventBuilder>;

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
            Option<(MappingByTimed<'a, Switch, Self::EventBuilder>, Self::Coords)>,
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
            Vec<(MappingByTimed<'a, Switch, Self::EventBuilder>, Self::Coords)>,
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
            Option<(MappingByTimed<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        ) {
            let State {
                modifiers,
                timed_state,
                scheduler,
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
                return (Self::new(modifiers, timed_state, scheduler), None, None);
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
                return (Self::new(modifiers, timed_state, scheduler), None, None);
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

            println!("Ev1: {:?}", event);
            if let Some(mapping) = &mapping {
                println!("Ma1: {:?}", mapping.inner());
            }
            (
                Self::new(modifiers, timed_state, scheduler),
                next_scheduled,
                mapping.map(|mapping| (mapping, event.coords)),
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
            Option<(MappingByTimed<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        ) {
            let State {
                modifiers,
                timed_state,
                scheduler,
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
                return (Self::new(modifiers, timed_state, scheduler), None, None);
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
                return (Self::new(modifiers, timed_state, scheduler), None, None);
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

            /*let event = event.with_timed_data(timed_data);

            println!("Ev2: {:?}", event);
            if let Some(mapping) = &mapping {
                println!("Ma2: {:?}", mapping.inner());
            }*/

            (
                Self::new(modifiers, timed_state, scheduler),
                next_scheduled,
                mapping.map(|mapping| (mapping, event.coords)),
            )
        }
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
            Vec<(MappingByTimed<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        ) {
            // TODO: Better filtering

            const KEYBOARD_LONG_PRESS_DURATION: DurationMs = 1000;

            let mut timed_state = self.timed_state;
            let (scheduler, requests) = self
                .scheduler
                .take_scheduled(&(time - KEYBOARD_LONG_PRESS_DURATION));
            let mut bindings = Vec::new();
            for (_, requests) in requests {
                for (event, request) in requests {
                    let mapping = mapping.keyboard_long_press.filter_by_switch(&event.switch);
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
                        timed_state.with_long_press_event(event.switch, request);
                    timed_state = new_timed_state;

                    let timed_data = result.unwrap();
                    let timed_data = if let Some(timed_data) = timed_data {
                        timed_data
                    } else {
                        continue;
                    };

                    let mapping = mapping.filter_by_timed_data(&timed_data);

                    let event = event.with_timed_data(timed_data);

                    //println!("Ev3: {:?}", event);
                    if let Some(mapping) = mapping {
                        //println!("Ma3: {:?}", mapping.inner());
                        bindings.push((mapping, event.coords));
                    }
                }
            }
            (Self::new(self.modifiers, timed_state, scheduler), bindings)
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
            Vec<(MappingByTimed<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        ) {
            // TODO: Better filtering

            const KEYBOARD_CLICK_EXACT_DURATION: DurationMs = 300;

            let mut timed_state = self.timed_state;
            let (scheduler, requests) = self
                .scheduler
                .take_scheduled(&(time - KEYBOARD_CLICK_EXACT_DURATION));
            let mut bindings = Vec::new();
            for (_, requests) in requests {
                for (event, request) in requests {
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
                    }
                }
            }
            (Self::new(self.modifiers, timed_state, scheduler), bindings)
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
                event: BasicAppEventBuilder::Undo(10),
            },
            Binding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new()
                    .with_press_event(Switch::Keyboard(KeyboardSwitch("LeftShift")))
                    .0,
                timed_data: (),
                event: BasicAppEventBuilder::Undo(110),
            },
            Binding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new()
                    .with_press_event(Switch::Keyboard(KeyboardSwitch("LeftAlt")))
                    .0,
                timed_data: (),
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
                event: BasicAppEventBuilder::Undo(20),
            },
            Binding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new(),
                timed_data: Some(TimedReleaseEventData {
                    kind: TimedReleaseEventKind::Click,
                    num_possible_clicks: 1,
                }),
                event: BasicAppEventBuilder::Undo(30),
            },
            Binding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new(),
                timed_data: Some(TimedReleaseEventData {
                    kind: TimedReleaseEventKind::Click,
                    num_possible_clicks: 2,
                }),
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
        bindings: MappingByTimed<'a, Switch, Bu>,
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
