// TODO: Add more keys
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum KeyboardKey {
    Escape,
    Tab,
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,
    Command,
    Space,
    Return,
    Backspace,
    Other(String),
}
