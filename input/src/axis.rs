use crate::TouchId;

pub type AxisValue = i32;

#[allow(missing_copy_implementations)] // TODO
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AxisKind {
    MouseX,
    MouseY,
    TouchX(TouchId),
    TouchY(TouchId),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Axis {
    pub kind: AxisKind,
    pub value: Option<AxisValue>,
}

impl Axis {
    #[must_use]
    pub const fn new(kind: AxisKind, value: Option<AxisValue>) -> Self {
        Self { kind, value }
    }
}
