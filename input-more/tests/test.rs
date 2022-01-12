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
        >   Remain coords only in PointerMove event, but not in switchers
        6:
            Triggers and PointerMove
        5:
            Mouse
        6:
            Move smth to library
        7:
            hooray
    */

    /*
        switch without coords
        mousemove without switch

        switch
        trigger
        coords

        state
            co
        touch with coords
        touchmove with coords
        switch without coords
        untouch without coords
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

    impl From<KeyboardSwitch> for Switch {
        fn from(switch: KeyboardSwitch) -> Self {
            Self::Keyboard(switch)
        }
    }

    impl From<MouseSwitch> for Switch {
        fn from(switch: MouseSwitch) -> Self {
            Self::Mouse(switch)
        }
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

    pub type KeyboardMapping = DeviceMapping<KeyboardSwitch, (), Switch, BasicAppEventBuilder>;
    pub type MouseMapping = DeviceMapping<MouseSwitch, (), Switch, PointerAppEventBuilder>;

    pub type KeyboardSwitchEvent = SwitchEvent<TimestampMs, KeyboardSwitch>;
    pub type MouseSwitchEvent = SwitchEvent<TimestampMs, MouseSwitch>;

    pub type Modifiers = input_core::Modifiers<Switch>;
    pub type KeyboardTimedState = TimedState<KeyboardSwitch>;
    pub type MouseTimedState = TimedState<MouseSwitch>;

    pub type KeyboardCoordsState = CoordsState<()>;
    pub type MouseCoordsState = CoordsState<Coords>;

    pub type CustomState<Ts, Cs, ShLo, ShCl, Ps> = DeviceState<Modifiers, Cs, Ts, ShLo, ShCl, Ps>;
    pub type CustomScheduler<Sw, Re, Co> = DeviceSchedulerState<TimestampMs, Sw, Modifiers, Co, Re>;

    pub type KeyboardLongPressScheduler =
        CustomScheduler<KeyboardSwitch, LongPressHandleRequest, ()>;
    pub type KeyboardClickExactScheduler =
        CustomScheduler<KeyboardSwitch, ClickExactHandleRequest, ()>;
    pub type MouseLongPressScheduler = CustomScheduler<MouseSwitch, LongPressHandleRequest, Coords>;
    pub type MouseClickExactScheduler =
        CustomScheduler<MouseSwitch, ClickExactHandleRequest, Coords>;
    pub type KeyboardPointerState = PointerState<KeyboardSwitch, ()>;
    pub type MousePointerState = PointerState<MouseSwitch, Coords>;

    pub type KeyboardState = CustomState<
        KeyboardTimedState,
        KeyboardCoordsState,
        KeyboardLongPressScheduler,
        KeyboardClickExactScheduler,
        KeyboardPointerState,
    >;
    pub type KeyboardDelayedState = CustomState<MouseTimedState, KeyboardCoordsState, (), (), ()>;

    pub type MousePressState = CustomState<
        MouseTimedState,
        MouseCoordsState,
        MouseLongPressScheduler,
        MouseClickExactScheduler,
        MousePointerState,
    >;
    pub type MouseDelayedState = CustomState<MouseTimedState, MouseCoordsState, (), (), ()>;

    pub type GlobalState = input_more::GlobalState<
        Modifiers,
        KeyboardCoordsState,
        MouseCoordsState,
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
            Option<(SwitchBindings<'a, Switch, Self::EventBuilder>, Self::Coords)>,
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
            Vec<(SwitchBindings<'a, Switch, Self::EventBuilder>, Self::Coords)>,
        );
    }

    /*
        press
            nothing to filter mapping
            if mapping for release, mouse-trigger
            call with_press_event
                possibly dragging later
        trigger
            filter mapping
            with_move_event
                DragStart
                DragMove
        release
            filter mapping
            with_release_event
                DragEnd
    */

    /*#[derive(Clone, Debug)]
    pub struct GlobalMapping {
        keyboard: KeyboardMapping,
    }*/

    #[derive(Clone, Debug)]
    pub struct GlobalMappingCache {
        keyboard: DeviceMappingCache<KeyboardSwitch, (), Switch, BasicAppEventBuilder>,
        mouse: DeviceMappingCache<MouseSwitch, (), Switch, PointerAppEventBuilder>,
        modifiers: MappingModifiersCache<Switch>,
    }

    impl GlobalMappingCache {
        fn contains(&self, switch: &Switch) -> bool {
            self.modifiers.switches().contains(switch)
        }
    }

    let keyboard_mapping = KeyboardMapping::new(
        [
            Binding::Press(SwitchBinding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new(),
                timed_data: (),
                pointer_data: (),
                event: BasicAppEventBuilder::Undo(10),
            }),
            Binding::Press(SwitchBinding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new()
                    .with_press_event(Switch::Keyboard(KeyboardSwitch("LeftShift")))
                    .0,
                timed_data: (),
                pointer_data: (),
                event: BasicAppEventBuilder::Undo(110),
            }),
            Binding::Press(SwitchBinding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new()
                    .with_press_event(Switch::Keyboard(KeyboardSwitch("LeftAlt")))
                    .0,
                timed_data: (),
                pointer_data: (),
                event: BasicAppEventBuilder::Undo(120),
            }),
            Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: None,
                event: BasicAppEventBuilder::Undo(20),
            }),
            Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new(),
                timed_data: Some(TimedReleaseEventData {
                    kind: TimedReleaseEventKind::Click,
                    num_possible_clicks: 1,
                }),
                pointer_data: None,
                event: BasicAppEventBuilder::Undo(30),
            }),
            Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("LeftCtrl"),
                modifiers: Modifiers::new(),
                timed_data: Some(TimedReleaseEventData {
                    kind: TimedReleaseEventKind::Click,
                    num_possible_clicks: 2,
                }),
                pointer_data: None,
                event: BasicAppEventBuilder::Undo(40),
            }),
        ]
        .into_iter()
        .collect(),
    );
    let mouse_mapping = MouseMapping::new(
        [Binding::Press(SwitchBinding {
            switch: MouseSwitch("LeftMouseButton"),
            modifiers: Modifiers::new(),
            timed_data: (),
            pointer_data: (),
            event: PointerAppEventBuilder::CreateNode(10),
        })]
        .into_iter()
        .collect(),
    );

    /*let mapping = GlobalMapping {
        keyboard: mapping,
    };*/

    let mapping_cache = GlobalMappingCache {
        keyboard: DeviceMappingCache::from_bindings(keyboard_mapping.bindings()),
        mouse: DeviceMappingCache::from_bindings(mouse_mapping.bindings()),
        modifiers: MappingModifiersCache::from_bindings(
            keyboard_mapping.bindings(), /* + mouse_mapping */
        ),
    };

    let mut global_state = GlobalState::new(
        Modifiers::default(),
        KeyboardCoordsState::with_coords(()),
        MouseCoordsState::with_coords((0, 0)),
        KeyboardTimedState::default(),
        MouseTimedState::default(),
        KeyboardLongPressScheduler::default(),
        KeyboardClickExactScheduler::default(),
        MouseLongPressScheduler::default(),
        MouseClickExactScheduler::default(),
        KeyboardPointerState::default(),
        MousePointerState::default(),
    );

    #[derive(Clone, Debug)]
    enum RawEvent {
        KeyboardPress(KeyboardSwitchEvent),
        KeyboardRelease(KeyboardSwitchEvent),
        MousePress(MouseSwitchEvent),
        MouseRelease(MouseSwitchEvent),
    }

    impl RawEvent {
        fn time(&self) -> TimestampMs {
            match self {
                RawEvent::KeyboardPress(event) => event.time,
                RawEvent::KeyboardRelease(event) => event.time,
                RawEvent::MousePress(event) => event.time,
                RawEvent::MouseRelease(event) => event.time,
            }
        }
    }

    fn build_bindings<'a, Bu, Co>(
        bindings: SwitchBindings<'a, Switch, Bu>,
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
            let (state, press_bindings) = state.with_press_timeout(time, &mapping.keyboard);
            let global_state = global_state.with_state(state);
            let press_bindings = press_bindings
                .into_iter()
                .filter_map(|(bindings, coords)| build_bindings(bindings, &coords));

            let (state, global_state): (KeyboardReleaseState, _) = global_state.take_state();
            let (state, release_bindings) = state.with_release_timeout(time, &mapping.keyboard);
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
                    let (state, scheduled, bindings) =
                        state.with_press_event(event, &mapping.keyboard, &mapping.modifiers);
                    let bindings =
                        bindings.and_then(|(bindings, coords)| build_bindings(bindings, &coords));
                    (global_state.with_state(state), scheduled, bindings)
                }
                RawEvent::KeyboardRelease(event) => {
                    let (state, global_state): (KeyboardReleaseState, _) =
                        global_state.take_state();
                    let (state, scheduled, bindings) =
                        state.with_release_event(event, &mapping.keyboard, &mapping.modifiers);
                    let bindings =
                        bindings.and_then(|(bindings, coords)| build_bindings(bindings, &coords));
                    (global_state.with_state(state), scheduled, bindings)
                }
                RawEvent::MousePress(event) => {
                    let (state, global_state): (MousePressState, _) = global_state.take_state();
                    let (state, scheduled, bindings) =
                        state.with_press_event(event, &mapping.mouse, &mapping.modifiers);
                    let bindings =
                        bindings.and_then(|(bindings, coords)| build_bindings(bindings, &coords));
                    (global_state.with_state(state), scheduled, bindings)
                }

                RawEvent::MouseRelease(event) => {
                    let (state, global_state): (MouseReleaseState, _) = global_state.take_state();
                    let (state, scheduled, bindings) =
                        state.with_release_event(event, &mapping.mouse, &mapping.modifiers);
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
            //(),
        )),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            1100,
            KeyboardSwitch("LeftAlt"),
            //(),
        )),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            2000,
            KeyboardSwitch("LeftCtrl"),
            //(),
        )),
        RawEvent::KeyboardRelease(KeyboardSwitchEvent::new(
            2100,
            KeyboardSwitch("LeftCtrl"),
            //(),
        )),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            2200,
            KeyboardSwitch("LeftCtrl"),
            //(),
        )),
        RawEvent::KeyboardRelease(KeyboardSwitchEvent::new(
            2300,
            KeyboardSwitch("LeftCtrl"),
            //(),
        )),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            3000,
            KeyboardSwitch("LeftShift"),
            //(),
        )),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
            3100,
            KeyboardSwitch("LeftAlt"),
            //(),
        )),
        RawEvent::MousePress(MouseSwitchEvent::new(
            4000,
            MouseSwitch("LeftMouseButton"),
            //(150, 150),
        )),
        RawEvent::MouseRelease(MouseSwitchEvent::new(
            4100,
            MouseSwitch("LeftMouseButton"),
            //(150, 150),
        )),
        RawEvent::MousePress(MouseSwitchEvent::new(
            4200,
            MouseSwitch("LeftMouseButton"),
            //(50, 50),
        )),
        RawEvent::MouseRelease(MouseSwitchEvent::new(
            4300,
            MouseSwitch("LeftMouseButton"),
            //(50, 50),
        )),
        RawEvent::MousePress(MouseSwitchEvent::new(
            4400,
            MouseSwitch("LeftMouseButton"),
            //(150, 150),
        )),
        RawEvent::MouseRelease(MouseSwitchEvent::new(
            4500,
            MouseSwitch("LeftMouseButton"),
            //(150, 150),
        )),
    ];

    for event in events {
        println!("Ev: {:?}", event);
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
