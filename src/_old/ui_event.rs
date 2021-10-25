use derive_more::From;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

pub type UiEventTimeStampMs = u64;
pub type UiEventTimeDeltaMs = u64;

pub type UiEventCoords = (i32, i32);

pub type TouchId = u32;
pub type NumMouseClicks = u32;
pub type MouseWheelDelta = i32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventStartEndCoords {
    pub start: UiEventCoords,
    pub end: UiEventCoords,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KeyboardKey {
    Space,
    Escape,
    Enter,
    Backspace,
    Delete,
    Ctrl,
    Shift,
    Alt,
    A,
    S,
    Z,
    X,
    C,
    V,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiModifier {
    KeyboardKey(KeyboardKey),
}

pub type UiModifiers = HashSet<UiModifier>;

#[derive(Clone, Debug, Eq, From, PartialEq)]
pub struct LowLevelUiEvent {
    pub kind: LowLevelUiEventKind,
    pub timestamp: UiEventTimeStampMs,
}

#[derive(Clone, Debug, From)]
pub struct UiEvent {
    pub kind: UiEventKind,
    pub modifiers: Arc<UiModifiers>,
    pub timestamp: UiEventTimeStampMs,
}

#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum LowLevelUiEventKind {
    MouseDown(UiMouseDownEvent),
    MouseUp(UiMouseUpEvent),
    MouseMove(UiMouseMoveEvent),
    MouseWheel(UiMouseWheelEvent),
    TouchStart(UiTouchStartEvent),
    TouchEnd(UiTouchEndEvent),
    TouchMove(UiTouchMoveEvent),
    KeyDown(UiKeyDownEvent),
    KeyUp(UiKeyUpEvent),
    Char(UiCharEvent),
}

#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum UiEventKind {
    /*
    if recv KeyDown/ButtonDown/TouchStart
        if RecvEvent in MaybeMultiClick
            emit     CurrentModifiers + RecvEvent down (num_clicks)
            remember num_click
            remove   RecvEvent from MaybeMultiClick
        else
            emit     CurrentModifiers + RecvEvent down
        endif
        add      RecvEvent to CurrentModifiers
        remember time of the RecvEvent to MaybeLongPressed
        add      RecvEvent to ClickCandidates
        remember coords of the RecvEvent to MaybeClick
        schedule LongPressedCheck for RecvEvent

    if recv KeyUp/ButtonUp/TouchEnd
        emit     CurrentModifiers + RecvEvent up (num_clicks)
        clear    LongPressedCheck for RecvEvent
        add      RecvEvent to MaybeMultiClick
        schedule NotAMultiClick for RecvEvent
        if       RecvEvent in ClickCandidates
            emit    SavedModifiers + RecvEvent click or multiclick (num_clicks)

    on LongPressedCheck
        emit    SavedModifiers + RecvEvent long pressed
        remove  RecvEvent from MaybeLongPressed

    on NotAMultiClick
        emit    SavedModifiers + NotAClick
        remove  RecvEvent from MaybeMultiClick

    if recv MouseWheel/Char
        emit     CurrentModifiers + RecvEvent

    if recv MouseMove/TouchMove
        check coords of all the RecvEvent
            remove  RecvEvent from MaybeClick






     */
    MouseClick(UiMouseClickEvent),
    MouseClickExact(UiMouseClickExactEvent),
    MouseMove(UiMouseMoveEvent),
    MouseDragMaybeStart(UiMouseDragMaybeStartEvent),
    MouseDragStart(UiMouseDragStartEvent),
    MouseDragging(UiMouseDraggingEvent),
    MouseDrop(UiMouseDropEvent),
    MouseWheel(UiMouseWheelEvent),
    TouchClick(UiTouchClickEvent),
    TouchClickExact(UiTouchClickExactEvent),
    TouchMoving(UiTouchMovingEvent),
    TouchMoveEnd(UiTouchMoveEndEvent),
    Key(UiKeyEvent),
    Char(UiCharEvent),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMouseDownEvent {
    pub coords: UiEventCoords,
    pub button: MouseButton,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMouseUpEvent {
    pub coords: UiEventCoords,
    pub button: MouseButton,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMouseMoveEvent {
    pub coords: UiEventCoords,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMouseWheelEvent {
    pub coords: UiEventCoords,
    pub delta: MouseWheelDelta,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiTouchStartEvent {
    pub coords: UiEventCoords,
    pub touch_id: TouchId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiTouchEndEvent {
    pub coords: UiEventCoords,
    pub touch_id: TouchId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiTouchMoveEvent {
    pub coords: UiEventCoords,
    pub touch_id: TouchId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiKeyDownEvent {
    pub key: KeyboardKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiKeyUpEvent {
    pub key: KeyboardKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiKeyEvent {
    pub key: KeyboardKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCharEvent {
    pub ch: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMouseClickEvent {
    coords: UiEventCoords,
    button: MouseButton,
    clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMouseClickExactEvent {
    coords: UiEventCoords,
    button: MouseButton,
    clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMouseDragMaybeStartEvent {
    coords: UiEventStartEndCoords,
    button: MouseButton,
    clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMouseDragStartEvent {
    coords: UiEventStartEndCoords,
    button: MouseButton,
    clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMouseDraggingEvent {
    coords: UiEventStartEndCoords,
    button: MouseButton,
    clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMouseDropEvent {
    coords: UiEventStartEndCoords,
    button: MouseButton,
    clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiTouchClickEvent {
    coords: UiEventCoords,
    is_long: bool,
    clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiTouchClickExactEvent {
    coords: UiEventCoords,
    is_long: bool,
    clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiTouchMovingEvent {
    coords: Vec<UiEventStartEndCoords>,
    is_long: bool,
    clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiTouchMoveEndEvent {
    coords: Vec<UiEventStartEndCoords>,
    is_long: bool,
    clicks: NumMouseClicks,
}

impl LowLevelUiEvent {
    pub fn new(kind: LowLevelUiEventKind, timestamp: UiEventTimeStampMs) -> Self {
        Self { timestamp, kind }
    }
}

impl UiEvent {
    pub fn new(
        kind: UiEventKind,
        modifiers: Arc<UiModifiers>,
        timestamp: UiEventTimeStampMs,
    ) -> Self {
        Self {
            timestamp,
            modifiers,
            kind,
        }
    }
}
