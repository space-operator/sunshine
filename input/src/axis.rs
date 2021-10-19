use crate::TouchId;

pub type AxisValue = i32;

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
    pub fn new(kind: AxisKind, value: Option<AxisValue>) -> Self {
        Self { kind, value }
    }
}
