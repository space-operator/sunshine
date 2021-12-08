#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MouseButton {
    Primary,
    Secondary,
    Auxiliary,
    Other(u32),
}

pub type MouseScrollDelta = i32;
