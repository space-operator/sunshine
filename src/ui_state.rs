// use crate::ui_event::*;
// use core::panic;
// use derive_more::From;
// use std::collections::HashSet;
// use std::sync::{Arc, Weak};

// pub const MAX_CLICK_TIME_MS: u64 = 300;
// pub const MAX_DBG_CLICK_TIME_MS: u64 = 600;

// pub type NodeId = u32;

// pub type GraphCoords = (i32, i32);

// pub type ScheduledTimeout = Arc<UiEventTimestamp>;

// pub trait Context {
//     fn max_click_time(&self) -> UiEventTimeDeltaMs;

//     fn max_dbl_click_interval(&self) -> UiEventTimeDeltaMs;

//     fn min_long_touch_time(&self) -> UiEventTimeDeltaMs;

//     fn schedule_timeout(&mut self, timestamp: UiEventTimestamp) -> ScheduledTimeout;

//     fn emit_event(&mut self, ev: HighLevelUiEvent);

//     /*fn get_node_by_coords(&mut self, coords: GraphCoords) -> Option<NodeId>;

//     fn select_node(&mut self, node_id: NodeId);

//     fn rename_node(&mut self, node_id: NodeId);

//     fn show_node_menu(&mut self, node_id: NodeId);

//     fn nest_nodes(&mut self, nested_node_id: NodeId, parent_node_id: NodeId);*/
// }

// /*
//     LowLevelUiEventKind | Timeout
//         =>
//     UiStateKind change, emit some HighLevelUiEventKind

// */
// pub trait UiState: Sized {
//     fn on_timeout<T: Context>(self, ctx: &mut T, timestamp: UiEventTimestamp) -> UiStateKind {
//         panic!("state should not be called by on a timeout");
//     }

//     fn apply<T: Context>(self, ctx: &mut T, ev: UiEvent) -> UiStateKind;
// }

// #[derive(From, Clone, Debug, Eq, PartialEq)]
// pub enum UiStateKind {
//     Default(UiDefaultState),
//     MousePressed(UiMousePressedState),
//     MouseMoveMaybeStart(UiMouseMoveMaybeStartState),
//     MouseMoveStart(UiMouseMoveStartState),
//     MouseMoving(UiMouseMovingState),
//     MouseMoveEnd(UiMouseMoveState),
//     MouseClick(UiMouseClickState),
//     MouseClickExact(UiMouseClickExactState),
//     MouseWheel(UiMouseWheelState),
//     TouchStart(UiTouchStartState),
//     TouchMoving(UiTouchMoveState),
//     TouchMoveEnd(UiTouchMoveState),
//     TouchClick(UiTouchClickState),
//     TouchClickExact(UiTouchClickExactState),
// }

// impl Default for UiStateKind {
//     fn default() -> UiStateKind {
//         UiStateKind::Default(StateDefault::default())
//     }
// }

// #[derive(Clone, Debug, Eq, PartialEq, Default)]
// pub struct UiDefaultState {
//     modifiers: Modifiers,
// }

// #[derive(Clone, Debug, Eq, PartialEq, Default)]
// pub struct UiMousePressedState {
//     modifiers: Modifiers,
//     coords: UiEventCoords,
//     timestamp: UiEventTimestamp,
// }

// impl UiDefaultState {
//     pub fn new<T: Context>(ctx: &mut T) -> Self {
//         Self {}
//     }
// }

// impl UiState for UiDefaultState {
//     fn apply<T: Context>(self, ctx: &mut T, ev: LowLevelUiEvent) -> UiStateKind {
//         let timestamp = ev.timestamp;
//         match ev.kind {
//             UiLowLevelEventKind::MouseDown(ev) => {
//                 UiMousePressedState::new(ctx, ev.coords, timestamp)
//             }
//             UiLowLevelEventKind::MouseUp(ev) => {
//                 panic!();
//             }
//             UiLowLevelEventKind::MouseMove(ev) => ctx.emit_event(HighLevelUiEvent {
//                 timestamp,
//                 modifiers: self.modifiers.clone(),
//                 kind: HighLevelUiEventKind::MouseMove(ev),
//             }),
//             UiLowLevelEventKind::MouseWheel(ev) => ctx.emit_event(HighLevelUiEvent {
//                 timestamp,
//                 modifiers: self.modifiers.clone(),
//                 kind: HighLevelUiEventKind::MouseWheel(ev),
//             }),
//             UiLowLevelEventKind::TouchStart(ev) => UiTouchStart::new(ctx, ev.coords, timestamp),
//             UiLowLevelEventKind::TouchEnd(ev) => {
//                 panic!();
//             }
//             UiLowLevelEventKind::TouchMove(ev) => {
//                 panic!();
//             }
//             UiLowLevelEventKind::KeyDown(ev) => {
//                 let is_added = self.modifiers.insert(ev.key);
//                 assert!(is_added);
//             }
//             UiLowLevelEventKind::KeyUp(ev) => {
//                 let is_removed = self.modifiers.remove(ev.key);
//                 assert!(is_removed);
//                 if is_removed {
//                     ctx.emit_event(HighLevelUiEvent {
//                         timestamp,
//                         modifiers: self.modifiers.clone(),
//                         kind: HighLevelUiEventKind::Key(ev),
//                     })
//                 }
//             }
//             UiLowLevelEventKind::Char(ev) => ctx.emit_event(HighLevelUiEvent {
//                 timestamp,
//                 modifiers: self.modifiers.clone(),
//                 kind: HighLevelUiEventKind::Char(ev),
//             }),
//             _ => todo!(),
//         }
//     }
// }

// /*
// // Needs a timeout?
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct StateEmptyMouseDown {
//     coords: MouseCoords,
//     timestamp: UiEventTimestamp,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct StateNodeMouseDown {
//     node_id: NodeId,
//     coords: MouseCoords,
//     start_timestamp: UiEventTimestamp,
//     timeout: ScheduledTimeout,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct StateNodeMouseLongDown {
//     node_id: NodeId,
//     coords: MouseCoords,
//     start_timestamp: UiEventTimestamp,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct StateNodeMouseClick {
//     node_id: NodeId,
//     coords: MouseCoords,
//     start_timestamp: UiEventTimestamp,
//     timeout: ScheduledTimeout,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct StateNodeMouseClickDown {
//     node_id: NodeId,
//     coords: MouseCoords,
//     start_timestamp: UiEventTimestamp,
//     timeout: ScheduledTimeout,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct StateNodeMouseMove {
//     node_id: NodeId,
//     coords: MouseCoords,
//     start_timestamp: UiEventTimestamp,
//     // delta
// }

// // #[derive(Clone, Debug, Eq, PartialEq)]
// // pub struct StateNodeMouseMoveMouseUp {
// //     node_id: NodeId,
// //     coords: MouseCoords,
// //     start_timestamp: TimestampMs,
// //     end_node_id: Option<NodeId>,
// // }

// impl UiState for UiStateKind {
//     fn on_timeout<T: Context>(self, ctx: &mut T, timestamp: UiEventTimestamp) -> Self {
//         match self {
//             Self::Default(state) => state.on_timeout(ctx, timestamp),
//             Self::NodeMouseDown(state) => state.on_timeout(ctx, timestamp),
//             Self::NodeMouseLongDown(state) => state.on_timeout(ctx, timestamp),
//             Self::NodeMouseClick(state) => state.on_timeout(ctx, timestamp),
//             Self::NodeMouseClickDown(state) => state.on_timeout(ctx, timestamp),
//             Self::EmptyMouseDown(state) => state.on_timeout(ctx, timestamp),
//             Self::NodeMouseMove(state) => state.on_timeout(ctx, timestamp),
//             // Self::NodeMouseMoveMouseUp(state) => state.on_timeout(ctx),
//         }
//     }

//     fn apply<T: Context>(self, ctx: &mut T, ev: UiEvent) -> Self {
//         match self {
//             Self::Default(state) => state.apply(ctx, ev),
//             Self::NodeMouseDown(state) => state.apply(ctx, ev),
//             Self::NodeMouseLongDown(state) => state.apply(ctx, ev),
//             Self::NodeMouseClick(state) => state.apply(ctx, ev),
//             Self::NodeMouseClickDown(state) => state.apply(ctx, ev),
//             Self::EmptyMouseDown(state) => state.apply(ctx, ev),
//             Self::NodeMouseMove(state) => state.apply(ctx, ev),
//             // Self::NodeMouseMoveMouseUp(state) => state.apply(ctx, ev),
//         }
//     }
// }

// impl StateDefault {
//     pub fn new<T: Context>(ctx: &mut T) -> Self {
//         Self {}
//     }
// }

// impl UiState for StateDefault {
//     fn apply<T: Context>(self, ctx: &mut T, ev: UiEvent) -> UiStateKind {
//         let timestamp = ev.timestamp;
//         match ev.kind {
//             UiEventKind::MouseDown(ev) => {
//                 if let Some(node_id) = ctx.get_node_by_coords(ev.coords) {
//                     StateNodeMouseDown::new(ctx, node_id, ev.coords, timestamp).into()
//                 } else {
//                     StateEmptyMouseDown::new(ctx, ev.coords, timestamp).into()
//                 }
//             }
//             UiEventKind::MouseUp(ev) => {
//                 panic!();
//             }
//             UiEventKind::MouseMove(ev) => todo!(),
//             _ => todo!(),
//         }
//     }
// }

// impl StateNodeMouseDown {
//     pub fn new<T: Context>(
//         ctx: &mut T,
//         node_id: NodeId,
//         coords: MouseCoords,
//         timestamp: UiEventTimestamp,
//     ) -> Self {
//         Self {
//             node_id,
//             coords,
//             start_timestamp: timestamp,
//             timeout: ctx.schedule_timeout(timestamp + ctx.max_click_time()),
//         }
//     }
// }

// impl UiState for StateNodeMouseDown {
//     fn on_timeout<T: Context>(self, ctx: &mut T, timestamp: UiEventTimestamp) -> UiStateKind {
//         ctx.show_node_menu(self.node_id);
//         StateNodeMouseLongDown::new(ctx, self.node_id, self.coords, self.start_timestamp).into()
//     }

//     fn apply<T: Context>(self, ctx: &mut T, ev: UiEvent) -> UiStateKind {
//         match ev.kind {
//             UiEventKind::MouseDown(ev) => {
//                 panic!();
//             }
//             UiEventKind::MouseUp(ev) => {
//                 ctx.select_node(self.node_id);
//                 StateNodeMouseClick::new(ctx, self.node_id, self.coords, self.start_timestamp)
//                     .into()
//             }
//             UiEventKind::MouseMove(ev) => {
//                 // only after certain threshold
//                 ctx.select_node(self.node_id);
//                 StateNodeMouseMove::new(ctx, self.node_id, self.coords, self.start_timestamp).into()
//             }
//             _ => todo!(),
//         }
//     }
// }

// impl StateNodeMouseMove {
//     pub fn new<T: Context>(
//         ctx: &mut T,
//         node_id: NodeId,
//         coords: MouseCoords,
//         timestamp: UiEventTimestamp,
//     ) -> Self {
//         Self {
//             node_id,
//             coords,
//             start_timestamp: timestamp,
//         }
//     }
// }

// impl UiState for StateNodeMouseMove {
//     fn apply<T: Context>(self, ctx: &mut T, ev: UiEvent) -> UiStateKind {
//         match ev.kind {
//             UiEventKind::MouseDown(ev) => {
//                 panic!();
//             }
//             UiEventKind::MouseUp(ev) => {
//                 let hover_node_id = ctx.get_node_by_coords(ev.coords);

//                 if hover_node_id != Some(self.node_id) {
//                     ctx.nest_nodes(hover_node_id.unwrap(), self.node_id);
//                     StateDefault::new(ctx).into()
//                 } else {
//                     StateDefault::new(ctx).into()
//                 }
//                 //
//             }
//             UiEventKind::MouseMove(ev) => {
//                 // 1. above minimum threshold
//                 //
//                 // hover above another node
//                 //
//                 StateNodeMouseMove::new(ctx, self.node_id, self.coords, self.start_timestamp).into()
//             }
//             _ => todo!(),
//         }
//     }
// }

// // impl StateNodeMouseMoveMouseUp {
// //     pub fn new<T: Context>(
// //         ctx: &mut T,
// //         node_id: NodeId,
// //         coords: MouseCoords,
// //         timestamp: TimestampMs,
// //         end_node_id: Option<NodeId>,
// //     ) -> Self {
// //         Self {
// //             node_id,
// //             coords,
// //             start_timestamp: timestamp,
// //             end_node_id,
// //         }
// //     }

// //     fn on_timeout<T: Context>(self, ctx: &mut T, ev.timestamp: TimestampMs) -> UiState {
// //         panic!();
// //     }

// //     fn apply<T: Context>(
// //         self,
// //         ctx: &mut T,
// //         ev: UiEvent,
// //         ev.timestamp: TimestampMs,
// //     ) -> UiState {
// //         match ev.kind {
// //             UiEventKind::MouseDown(ev) => {
// //                 panic!();
// //             }
// //             UiEventKind::MouseUp(ev) => {
// //                 //
// //             }
// //             _ => todo!(),
// //         }
// //     }
// // }

// impl StateNodeMouseLongDown {
//     pub fn new<T: Context>(
//         ctx: &mut T,
//         node_id: NodeId,
//         coords: MouseCoords,
//         timestamp: UiEventTimestamp,
//     ) -> Self {
//         Self {
//             node_id,
//             coords,
//             start_timestamp: timestamp,
//         }
//     }
// }

// impl UiState for StateNodeMouseLongDown {
//     fn apply<T: Context>(self, ctx: &mut T, ev: UiEvent) -> UiStateKind {
//         match ev.kind {
//             UiEventKind::MouseDown(ev) => {
//                 panic!();
//             }
//             UiEventKind::MouseUp(ev) => StateDefault::new(ctx).into(),
//             UiEventKind::MouseMove(ev) => todo!(),
//             _ => todo!(),
//         }
//     }
// }

// impl StateNodeMouseClick {
//     pub fn new<T: Context>(
//         ctx: &mut T,
//         node_id: NodeId,
//         coords: MouseCoords,
//         start_timestamp: UiEventTimestamp,
//     ) -> Self {
//         Self {
//             node_id,
//             coords,
//             start_timestamp,
//             timeout: ctx.schedule_timeout(start_timestamp + MAX_DBG_CLICK_TIME_MS),
//         }
//     }
// }

// impl UiState for StateNodeMouseClick {
//     fn on_timeout<T: Context>(self, ctx: &mut T, timestamp: UiEventTimestamp) -> UiStateKind {
//         ctx.show_node_menu(self.node_id);
//         StateNodeMouseLongDown::new(ctx, self.node_id, self.coords, self.start_timestamp).into()
//     }

//     fn apply<T: Context>(self, ctx: &mut T, ev: UiEvent) -> UiStateKind {
//         match ev.kind {
//             UiEventKind::MouseDown(ev) => {
//                 StateNodeMouseClickDown::new(ctx, self.node_id, ev.coords, self.start_timestamp)
//                     .into()
//             }
//             UiEventKind::MouseUp(ev) => {
//                 panic!();
//             }
//             UiEventKind::MouseMove(ev) => todo!(),
//             _ => todo!(),
//         }
//     }
// }

// impl StateNodeMouseClickDown {
//     pub fn new<T: Context>(
//         ctx: &mut T,
//         node_id: NodeId,
//         coords: MouseCoords,
//         start_timestamp: UiEventTimestamp,
//     ) -> Self {
//         Self {
//             node_id,
//             coords,
//             start_timestamp,
//             timeout: ctx.schedule_timeout(start_timestamp + MAX_DBG_CLICK_TIME_MS),
//         }
//     }
// }

// impl UiState for StateNodeMouseClickDown {
//     fn on_timeout<T: Context>(self, ctx: &mut T, timestamp: UiEventTimestamp) -> UiStateKind {
//         ctx.show_node_menu(self.node_id);
//         StateNodeMouseLongDown::new(ctx, self.node_id, self.coords, self.start_timestamp).into()
//     }

//     fn apply<T: Context>(self, ctx: &mut T, ev: UiEvent) -> UiStateKind {
//         match ev.kind {
//             UiEventKind::MouseDown(ev) => {
//                 panic!();
//             }
//             UiEventKind::MouseUp(ev) => {
//                 ctx.rename_node(self.node_id);
//                 StateDefault::new(ctx).into()
//             }
//             UiEventKind::MouseMove(ev) => todo!(),
//             _ => todo!(),
//         }
//     }
// }

// impl StateEmptyMouseDown {
//     pub fn new<T: Context>(ctx: &mut T, coords: MouseCoords, timestamp: UiEventTimestamp) -> Self {
//         Self { coords, timestamp }
//     }
// }

// impl UiState for StateEmptyMouseDown {
//     fn apply<T: Context>(self, ctx: &mut T, ev: UiEvent) -> UiStateKind {
//         match ev.kind {
//             UiEventKind::MouseDown(ev) => {
//                 panic!();
//             }
//             UiEventKind::MouseUp(ev) => StateDefault::new(ctx).into(),
//             UiEventKind::MouseMove(ev) => todo!(),
//             _ => todo!(),
//         }
//     }
// }*/
// /*#[test]
// fn test() {
//     #[derive(Clone, Debug, Default)]
//     pub struct DummyContext {
//         pub timeout: Weak<UiEventTimestamp>,
//         pub selected_node_ids: Vec<NodeId>,
//         pub renamable_node_ids: Vec<NodeId>,
//         pub menu_node_ids: Vec<NodeId>,
//     }

//     fn apply(mut state: UiStateKind, ctx: &mut DummyContext, ev: UiEvent) -> UiStateKind {
//         while let Some(timeout) = ctx.timeout.upgrade() {
//             if *timeout <= ev.timestamp {
//                 ctx.timeout = Weak::new();
//                 state = state.on_timeout(ctx, *timeout);
//                 dbg!(&state);
//             } else {
//                 break;
//             }
//         }
//         state.apply(ctx, ev)
//     }

//     impl Context for DummyContext {
//         fn schedule_timeout(&mut self, timestamp: UiEventTimestamp) -> ScheduledTimeout {
//             let timeout = Arc::new(timestamp);
//             self.timeout = Arc::downgrade(&timeout);
//             timeout
//         }

//         fn get_node_by_coords(&mut self, coords: GraphCoords) -> Option<NodeId> {
//             if coords.0 > 200 {
//                 Some(2)
//             } else if coords.0 > 100 {
//                 Some(1)
//             } else {
//                 None
//             }
//         }

//         fn select_node(&mut self, node_id: NodeId) {
//             self.selected_node_ids.push(node_id);
//         }

//         fn rename_node(&mut self, node_id: NodeId) {
//             self.renamable_node_ids.push(node_id);
//         }

//         fn show_node_menu(&mut self, node_id: NodeId) {
//             self.menu_node_ids.push(node_id);
//         }

//         fn nest_nodes(&mut self, nested_node_id: NodeId, parent_node_id: NodeId) {
//             todo!()
//         }
//     }

//     let mut ctx = DummyContext::default();

//     let state = UiStateKind::default();
//     dbg!(&state);
//     let state = apply(
//         state,
//         &mut ctx,
//         UiEvent::new(
//             10000,
//             UiEventMouseDown {
//                 coords: (150, 50),
//                 button: MouseButton::Left,
//             }
//             .into(),
//         ),
//     );
//     dbg!(&state);
//     let state = apply(
//         state,
//         &mut ctx,
//         UiEvent::new(
//             10100,
//             UiEventMouseUp {
//                 coords: (150, 50),
//                 button: MouseButton::Left,
//             }
//             .into(),
//         ),
//     );
//     dbg!(&state);
//     let state = apply(
//         state,
//         &mut ctx,
//         UiEvent::new(
//             10200,
//             UiEventMouseDown {
//                 coords: (150, 50),
//                 button: MouseButton::Left,
//             }
//             .into(),
//         ),
//     );
//     dbg!(&state);
//     let state = apply(
//         state,
//         &mut ctx,
//         UiEvent::new(
//             10300,
//             UiEventMouseUp {
//                 coords: (150, 50),
//                 button: MouseButton::Left,
//             }
//             .into(),
//         ),
//     );
//     dbg!(&state);
//     assert_eq!(ctx.selected_node_ids, vec![1]);
//     assert_eq!(ctx.renamable_node_ids, vec![1]);
//     assert_eq!(ctx.menu_node_ids, vec![0; 0]);
// }*/
// /*fn apply_mouse_down(self, ctx: &mut T, ev: UiEventMouseDown, ev.timestamp: Timestamp) {
//     mst() {
//     let st = UiState::
// }atch self {
//                 x,
//                 y,
//                 button: MouseButton::Left,
//             },
//         ) => {
//             if let Some(node_id) = ctx.get_node_by_coords(x, y) {
//                 ctx.schedule_timeout(ev.timestamp + MAX_CLICK_TIME);
//                 UiState::NodeMouseDown {
//                     node_id,
//                     x,
//                     y,
//                     timestamp: ev.timestamp,
//                 }
//             } else {
//                 todo!();
//             }
//         }
//         (
//             UiState::NodeMouseDown {
//                 node_id,
//                 x,
//                 y,
//                 timestamp,
//             },
//             UiEventKind::MouseUp {
//                 x,
//                 y,
//                 button: MouseButton::Left,
//             },
//         ) => {
//             UiState::NodeMouseClick {
//                 node_id: node_id,
//                 timestamp: ev.timestamp,
//             }
//         }
//         (Default, UiEventKind::MouseUp { .. }) => UiState::Default,
//         _ => todo!(),
//     }
// }*/
// /*
//             -> mousedown -> mouseup -> mousedown -> mouseup
//      initial                select                  rename

//     click node
//         select
//     click empty
//         deselect

//     dblclick node
//         rename
//     dblclick empty
//         create node

//     right
//         menu

// */
// // click
// // down <300ms up

// //          Default
// //              |
// // down     NodeMouseDown           -> NodeMouseLongDown
// //              |
// // up       NodeMouseClick, click   -> Default
// //              |
// // down     NodeMouseClickDown      -> NodeMouseLongDown
// //              |
// // up       Default, dblclick
