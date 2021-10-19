use crate::{Event, KeyboardKey, MouseButton, MouseScrollDelta, TouchId};

pub type EventCoords = (i32, i32);
pub type RawEvent = Event<RawInput>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawInput {
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
}
