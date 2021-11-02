use core::ops::RangeInclusive;
use std::collections::{HashMap, HashSet};

use crate::{AxisKind, AxisValue, ButtonKind};

pub type ModifiersButtons = HashSet<ButtonKind>;
pub type ModifiersAxes = HashMap<AxisKind, AxisValue>;

#[derive(Clone, Debug, Default)]
pub struct Modifiers {
    pub buttons: ModifiersButtons,
    pub axes: ModifiersAxes,
}

#[derive(Clone, Debug, Default)]
pub struct ModifiersFilter {
    pub buttons: HashSet<ButtonKind>,
    pub axes_ranges: HashMap<AxisKind, RangeInclusive<AxisValue>>,
}
