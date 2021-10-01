use derive_more::From;
use serde::{Deserialize, Serialize};

pub type MouseCoords = (i32, i32);
pub type MouseWheelDelta = i32;
pub type TouchId = u32;

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

#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum UiEvent {
    MouseDown(UiEventMouseDown),
    MouseUp(UiEventMouseUp),
    MouseMove(UiEventMouseMove),
    MouseWheelDelta(UiEventMouseWheelDelta),
    TouchStart(UiEventTouchStart),
    TouchEnd(UiEventTouchEnd),
    TouchMove(UiEventTouchMove),
    KeyDown(UiEventKeyDown),
    KeyUp(UiEventKeyUp),
    Char(UiEventChar),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventMouseDown {
    pub coords: MouseCoords,
    pub button: MouseButton,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventMouseUp {
    pub coords: MouseCoords,
    pub button: MouseButton,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventMouseMove {
    pub coords: MouseCoords,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventMouseWheelDelta {
    pub coords: MouseCoords,
    pub delta: MouseWheelDelta,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventTouchStart {
    pub coords: MouseCoords,
    pub touch_id: TouchId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventTouchEnd {
    pub coords: MouseCoords,
    pub touch_id: TouchId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventTouchMove {
    pub coords: MouseCoords,
    pub touch_id: TouchId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventKeyDown {
    pub key: KeyboardKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventKeyUp {
    pub key: KeyboardKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEventChar {
    pub ch: String,
}
