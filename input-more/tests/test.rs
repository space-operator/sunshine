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

            coords -> context
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

    use input_core::*;
    use input_more::*;

    //type DurationMs = u64;
    type TimestampMs = u64;

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Switch {
        Keyboard(KeyboardSwitch),
        Mouse(MouseSwitch),
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct KeyboardSwitch(&'static str);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct MouseSwitch(&'static str);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct KeyboardTrigger(&'static str);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct MouseTrigger(&'static str);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct KeyboardCoords;

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct MouseCoords(u64, u64);

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

    trait BuildAppEvent<Co> {
        fn build(&self, coords: &Co) -> Option<AppEvent>;
    }

    impl BuildAppEvent<KeyboardCoords> for BasicAppEventBuilder {
        fn build(&self, _: &KeyboardCoords) -> Option<AppEvent> {
            match self {
                Self::Undo(id) => Some(AppEvent::Undo(*id)),
            }
        }
    }

    impl BuildAppEvent<MouseCoords> for PointerAppEventBuilder {
        fn build(&self, coords: &MouseCoords) -> Option<AppEvent> {
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

    type KeyboardMapping = Mapping<KeyboardSwitch, KeyboardTrigger, Switch, BasicAppEventBuilder>;
    type MouseMapping = Mapping<MouseSwitch, MouseTrigger, Switch, PointerAppEventBuilder>;

    type KeyboardSwitchEvent = SwitchEvent<TimestampMs, KeyboardSwitch>;
    type MouseSwitchEvent = SwitchEvent<TimestampMs, MouseSwitch>;
    type KeyboardTriggerEvent = TriggerEvent<TimestampMs, KeyboardTrigger>;
    type MouseTriggerEvent = TriggerEvent<TimestampMs, MouseTrigger>;
    type KeyboardCoordsEvent = CoordsEvent<TimestampMs, KeyboardCoords>;
    type MouseCoordsEvent = CoordsEvent<TimestampMs, MouseCoords>;

    type Modifiers = input_core::Modifiers<Switch>;
    type KeyboardTimedState = TimedState<KeyboardSwitch>;
    type MouseTimedState = TimedState<MouseSwitch>;

    type KeyboardCoordsState = CoordsState<KeyboardCoords>;
    type MouseCoordsState = CoordsState<MouseCoords>;

    type CustomScheduler<Sw, Re, Co> = DeviceSchedulerState<TimestampMs, Sw, Switch, Co, Re>;

    type KeyboardLongPressScheduler =
        CustomScheduler<KeyboardSwitch, LongPressHandleRequest, KeyboardCoords>;
    type KeyboardClickExactScheduler =
        CustomScheduler<KeyboardSwitch, ClickExactHandleRequest, KeyboardCoords>;
    type MouseLongPressScheduler =
        CustomScheduler<MouseSwitch, LongPressHandleRequest, MouseCoords>;
    type MouseClickExactScheduler =
        CustomScheduler<MouseSwitch, ClickExactHandleRequest, MouseCoords>;
    type KeyboardPointerState = PointerState<KeyboardSwitch, KeyboardCoords>;
    type MousePointerState = PointerState<MouseSwitch, MouseCoords>;

    type GlobalState = input_more::GlobalState<
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

    type GlobalMappingCache = input_more::GlobalMappingCache<
        DeviceMappingCache<KeyboardSwitch, KeyboardTrigger, Switch, BasicAppEventBuilder>,
        DeviceMappingCache<MouseSwitch, MouseTrigger, Switch, PointerAppEventBuilder>,
        MappingModifiersCache<Switch>,
    >;

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

    let mapping = GlobalMapping {
        keyboard: keyboard_mapping,
        mouse: mouse_mapping,
    };

    let mapping_cache = GlobalMappingCache::from_mapping(mapping);

    let mut global_state = GlobalState::new(
        Modifiers::default(),
        KeyboardCoordsState::with_coords(KeyboardCoords),
        MouseCoordsState::with_coords(MouseCoords(0, 0)),
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
        KeyboardTrigger(KeyboardTriggerEvent),
        KeyboardCoords(KeyboardCoordsEvent),
        MousePress(MouseSwitchEvent),
        MouseRelease(MouseSwitchEvent),
        MouseTrigger(MouseTriggerEvent),
        MouseCoords(MouseCoordsEvent),
    }

    impl RawEvent {
        fn time(&self) -> TimestampMs {
            match self {
                RawEvent::KeyboardPress(event) => event.time,
                RawEvent::KeyboardRelease(event) => event.time,
                RawEvent::KeyboardTrigger(event) => event.time,
                RawEvent::KeyboardCoords(event) => event.time,
                RawEvent::MousePress(event) => event.time,
                RawEvent::MouseRelease(event) => event.time,
                RawEvent::MouseTrigger(event) => event.time,
                RawEvent::MouseCoords(event) => event.time,
            }
        }
    }

    let events = [
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(1000, KeyboardSwitch("LeftShift"))),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(1100, KeyboardSwitch("LeftAlt"))),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(2000, KeyboardSwitch("LeftCtrl"))),
        RawEvent::KeyboardRelease(KeyboardSwitchEvent::new(2100, KeyboardSwitch("LeftCtrl"))),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(2200, KeyboardSwitch("LeftCtrl"))),
        RawEvent::KeyboardRelease(KeyboardSwitchEvent::new(2300, KeyboardSwitch("LeftCtrl"))),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(3000, KeyboardSwitch("LeftShift"))),
        RawEvent::KeyboardPress(KeyboardSwitchEvent::new(3100, KeyboardSwitch("LeftAlt"))),
        RawEvent::MouseCoords(MouseCoordsEvent::new(4000, MouseCoords(150, 150))),
        RawEvent::MousePress(MouseSwitchEvent::new(4100, MouseSwitch("LeftMouseButton"))),
        RawEvent::MouseRelease(MouseSwitchEvent::new(4200, MouseSwitch("LeftMouseButton"))),
        RawEvent::MouseCoords(MouseCoordsEvent::new(4300, MouseCoords(50, 50))),
        RawEvent::MousePress(MouseSwitchEvent::new(4400, MouseSwitch("LeftMouseButton"))),
        RawEvent::MouseRelease(MouseSwitchEvent::new(4500, MouseSwitch("LeftMouseButton"))),
        RawEvent::MouseCoords(MouseCoordsEvent::new(4600, MouseCoords(150, 150))),
        RawEvent::MousePress(MouseSwitchEvent::new(4700, MouseSwitch("LeftMouseButton"))),
        RawEvent::MouseRelease(MouseSwitchEvent::new(4800, MouseSwitch("LeftMouseButton"))),
    ];

    // fn

    for event in events {
        let result =
            global_state.with_timeout(event.time() - 1000, event.time() - 300, &mapping_cache);
        global_state = result.state;
        println!("Ti: {:?}", event.time());
        println!("BiKeLo: {:?}", result.keyboard_long_press);
        println!("BiKeCl: {:?}", result.keyboard_click_exact);
        println!("BiMsLo: {:?}", result.mouse_long_press);
        println!("BiMsCl: {:?}", result.mouse_click_exact);
        println!();

        println!("In: {:?}", event);
        let (state, scheduled, keyboard_bindings, mouse_bindings) = match event {
            RawEvent::KeyboardPress(event) => {
                let result = global_state.with_keyboard_press_event(event, &mapping_cache);
                (
                    result.state,
                    result.scheduled,
                    result.bindings.into_iter().collect(),
                    vec![],
                )
            }
            RawEvent::KeyboardRelease(event) => {
                let result = global_state.with_keyboard_release_event(event, &mapping_cache);
                (
                    result.state,
                    result.scheduled,
                    result.bindings.into_iter().collect(),
                    vec![],
                )
            }
            RawEvent::KeyboardTrigger(event) => {
                let result = global_state.with_keyboard_trigger_event(event, &mapping_cache);
                (
                    result.state,
                    None,
                    result.bindings.into_iter().collect(),
                    vec![],
                )
            }
            RawEvent::KeyboardCoords(event) => {
                let result = global_state.with_keyboard_coords_event(event, &mapping_cache);
                (result.state, None, result.bindings, vec![])
            }
            RawEvent::MousePress(event) => {
                let result = global_state.with_mouse_press_event(event, &mapping_cache);
                (
                    result.state,
                    result.scheduled,
                    vec![],
                    result.bindings.into_iter().collect(),
                )
            }
            RawEvent::MouseRelease(event) => {
                let result = global_state.with_mouse_release_event(event, &mapping_cache);
                (
                    result.state,
                    result.scheduled,
                    vec![],
                    result.bindings.into_iter().collect(),
                )
            }
            RawEvent::MouseTrigger(event) => {
                let result = global_state.with_mouse_trigger_event(event, &mapping_cache);
                (
                    result.state,
                    None,
                    vec![],
                    result.bindings.into_iter().collect(),
                )
            }
            RawEvent::MouseCoords(event) => {
                let result = global_state.with_mouse_coords_event(event, &mapping_cache);
                (result.state, None, vec![], result.bindings)
            }
        };
        global_state = state;
        println!("Sh: {:?}", scheduled);

        for (bindings, coords) in keyboard_bindings {
            println!("Bi: {:?}", bindings);
            let app_events = bindings.build(|builder| builder.build(&coords));
            println!("Ev: {:?}", app_events);
            println!();
        }

        for (bindings, coords) in mouse_bindings {
            println!("Bi: {:?}", bindings);
            let app_events = bindings.build(|builder| builder.build(&coords));
            println!("Ev: {:?}", app_events);
            println!();
        }
    }

    panic!();
}
