use crate::{KeyboardKey, MouseButton, MouseScrollDelta, TouchId};

pub type EventCoord = i32;
pub type EventCoords = (EventCoord, EventCoord);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawInput<T> {
    KeyDown(KeyboardKey),
    KeyUp(KeyboardKey),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseMove(EventCoords),
    MouseWheelDown,
    MouseWheelUp,
    MouseScroll(MouseScrollDelta),
    TouchStart {
        touch_id: TouchId,
        coords: EventCoords,
    },
    TouchEnd {
        touch_id: TouchId,
    },
    TouchMove {
        touch_id: TouchId,
        coords: EventCoords,
    },
    Char(String),
    Custom(T),
}
