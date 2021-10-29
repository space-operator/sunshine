use serde::{Deserialize, Serialize};

use crate::{KeyboardKey, MouseButton, MouseScrollDelta, TouchId};

pub type EventCoords = (i32, i32);

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
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
