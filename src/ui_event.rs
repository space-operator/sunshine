// use derive_more::From;
// use serde::{Deserialize, Serialize};
// use std::collections::HashSet;

// pub type UiEventTimeStampMs = u64;
// pub type UiEventTimeDeltaMs = u64;

// pub type UiEventCoords = (i32, i32);

// pub type TouchId = u32;
// pub type NumMouseClick = u32;
// pub type MouseWheelDelta = i32;

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiEventStartEndCoords {
//     pub start: UiEventCoords,
//     pub end: UiEventCoords,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum MouseButton {
//     Left,
//     Middle,
//     Right,
//     Other(u32),
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum KeyboardKey {
//     Space,
//     Escape,
//     Enter,
//     Backspace,
//     Delete,
//     Ctrl,
//     Shift,
//     Alt,
//     A,
//     S,
//     Z,
//     X,
//     C,
//     V,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum Modifier {
//     KeyboardKey(KeyboardKey),
// }

// pub type Modifiers = HashSet<Modifiers>;

// #[derive(Clone, Debug, Eq, From, PartialEq)]
// pub struct LowLevelUiEvent {
//     pub timestamp: UiEventTimeStampMs,
//     pub kind: LowLevelUiEventKind,
// }

// #[derive(Clone, Debug, Eq, From, PartialEq)]
// pub struct HighLevelUiEvent {
//     pub timestamp: UiEventTimeStampMs,
//     pub modifiers: Modifiers,
//     pub kind: HighLevelUiEventKind,
// }

// #[derive(Clone, Debug, Eq, From, PartialEq)]
// pub enum LowLevelUiEventKind {
//     MouseDown(UiMouseDownEvent),
//     MouseUp(UiMouseUpEvent),
//     MouseMove(UiMouseMoveEvent),
//     MouseWheel(UiMouseWheelEvent),
//     TouchStart(UiTouchStartEvent),
//     TouchEnd(UiTouchEndEvent),
//     TouchMove(UiTouchMoveEvent),
//     KeyDown(UiKeyDownEvent),
//     KeyUp(UiKeyUpEvent),
//     Char(UiCharEvent),
// }

// #[derive(Clone, Debug, Eq, From, PartialEq)]
// pub enum HighLevelUiEventKind {
//     MouseMove(UiMouseMoveEvent),
//     MouseDragMaybeStart(UiMouseDragEvent),
//     MouseDragStart(UiMouseDragEvent),
//     MouseDragging(UiMouseDragEvent),
//     MouseDrop(UiMouseDropEvent),
//     MouseClick(UiMouseClickEvent),
//     MouseClickExact(UiMouseClickExactEvent),
//     MouseWheel(UiMouseWheelEvent),
//     TouchMoving(UiTouchMoveEvent),
//     TouchMoveEnd(UiTouchMoveEvent),
//     TouchClick(UiTouchClickEvent),
//     TouchClickExact(UiTouchClickExactEvent),
//     Key(UiEventKey),
//     Char(UiCharEvent),
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiMouseDownEvent {
//     pub coords: UiEventCoords,
//     pub button: MouseButton,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiMouseUpEvent {
//     pub coords: UiEventCoords,
//     pub button: MouseButton,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiMouseMoveEvent {
//     pub coords: UiEventCoords,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiMouseWheelEvent {
//     pub coords: UiEventCoords,
//     pub delta: MouseWheelDelta,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiTouchStartEvent {
//     pub coords: UiEventCoords,
//     pub touch_id: TouchId,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiTouchEndEvent {
//     pub coords: UiEventCoords,
//     pub touch_id: TouchId,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiTouchMoveEvent {
//     pub coords: UiEventCoords,
//     pub touch_id: TouchId,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiKeyDownEvent {
//     pub key: KeyboardKey,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiKeyUpEvent {
//     pub key: KeyboardKey,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiKeyEvent {
//     pub key: KeyboardKey,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiCharEvent {
//     pub ch: String,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiMouseDragEvent {
//     coords: UiEventStartEndCoords,
//     button: MouseButton,
//     is_long: bool,
//     clicks: NumMouseClick,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiMouseDropEvent {
//     coords: UiEventStartEndCoords,
//     button: MouseButton,
//     is_long: bool,
//     clicks: NumMouseClick,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiMouseClickEvent {
//     coords: UiEventCoords,
//     button: MouseButton,
//     clicks: NumMouseClick,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiMouseClickExactEvent {
//     coords: UiEventCoords,
//     button: MouseButton,
//     is_long: bool,
//     clicks: NumMouseClick,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiTouchMoveEvent {
//     coords: Vec<UiEventStartEndCoords>,
//     is_long: bool,
//     clicks: NumMouseClick,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiTouchClickEvent {
//     coords: UiEventCoords,
//     is_long: bool,
//     clicks: NumMouseClick,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UiTouchClickExactEvent {
//     coords: UiEventCoords,
//     is_long: bool,
//     clicks: NumMouseClick,
// }

// impl LowLevelUiEvent {
//     pub fn new(timestamp: UiEventTimeStampMs, kind: LowLevelUiEventKind) -> Self {
//         Self { timestamp, kind }
//     }
// }

// impl HighLevelUiEvent {
//     pub fn new(timestamp: UiEventTimeStampMs, kind: HighLevelUiEventKind) -> Self {
//         Self { timestamp, kind }
//     }
// }
