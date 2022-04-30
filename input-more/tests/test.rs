#[test]
fn test_chain() {
    use core::fmt::Debug;
    use core::hash::Hash;
    use std::collections::{HashMap, HashSet};

    use input_core::*;
    use input_more::*;

    type TimestampMs = i64;

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
    struct MouseCoords(i64, i64);

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

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct NodeId(u32);

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum AppEvent {
        Unselect,
        SelectNode(NodeId),
        CreateNode(MouseCoords),
        EditNode(NodeId),
        StartSelection(MouseCoords),
        ContinueSelection(MouseCoords, MouseCoords),
        EndSelection(MouseCoords, MouseCoords),
        CancelSelection,
        StartMove(MouseCoords),
        ContinueMove(MouseCoords, MouseCoords),
        EndMove(MouseCoords, MouseCoords),
        CancelMove,
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum BasicAppEventBuilder {
        Unselect,
        CancelSelection,
        CancelMove,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum PointerAppEventBuilder {
        Unselect,
        SelectNode,
        CreateNode,
        EditNode,
        StartSelection,
        EndSelection,
        CancelSelection,
        ContinueSelection,
        StartMove,
        EndMove,
        CancelMove,
        ContinueMove,
    }

    #[derive(Clone, Debug, Default)]
    struct AppContext {
        nodes: HashMap<NodeId, MouseCoords>,
        selected: HashSet<NodeId>,
        ui_state: UiState,
    }

    #[derive(Clone, Debug)]
    enum UiState {
        Default,
        Selection(MouseCoords),
        Move(MouseCoords),
    }

    impl Default for UiState {
        fn default() -> Self {
            Self::Default
        }
    }

    impl AppContext {
        fn node_at(&self, coords: &MouseCoords) -> Option<NodeId> {
            self.nodes
                .iter()
                .find(|(_, node_coords)| {
                    (node_coords.0 - coords.0).abs() < 10 && (node_coords.1 - coords.1).abs() < 10
                })
                .map(|(id, _)| *id)
        }
    }

    trait BuildAppEvent<Co> {
        fn build(&self, coords: &Co, ctx: &AppContext) -> Option<AppEvent>;
    }

    impl BuildAppEvent<KeyboardCoords> for BasicAppEventBuilder {
        fn build(&self, _: &KeyboardCoords, ctx: &AppContext) -> Option<AppEvent> {
            match self {
                Self::Unselect => {
                    if ctx.selected.is_empty() {
                        None
                    } else {
                        Some(AppEvent::Unselect)
                    }
                }
                Self::CancelSelection => match ctx.ui_state {
                    UiState::Selection(_) => Some(AppEvent::CancelSelection),
                    UiState::Default | UiState::Move(_) => None,
                },
                Self::CancelMove => match ctx.ui_state {
                    UiState::Move(_) => Some(AppEvent::CancelMove),
                    UiState::Default | UiState::Selection(_) => None,
                },
            }
        }
    }

    impl BuildAppEvent<MouseCoords> for PointerAppEventBuilder {
        fn build(&self, coords: &MouseCoords, ctx: &AppContext) -> Option<AppEvent> {
            match self {
                Self::Unselect => {
                    if ctx.selected.is_empty() {
                        None
                    } else {
                        Some(AppEvent::Unselect)
                    }
                }
                Self::SelectNode => ctx.node_at(coords).map(AppEvent::SelectNode),
                Self::CreateNode => Some(AppEvent::CreateNode(*coords)),
                Self::EditNode => ctx.node_at(coords).map(AppEvent::EditNode),
                Self::StartSelection => match ctx.ui_state {
                    UiState::Default => Some(AppEvent::StartSelection(*coords)),
                    UiState::Selection(_) | UiState::Move(_) => None,
                },
                Self::EndSelection => match ctx.ui_state {
                    UiState::Selection(start_coords) => {
                        Some(AppEvent::EndSelection(start_coords, *coords))
                    }
                    UiState::Default | UiState::Move(_) => None,
                },
                Self::CancelSelection => match ctx.ui_state {
                    UiState::Selection(_) => Some(AppEvent::CancelSelection),
                    UiState::Default | UiState::Move(_) => None,
                },
                Self::StartMove => match ctx.ui_state {
                    UiState::Default => Some(AppEvent::StartMove(*coords)),
                    UiState::Selection(_) | UiState::Move(_) => None,
                },
                Self::EndMove => match ctx.ui_state {
                    UiState::Move(start_coords) => Some(AppEvent::EndMove(start_coords, *coords)),
                    UiState::Default | UiState::Selection(_) => None,
                },
                Self::CancelMove => match ctx.ui_state {
                    UiState::Move(_) => Some(AppEvent::CancelMove),
                    UiState::Default | UiState::Selection(_) => None,
                },
                Self::ContinueSelection => match ctx.ui_state {
                    UiState::Selection(start_coords) => {
                        Some(AppEvent::ContinueSelection(start_coords, *coords))
                    }
                    UiState::Default | UiState::Move(_) => None,
                },
                Self::ContinueMove => match ctx.ui_state {
                    UiState::Move(start_coords) => {
                        Some(AppEvent::ContinueMove(start_coords, *coords))
                    }
                    UiState::Default | UiState::Selection(_) => None,
                },
            }
        }
    }

    #[allow(unused_assignments, unused_variables)]
    fn filter_by_priority(events: Vec<AppEvent>) -> impl Iterator<Item = AppEvent> {
        let mut is_unselect_used = false;
        let mut is_select_node_used = false;
        let mut is_create_node_used = false;
        let mut is_edit_node_used = false;
        let mut is_start_selection_used = false;
        let mut is_end_selection_used = false;
        let mut is_cancel_selection_used = false;
        let mut is_continue_selection_used = false;
        let mut is_start_move_used = false;
        let mut is_end_move_used = false;
        let mut is_cancel_move_used = false;
        let mut is_continue_move_used = false;
        for event in &events {
            match event {
                AppEvent::Unselect => is_unselect_used = true,
                AppEvent::SelectNode(_) => is_select_node_used = true,
                AppEvent::CreateNode(_) => is_create_node_used = true,
                AppEvent::EditNode(_) => is_edit_node_used = true,
                AppEvent::StartSelection(_) => is_start_selection_used = true,
                AppEvent::EndSelection(_, _) => is_end_selection_used = true,
                AppEvent::CancelSelection => is_cancel_selection_used = true,
                AppEvent::ContinueSelection(_, _) => is_continue_selection_used = true,
                AppEvent::StartMove(_) => is_start_move_used = true,
                AppEvent::EndMove(_, _) => is_end_move_used = true,
                AppEvent::CancelMove => is_cancel_move_used = true,
                AppEvent::ContinueMove(_, _) => is_continue_move_used = true,
            }
        }

        let is_end_or_cancel_move_or_selection = is_end_selection_used
            //|| is_cancel_selection_used
            || is_end_move_used
            || is_cancel_move_used;

        events.into_iter().filter(move |event| match event {
            AppEvent::Unselect => !is_create_node_used && !is_select_node_used,
            AppEvent::SelectNode(_) => !is_create_node_used,
            AppEvent::CreateNode(_) => !is_edit_node_used && !is_end_or_cancel_move_or_selection,
            AppEvent::EditNode(_) => !is_end_or_cancel_move_or_selection,
            AppEvent::StartSelection(_) => !is_create_node_used && !is_edit_node_used,
            AppEvent::EndSelection(_, _) => true,
            AppEvent::CancelSelection => true,
            AppEvent::ContinueSelection(_, _) => true,
            AppEvent::StartMove(_) => !is_create_node_used && !is_edit_node_used,
            AppEvent::EndMove(_, _) => true,
            AppEvent::CancelMove => true,
            AppEvent::ContinueMove(_, _) => true,
        })
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

    let lmb = MouseSwitch("LeftMouseButton");
    let rmb = MouseSwitch("RightMouseButton");
    let click = Some(TimedEventData {
        kind: TimedReleaseEventKind::Click,
        num_possible_clicks: 1,
    });
    let dbl_click = Some(TimedEventData {
        kind: TimedReleaseEventKind::Click,
        num_possible_clicks: 2,
    });

    let keyboard_mapping = KeyboardMapping::new(
        [Binding::Release(SwitchBinding {
            switch: KeyboardSwitch("Escape"),
            modifiers: Modifiers::new(),
            timed_data: click,
            pointer_data: None,
            event: BasicAppEventBuilder::Unselect,
        })]
        .into_iter()
        .collect(),
    );
    let mouse_mapping = MouseMapping::new(
        [
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: PointerAppEventBuilder::Unselect,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: PointerAppEventBuilder::SelectNode,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click,
                pointer_data: None,
                event: PointerAppEventBuilder::CreateNode,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click,
                pointer_data: None,
                event: PointerAppEventBuilder::EditNode,
            }),
            Binding::Press(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: (),
                pointer_data: (),
                event: PointerAppEventBuilder::StartSelection,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::CancelSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: Some(PointerReleaseEventData::DragEnd),
                event: PointerAppEventBuilder::EndSelection,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click, // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click, // FIXME
                pointer_data: Some(PointerReleaseEventData::DragEnd),
                event: PointerAppEventBuilder::EndSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click, // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click, // FIXME
                pointer_data: Some(PointerReleaseEventData::DragEnd),
                event: PointerAppEventBuilder::EndSelection,
            }),
            //
            Binding::Press(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(),
                timed_data: (),
                pointer_data: (),
                event: PointerAppEventBuilder::StartMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::CancelMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: Some(PointerReleaseEventData::DragEnd),
                event: PointerAppEventBuilder::EndMove,
            }),
            //
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: lmb,
                    kind: PointerMoveEventKind::DragMove,
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::ContinueSelection,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: lmb,
                    kind: PointerMoveEventKind::DragStart, // FIXME
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::ContinueSelection,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: rmb,
                    kind: PointerMoveEventKind::DragMove,
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::ContinueMove,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: rmb,
                    kind: PointerMoveEventKind::DragStart, // FIXME
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::ContinueMove,
            }),
        ]
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

    #[allow(dead_code)]
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

    let mut context = AppContext {
        nodes: [
            (NodeId(1), MouseCoords(100, 100)),
            (NodeId(2), MouseCoords(200, 200)),
            (NodeId(3), MouseCoords(300, 100)),
        ]
        .into_iter()
        .collect(),
        selected: [NodeId(2)].into_iter().collect(),
        ui_state: UiState::Default,
    };

    let mut delay = {
        let mut time = 1000;
        move |delay| {
            time += delay;
            time
        }
    };

    let events = [
        RawEvent::MouseCoords(MouseCoordsEvent::new(delay(100), MouseCoords(50, 50))),
        RawEvent::MousePress(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseRelease(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseCoords(MouseCoordsEvent::new(delay(2000), MouseCoords(100, 100))),
        RawEvent::MousePress(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseRelease(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseCoords(MouseCoordsEvent::new(delay(2000), MouseCoords(200, 200))),
        RawEvent::MousePress(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseRelease(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseCoords(MouseCoordsEvent::new(delay(2000), MouseCoords(300, 100))),
        RawEvent::MousePress(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseRelease(MouseSwitchEvent::new(delay(100), lmb)),
        //
        RawEvent::MouseCoords(MouseCoordsEvent::new(delay(2000), MouseCoords(79, 79))),
        RawEvent::MousePress(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseRelease(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MousePress(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseRelease(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseCoords(MouseCoordsEvent::new(delay(2000), MouseCoords(200, 200))),
        RawEvent::MousePress(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseRelease(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MousePress(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseRelease(MouseSwitchEvent::new(delay(100), lmb)),
        //
        RawEvent::MouseCoords(MouseCoordsEvent::new(delay(2000), MouseCoords(101, 101))),
        RawEvent::MousePress(MouseSwitchEvent::new(delay(100), lmb)),
        RawEvent::MouseCoords(MouseCoordsEvent::new(delay(100), MouseCoords(200, 200))),
        RawEvent::MouseRelease(MouseSwitchEvent::new(delay(100), lmb)),
    ];

    for event in events {
        println!("St: {:?}", global_state);
        println!("Co: {:?}", context);
        let result =
            global_state.with_timeout(event.time() - 1000, event.time() - 300, &mapping_cache);
        println!("Ti: {:?}", event.time());
        println!("BiKeLo: {:?}", result.keyboard_long_press);
        println!("BiKeCl: {:?}", result.keyboard_click_exact);
        println!("BiMsLo: {:?}", result.mouse_long_press);
        println!("BiMsCl: {:?}", result.mouse_click_exact);
        println!();

        println!("In: {:?}", event);
        let (scheduled, keyboard_bindings, mouse_bindings) = match event {
            RawEvent::KeyboardPress(event) => {
                let result = global_state.with_keyboard_press_event(event, &mapping_cache);
                (
                    result.scheduled,
                    result.bindings.into_iter().collect(),
                    vec![],
                )
            }
            RawEvent::KeyboardRelease(event) => {
                let result = global_state.with_keyboard_release_event(event, &mapping_cache);
                (
                    result.scheduled,
                    result.bindings.into_iter().collect(),
                    vec![],
                )
            }
            RawEvent::KeyboardTrigger(event) => {
                let result = global_state.with_keyboard_trigger_event(event, &mapping_cache);
                (None, result.bindings.into_iter().collect(), vec![])
            }
            RawEvent::KeyboardCoords(event) => {
                let result =
                    global_state.with_keyboard_coords_event(event, &mapping_cache, |a, b| a == b);
                (None, result.bindings, vec![])
            }
            RawEvent::MousePress(event) => {
                let result = global_state.with_mouse_press_event(event, &mapping_cache);
                (
                    result.scheduled,
                    vec![],
                    result.bindings.into_iter().collect(),
                )
            }
            RawEvent::MouseRelease(event) => {
                let result = global_state.with_mouse_release_event(event, &mapping_cache);
                (
                    result.scheduled,
                    vec![],
                    result.bindings.into_iter().collect(),
                )
            }
            RawEvent::MouseTrigger(event) => {
                let result = global_state.with_mouse_trigger_event(event, &mapping_cache);
                (None, vec![], result.bindings.into_iter().collect())
            }
            RawEvent::MouseCoords(event) => {
                let result =
                    global_state.with_mouse_coords_event(event, &mapping_cache, |lhs, rhs| {
                        (lhs.0 - rhs.0).pow(2) + (lhs.1 - rhs.1).pow(2) >= 5 * 5
                    });
                (None, vec![], result.bindings)
            }
        };
        println!("Sh: {:?}", scheduled);

        let mut all_app_events = Vec::new();

        for (bindings, coords) in keyboard_bindings {
            println!("Bi: {:?}", bindings);
            let app_events = bindings.build(|builder| builder.build(&coords, &context));
            println!("Ev: {:?}", app_events);
            all_app_events.extend(filter_by_priority(app_events));
        }

        for (bindings, coords) in mouse_bindings {
            println!("Bi: {:?}", bindings);
            let app_events = bindings.build(|builder| builder.build(&coords, &context));
            println!("Ev: {:?}", app_events);
            all_app_events.extend(filter_by_priority(app_events));
        }

        println!("Ev: {:?}", all_app_events);
        for event in all_app_events {
            match event {
                AppEvent::Unselect
                | AppEvent::SelectNode(_)
                | AppEvent::CreateNode(_)
                | AppEvent::EditNode(_)
                | AppEvent::ContinueMove(_, _)
                | AppEvent::ContinueSelection(_, _) => {}
                AppEvent::StartSelection(start) => {
                    context.ui_state = UiState::Selection(start);
                }
                AppEvent::EndSelection(_, _) => {
                    context.ui_state = UiState::Default;
                }
                AppEvent::CancelSelection => {
                    context.ui_state = UiState::Default;
                }
                AppEvent::StartMove(start) => {
                    context.ui_state = UiState::Move(start);
                }
                AppEvent::EndMove(_, _) => {
                    context.ui_state = UiState::Default;
                }
                AppEvent::CancelMove => {
                    context.ui_state = UiState::Default;
                }
            }
        }
        println!();
    }

    panic!();
}
