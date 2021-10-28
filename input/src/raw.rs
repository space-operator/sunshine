use serde::{Deserialize, Serialize};

use crate::{Event, KeyboardKey, MouseButton, MouseScrollDelta, TimestampMs, TouchId};

pub type EventCoords = (i32, i32);
pub type RawEvent<T> = Event<RawInput<T>>;

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

impl<T> RawEvent<T> {
    pub const fn new(kind: RawInput<T>, timestamp: TimestampMs) -> Self {
        Self {
            input: kind,
            timestamp,
        }
    }
}
