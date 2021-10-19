use std::collections::{HashMap, HashSet};

use crate::{AxisKind, AxisValue, ButtonKind};

#[derive(Clone, Debug, Default)]
pub struct Modifiers {
    pub buttons: HashSet<ButtonKind>,
    pub axes: HashMap<AxisKind, AxisValue>,
}
