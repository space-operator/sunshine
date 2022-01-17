use crate::{KeyboardKey, MouseButton, TouchId};

#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ButtonKind {
    KeyboardKey(KeyboardKey),
    MouseButton(MouseButton),
    Touch(TouchId),
}
