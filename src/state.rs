use crate::ui_event::*;
use core::panic;
use derive_more::From;
use std::collections::HashSet;
use std::sync::{Arc, Weak};

pub const MAX_CLICK_TIME_MS: u32 = 300;
pub const MAX_DBG_CLICK_TIME_MS: u32 = 600;

pub type TimestampMs = u32;
pub type NodeId = u32;

pub type GraphCoords = (i32, i32);

pub type ScheduledTimeout = Arc<TimestampMs>;

pub trait Context {
    fn schedule_timeout(&mut self, timestamp: TimestampMs) -> ScheduledTimeout;

    fn get_node_by_coords(&mut self, coords: GraphCoords) -> Option<NodeId>;

    fn select_node(&mut self, node_id: NodeId);

    fn rename_node(&mut self, node_id: NodeId);

    fn show_node_menu(&mut self, node_id: NodeId);

    fn nest_nodes(&mut self, nested_node_id: NodeId, parent_node_id: NodeId);
}

#[derive(Clone, Debug)]
pub struct StateSet {
    states: HashSet<State>,
}

#[derive(From, Clone, Debug, Eq, PartialEq)]
pub enum State {
    Default(StateDefault),
    NodeMouseDown(StateNodeMouseDown),
    NodeMouseLongDown(StateNodeMouseLongDown),
    NodeMouseClick(StateNodeMouseClick),
    NodeMouseClickDown(StateNodeMouseClickDown),
    EmptyMouseDown(StateEmptyMouseDown),
    NodeMouseMove(StateNodeMouseMove),
    // NodeMouseMoveMouseUp(StateNodeMouseMoveMouseUp),
}

impl Default for State {
    fn default() -> State {
        State::Default(StateDefault::default())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct StateDefault {}

// Needs a timeout?
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateEmptyMouseDown {
    coords: MouseCoords,
    timestamp: TimestampMs,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateNodeMouseDown {
    node_id: NodeId,
    coords: MouseCoords,
    start_timestamp: TimestampMs,
    timeout: ScheduledTimeout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateNodeMouseLongDown {
    node_id: NodeId,
    coords: MouseCoords,
    start_timestamp: TimestampMs,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateNodeMouseClick {
    node_id: NodeId,
    coords: MouseCoords,
    start_timestamp: TimestampMs,
    timeout: ScheduledTimeout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateNodeMouseClickDown {
    node_id: NodeId,
    coords: MouseCoords,
    start_timestamp: TimestampMs,
    timeout: ScheduledTimeout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateNodeMouseMove {
    node_id: NodeId,
    coords: MouseCoords,
    start_timestamp: TimestampMs,
    // delta
}

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct StateNodeMouseMoveMouseUp {
//     node_id: NodeId,
//     coords: MouseCoords,
//     start_timestamp: TimestampMs,
//     end_node_id: Option<NodeId>,
// }

impl State {
    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> Self {
        match self {
            Self::Default(state) => state.on_timeout(context, ev_timestamp),
            Self::NodeMouseDown(state) => state.on_timeout(context, ev_timestamp),
            Self::NodeMouseLongDown(state) => state.on_timeout(context, ev_timestamp),
            Self::NodeMouseClick(state) => state.on_timeout(context, ev_timestamp),
            Self::NodeMouseClickDown(state) => state.on_timeout(context, ev_timestamp),
            Self::EmptyMouseDown(state) => state.on_timeout(context, ev_timestamp),
            Self::NodeMouseMove(state) => state.on_timeout(context, ev_timestamp),
            // Self::NodeMouseMoveMouseUp(state) => state.on_timeout(context, ev_timestamp),
        }
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> Self {
        match self {
            Self::Default(state) => state.apply(context, ev, ev_timestamp),
            Self::NodeMouseDown(state) => state.apply(context, ev, ev_timestamp),
            Self::NodeMouseLongDown(state) => state.apply(context, ev, ev_timestamp),
            Self::NodeMouseClick(state) => state.apply(context, ev, ev_timestamp),
            Self::NodeMouseClickDown(state) => state.apply(context, ev, ev_timestamp),
            Self::EmptyMouseDown(state) => state.apply(context, ev, ev_timestamp),
            Self::NodeMouseMove(state) => state.apply(context, ev, ev_timestamp),
            // Self::NodeMouseMoveMouseUp(state) => state.apply(context, ev, ev_timestamp),
        }
    }
}

impl StateDefault {
    pub fn new<T: Context>(context: &mut T) -> Self {
        Self {}
    }

    pub fn on_timeout<T: Context>(self, _: &mut T, _: TimestampMs) -> State {
        panic!();
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        match ev {
            UiEvent::MouseDown(ev) => {
                if let Some(node_id) = context.get_node_by_coords(ev.coords) {
                    StateNodeMouseDown::new(context, node_id, ev.coords, ev_timestamp).into()
                } else {
                    StateEmptyMouseDown::new(context, ev.coords, ev_timestamp).into()
                }
            }
            UiEvent::MouseUp(ev) => {
                panic!();
            }
            UiEvent::MouseMove(ev) => todo!(),
            _ => todo!(),
        }
    }
}

impl StateNodeMouseDown {
    pub fn new<T: Context>(
        context: &mut T,
        node_id: NodeId,
        coords: MouseCoords,
        timestamp: TimestampMs,
    ) -> Self {
        Self {
            node_id,
            coords,
            start_timestamp: timestamp,
            timeout: context.schedule_timeout(timestamp + MAX_CLICK_TIME_MS),
        }
    }

    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
        context.show_node_menu(self.node_id);
        StateNodeMouseLongDown::new(context, self.node_id, self.coords, self.start_timestamp).into()
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        match ev {
            UiEvent::MouseDown(ev) => {
                panic!();
            }
            UiEvent::MouseUp(ev) => {
                context.select_node(self.node_id);
                StateNodeMouseClick::new(context, self.node_id, self.coords, self.start_timestamp)
                    .into()
            }
            UiEvent::MouseMove(ev) => {
                // only after certain threshold
                context.select_node(self.node_id);
                StateNodeMouseMove::new(context, self.node_id, self.coords, self.start_timestamp)
                    .into()
            }
            _ => todo!(),
        }
    }
}

impl StateNodeMouseMove {
    pub fn new<T: Context>(
        context: &mut T,
        node_id: NodeId,
        coords: MouseCoords,
        timestamp: TimestampMs,
    ) -> Self {
        Self {
            node_id,
            coords,
            start_timestamp: timestamp,
        }
    }

    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
        panic!();
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        match ev {
            UiEvent::MouseDown(ev) => {
                panic!();
            }
            UiEvent::MouseUp(ev) => {
                let hover_node_id = context.get_node_by_coords(ev.coords);

                if hover_node_id != Some(self.node_id) {
                    context.nest_nodes(hover_node_id.unwrap(), self.node_id);
                    StateDefault::new(context).into()
                } else {
                    StateDefault::new(context).into()
                }
                //
            }
            UiEvent::MouseMove(ev) => {
                // 1. above minimum threshold
                //
                // hover above another node
                //
                StateNodeMouseMove::new(context, self.node_id, self.coords, self.start_timestamp)
                    .into()
            }
            _ => todo!(),
        }
    }
}

// impl StateNodeMouseMoveMouseUp {
//     pub fn new<T: Context>(
//         context: &mut T,
//         node_id: NodeId,
//         coords: MouseCoords,
//         timestamp: TimestampMs,
//         end_node_id: Option<NodeId>,
//     ) -> Self {
//         Self {
//             node_id,
//             coords,
//             start_timestamp: timestamp,
//             end_node_id,
//         }
//     }

//     pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
//         panic!();
//     }

//     pub fn apply<T: Context>(
//         self,
//         context: &mut T,
//         ev: UiEvent,
//         ev_timestamp: TimestampMs,
//     ) -> State {
//         match ev {
//             UiEvent::MouseDown(ev) => {
//                 panic!();
//             }
//             UiEvent::MouseUp(ev) => {
//                 //
//             }
//             _ => todo!(),
//         }
//     }
// }

impl StateNodeMouseLongDown {
    pub fn new<T: Context>(
        context: &mut T,
        node_id: NodeId,
        coords: MouseCoords,
        timestamp: TimestampMs,
    ) -> Self {
        Self {
            node_id,
            coords,
            start_timestamp: timestamp,
        }
    }

    pub fn on_timeout<T: Context>(self, _: &mut T, _: TimestampMs) -> State {
        panic!();
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        match ev {
            UiEvent::MouseDown(ev) => {
                panic!();
            }
            UiEvent::MouseUp(ev) => StateDefault::new(context).into(),
            UiEvent::MouseMove(ev) => todo!(),
            _ => todo!(),
        }
    }
}

impl StateNodeMouseClick {
    pub fn new<T: Context>(
        context: &mut T,
        node_id: NodeId,
        coords: MouseCoords,
        start_timestamp: TimestampMs,
    ) -> Self {
        Self {
            node_id,
            coords,
            start_timestamp,
            timeout: context.schedule_timeout(start_timestamp + MAX_DBG_CLICK_TIME_MS),
        }
    }

    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
        context.show_node_menu(self.node_id);
        StateNodeMouseLongDown::new(context, self.node_id, self.coords, self.start_timestamp).into()
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        match ev {
            UiEvent::MouseDown(ev) => {
                StateNodeMouseClickDown::new(context, self.node_id, ev.coords, self.start_timestamp)
                    .into()
            }
            UiEvent::MouseUp(ev) => {
                panic!();
            }
            UiEvent::MouseMove(ev) => todo!(),
            _ => todo!(),
        }
    }
}

impl StateNodeMouseClickDown {
    pub fn new<T: Context>(
        context: &mut T,
        node_id: NodeId,
        coords: MouseCoords,
        start_timestamp: TimestampMs,
    ) -> Self {
        Self {
            node_id,
            coords,
            start_timestamp,
            timeout: context.schedule_timeout(start_timestamp + MAX_DBG_CLICK_TIME_MS),
        }
    }

    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
        context.show_node_menu(self.node_id);
        StateNodeMouseLongDown::new(context, self.node_id, self.coords, self.start_timestamp).into()
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        match ev {
            UiEvent::MouseDown(ev) => {
                panic!();
            }
            UiEvent::MouseUp(ev) => {
                context.rename_node(self.node_id);
                StateDefault::new(context).into()
            }
            UiEvent::MouseMove(ev) => todo!(),
            _ => todo!(),
        }
    }
}

impl StateEmptyMouseDown {
    pub fn new<T: Context>(context: &mut T, coords: MouseCoords, timestamp: TimestampMs) -> Self {
        Self { coords, timestamp }
    }

    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
        panic!();
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        match ev {
            UiEvent::MouseDown(ev) => {
                panic!();
            }
            UiEvent::MouseUp(ev) => StateDefault::new(context).into(),
            UiEvent::MouseMove(ev) => todo!(),
            _ => todo!(),
        }
    }
}

#[test]
fn test() {
    #[derive(Clone, Debug, Default)]
    pub struct DummyContext {
        pub timeout: Weak<TimestampMs>,
        pub selected_node_ids: Vec<NodeId>,
        pub renamable_node_ids: Vec<NodeId>,
        pub menu_node_ids: Vec<NodeId>,
    }

    fn apply(
        mut state: State,
        context: &mut DummyContext,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        while let Some(timeout) = context.timeout.upgrade() {
            if *timeout <= ev_timestamp {
                context.timeout = Weak::new();
                state = state.on_timeout(context, *timeout);
                dbg!(&state);
            } else {
                break;
            }
        }
        state.apply(context, ev, ev_timestamp)
    }

    impl Context for DummyContext {
        fn schedule_timeout(&mut self, timestamp: TimestampMs) -> ScheduledTimeout {
            let timeout = Arc::new(timestamp);
            self.timeout = Arc::downgrade(&timeout);
            timeout
        }

        fn get_node_by_coords(&mut self, coords: GraphCoords) -> Option<NodeId> {
            if coords.0 > 200 {
                Some(2)
            } else if coords.0 > 100 {
                Some(1)
            } else {
                None
            }
        }

        fn select_node(&mut self, node_id: NodeId) {
            self.selected_node_ids.push(node_id);
        }

        fn rename_node(&mut self, node_id: NodeId) {
            self.renamable_node_ids.push(node_id);
        }

        fn show_node_menu(&mut self, node_id: NodeId) {
            self.menu_node_ids.push(node_id);
        }

        fn nest_nodes(&mut self, nested_node_id: NodeId, parent_node_id: NodeId) {
            todo!()
        }
    }

    let mut context = DummyContext::default();

    let state = State::default();
    dbg!(&state);
    let state = apply(
        state,
        &mut context,
        UiEventMouseDown {
            coords: (150, 50),
            button: MouseButton::Left,
        }
        .into(),
        10000,
    );
    dbg!(&state);
    let state = apply(
        state,
        &mut context,
        UiEventMouseUp {
            coords: (150, 50),
            button: MouseButton::Left,
        }
        .into(),
        10100,
    );
    dbg!(&state);
    let state = apply(
        state,
        &mut context,
        UiEventMouseDown {
            coords: (150, 50),
            button: MouseButton::Left,
        }
        .into(),
        10200,
    );
    dbg!(&state);
    let state = apply(
        state,
        &mut context,
        UiEventMouseUp {
            coords: (150, 50),
            button: MouseButton::Left,
        }
        .into(),
        10300,
    );
    dbg!(&state);
    assert_eq!(context.selected_node_ids, vec![1]);
    assert_eq!(context.renamable_node_ids, vec![1]);
    assert_eq!(context.menu_node_ids, vec![0; 0]);
}

/*pub fn apply_mouse_down(self, context: &mut T, ev: UiEventMouseDown, ev_timestamp: Timestamp) {
    mst() {
    let st = State::
}atch self {
                x,
                y,
                button: MouseButton::Left,
            },
        ) => {
            if let Some(node_id) = context.get_node_by_coords(x, y) {
                context.schedule_timeout(ev_timestamp + MAX_CLICK_TIME);
                State::NodeMouseDown {
                    node_id,
                    x,
                    y,
                    timestamp: ev_timestamp,
                }
            } else {
                todo!();
            }
        }
        (
            State::NodeMouseDown {
                node_id,
                x,
                y,
                timestamp,
            },
            UiEvent::MouseUp {
                x,
                y,
                button: MouseButton::Left,
            },
        ) => {
            State::NodeMouseClick {
                node_id: node_id,
                timestamp: ev_timestamp,
            }
        }
        (Default, UiEvent::MouseUp { .. }) => State::Default,
        _ => todo!(),
    }
}*/

/*
            -> mousedown -> mouseup -> mousedown -> mouseup
     initial                select                  rename


    click node
        select
    click empty
        deselect

    dblclick node
        rename
    dblclick empty
        create node

    right
        menu

*/

// click
// down <300ms up

//          Default
//              |
// down     NodeMouseDown           -> NodeMouseLongDown
//              |
// up       NodeMouseClick, click   -> Default
//              |
// down     NodeMouseClickDown      -> NodeMouseLongDown
//              |
// up       Default, dblclick
