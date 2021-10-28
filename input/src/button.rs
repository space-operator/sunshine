use crate::{KeyboardKey, LongClickDuration, MouseButton, MultiClickDuration, TouchId};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ButtonKind {
    KeyboardKey(KeyboardKey),
    MouseButton(MouseButton),
    Touch(TouchId),
}

impl ButtonKind {
    #[must_use]
    pub const fn long_click_duration(&self) -> LongClickDuration {
        match self {
            ButtonKind::KeyboardKey(_) => LongClickDuration::Key,
            ButtonKind::MouseButton(_) => LongClickDuration::Mouse,
            ButtonKind::Touch(_) => LongClickDuration::Touch,
        }
    }

    #[must_use]
    pub const fn multi_click_duration(&self) -> MultiClickDuration {
        match self {
            ButtonKind::KeyboardKey(_) => MultiClickDuration::Key,
            ButtonKind::MouseButton(_) => MultiClickDuration::Mouse,
            ButtonKind::Touch(_) => MultiClickDuration::Touch,
        }
    }
}
