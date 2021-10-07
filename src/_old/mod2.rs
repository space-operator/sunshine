// use derive_more::From;
// use std::sync::Arc;

// use crate::ui_event::{KeyboardKey, UiEventCoords, UiEventTimeStampMs, UiModifiers};

// use super::context::Context;

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum PointerKind {
//     Mouse,
//     Touch,
//     Stylus,
//     InvertedStylus,
//     Unknown,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum Button {
//     Primary,
//     Secondary,
//     Tertiary,
// }

// // #[derive(Clone, Debug, Eq, PartialEq)]
// // pub struct LowLevelUiEvent {
// //     pub kind: LowLevelUiEventKind,
// //     pub timestamp: UiEventTimeStampMs,
// //     pub pointer_kind: PointerKind,
// // }

// pub enum PointerSignalKind {
//     None,
//     Scroll,
//     Unknown,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum LowLevelUiEventKind {
//     PointerDown(PointerDownEvent),
//     // PointerMove(PointerMoveEvent),
//     // PointerUp(PointerUpEvent),
//     // PointerHover(PointerHoverEvent),
//     // PointerCancel(PointerCancelEvent),
//     // PointerSignal(PointerSignalEvent),
//     // KeyDown(KeyDownEvent),
//     // KeyRepeat(KeyRepeatEvent),
//     // KeyUp(KeyUpEvent),
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct PointerDownEvent {
//     pub coords: UiEventCoords,
//     pub button: Button,
//     pub pointer_kind: PointerKind,
//     pub timestamp: UiEventTimeStampMs,
// }

// pub struct KeyDownEvent {
//     pub key: KeyboardKey,
// }

// #[derive(From, Clone, Debug, Eq, PartialEq)]
// pub enum UiState {
//     Default(UiDefaultState),
//     // PointerDownState(UiPointerDownState),
//     PointerDownMousePrimary,
//     PointerDownMouseSecondary,
//     PointerDownTouchPrimary,
//     PointerDownTouchSecondary,
//     PointerDownMouseMovePrimary,
//     PointerDownMouseMoveSecondary,
//     PointerUp,
//     PointerHover,
// }
// #[derive(Clone, Debug, Eq, PartialEq, Default)]
// pub struct UiDefaultState;

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiPointerDownState {
//     coords: UiEventCoords,
//     button: Button,
//     pointer_kind: PointerKind,
// }

// pub trait UiStateMachine: Sized {
//     fn with_timeout<'a, T: Context>(self, data: UiStateWithTimeoutData<'a, T>) -> UiState {
//         panic!("state should not be called by timeout");
//     }

//     fn with_event<'a, T: Context>(self, data: UiStateWithEventData<'a, T>) -> UiState;
// }
// pub struct UiStateWithEventData<'a, T: Context> {
//     pub ctx: &'a mut T,
//     pub ev: LowLevelUiEventKind,
//     pub timestamp: UiEventTimeStampMs,
//     pub modifiers: &'a Arc<UiModifiers>,
// }

// pub struct UiStateWithTimeoutData<'a, T: Context> {
//     pub ctx: &'a mut T,
//     pub timestamp: UiEventTimeStampMs,
//     pub modifiers: &'a Arc<UiModifiers>,
// }

// impl UiStateMachine for UiDefaultState {
//     fn with_timeout<'a, T: Context>(self, data: UiStateWithTimeoutData<'a, T>) -> UiState {
//         panic!("state should not be called by timeout");
//     }

//     fn with_event<'a, T: Context>(self, data: UiStateWithEventData<'a, T>) -> UiState {
//         match data.ev {
//             LowLevelUiEventKind::PointerDown(ev) => match ev.pointer_kind {
//                 PointerKind::Mouse => match ev.button {
//                     Button::Primary => todo!(),
//                     Button::Secondary => todo!(),
//                     Button::Tertiary => todo!(),
//                 },
//                 PointerKind::Touch => todo!(),
//                 PointerKind::Stylus => todo!(),
//                 PointerKind::InvertedStylus => todo!(),
//                 PointerKind::Unknown => todo!(),
//             },
//         }
//     }
// }

// impl UiPointerDownState {
//     fn new<T: Context>(
//         ctx: &mut T,
//         coords: UiEventCoords,
//         button: Button,
//         pointer_kind: PointerKind,
//         timestamp: UiEventTimeStampMs,
//     ) -> Self {
//         Self {
//             coords,
//             button,
//             pointer_kind,
//         }
//     }
// }

// impl UiStateMachine for UiPointerDownState {
//     fn with_event<'a, T: Context>(self, data: UiStateWithEventData<'a, T>) -> UiState {
//         match data.ev {
//             LowLevelUiEventKind::PointerDown(_) => todo!(),
//         }
//     }

//     fn with_timeout<'a, T: Context>(self, data: UiStateWithTimeoutData<'a, T>) -> UiState {
//         panic!("state should not be called by timeout");
//     }
// }
