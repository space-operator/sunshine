use crate::ui_event::*;
use derive_more::From;
use std::collections::HashSet;

pub const MAX_CLICK_TIME_MS: u32 = 300;

pub type TimestampMs = u32;
pub type NodeId = u32;

pub type GraphCoord = i32;

pub trait Context {
    fn schedule_timeout(&mut self, timestamp: TimestampMs);
    fn get_node_by_coords(&mut self, x: GraphCoord, y: GraphCoord) -> Option<NodeId>;
    fn select_node(&mut self, node_id: NodeId);
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
    EmptyMouseDown(StateEmptyMouseDown),
}

impl Default for State {
    fn default() -> State {
        State::Default(StateDefault::default())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct StateDefault {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateNodeMouseDown {
    node_id: NodeId,
    x: MouseCoord,
    y: MouseCoord,
    timestamp: TimestampMs,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateNodeMouseLongDown {
    node_id: NodeId,
    x: MouseCoord,
    y: MouseCoord,
    timestamp: TimestampMs,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateNodeMouseClick {
    node_id: NodeId,
    timestamp: TimestampMs,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateEmptyMouseDown {
    x: MouseCoord,
    y: MouseCoord,
    timestamp: TimestampMs,
}

impl State {
    /*pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: Timestamp) -> Self {
        match self {
            State::Default => State::Default
            NodeMouseDown {
                node_id,
                x,
                y,
                timestamp,
            } => {
                let dt = ev_timestamp - timestamp;
                if dt >= MAX_CLICK_TIME {
                    NodeMouseLongDown {
                        node_id,
                        x,
                        y,
                        timestamp,
                    }
                } else {
                    context.schedule_timeout(timestamp + MAX_CLICK_TIME);
                }
            }
            _ => todo!(),
        }
    }*/

    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> Self {
        match self {
            Self::Default(state) => state.on_timeout(context, ev_timestamp),
            Self::NodeMouseDown(state) => state.on_timeout(context, ev_timestamp),
            Self::NodeMouseLongDown(state) => state.on_timeout(context, ev_timestamp),
            Self::NodeMouseClick(state) => state.on_timeout(context, ev_timestamp),
            Self::EmptyMouseDown(state) => state.on_timeout(context, ev_timestamp),
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
            Self::EmptyMouseDown(state) => state.apply(context, ev, ev_timestamp),
        }
    }
}

impl StateDefault {
    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
        todo!()
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        match ev {
            UiEvent::MouseDown(ev) => {
                if let Some(node_id) = context.get_node_by_coords(ev.x, ev.y) {
                    context.schedule_timeout(ev_timestamp + MAX_CLICK_TIME_MS);
                    StateNodeMouseDown {
                        node_id: node_id,
                        x: ev.x,
                        y: ev.y,
                        timestamp: ev_timestamp,
                    }
                    .into()
                } else {
                    context.schedule_timeout(ev_timestamp + MAX_CLICK_TIME_MS);
                    StateEmptyMouseDown {
                        x: ev.x,
                        y: ev.y,
                        timestamp: ev_timestamp,
                    }
                    .into()
                }
            }
            UiEvent::MouseUp(ev) => todo!(),
            UiEvent::MouseMove(ev) => todo!(),
            _ => todo!(),
        }
    }
}

impl StateNodeMouseDown {
    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
        let dt = ev_timestamp - self.timestamp;
        if dt >= MAX_CLICK_TIME_MS {
            StateNodeMouseLongDown {
                node_id: self.node_id,
                x: self.x,
                y: self.y,
                timestamp: self.timestamp,
            }
            .into()
        } else {
            context.schedule_timeout(self.timestamp + MAX_CLICK_TIME_MS);
            self.into()
        }
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        match ev {
            UiEvent::MouseDown(ev) => {
                eprintln!("oh no");
                self.into()
            }
            UiEvent::MouseUp(ev) => {
                context.select_node(self.node_id);
                StateNodeMouseClick {
                    node_id: self.node_id,
                    timestamp: ev_timestamp,
                }
                .into()
            }
            UiEvent::MouseMove(ev) => todo!(),
            _ => todo!(),
        }
    }
}

impl StateNodeMouseLongDown {
    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
        todo!()
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        todo!()
    }
}

impl StateNodeMouseClick {
    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
        todo!()
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        todo!()
    }
}

impl StateEmptyMouseDown {
    pub fn on_timeout<T: Context>(self, context: &mut T, ev_timestamp: TimestampMs) -> State {
        todo!()
    }

    pub fn apply<T: Context>(
        self,
        context: &mut T,
        ev: UiEvent,
        ev_timestamp: TimestampMs,
    ) -> State {
        todo!()
    }
}

#[test]
fn test() {
    pub struct DummyContext {
        pub last_selected_node_id: Option<NodeId>,
    }

    impl Context for DummyContext {
        fn schedule_timeout(&mut self, timestamp: TimestampMs) {}

        fn get_node_by_coords(&mut self, x: GraphCoord, y: GraphCoord) -> Option<NodeId> {
            if x > 200 {
                Some(2)
            } else if x > 100 {
                Some(1)
            } else {
                None
            }
        }

        fn select_node(&mut self, node_id: NodeId) {
            self.last_selected_node_id = Some(node_id);
        }
    }

    let mut context = DummyContext {
        last_selected_node_id: None,
    };

    let st = State::default();
    let st = st.apply(
        &mut context,
        UiEventMouseDown {
            x: 150,
            y: 50,
            button: MouseButton::Left,
        }
        .into(),
        10000,
    );
    let st = st.apply(
        &mut context,
        UiEventMouseUp {
            x: 150,
            y: 50,
            button: MouseButton::Left,
        }
        .into(),
        10200,
    );
    assert_eq!(context.last_selected_node_id, Some(1));
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
