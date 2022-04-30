/// An event that is caused by enabling/pressing or disabling/releasing
/// some switch such as keyboard or mouse button.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SwitchEvent<Ti, Sw> {
    pub time: Ti,
    pub switch: Sw,
}

/// An event that is caused by some trigger event
/// such as mouse scroll down or up.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TriggerEvent<Ti, Tr> {
    pub time: Ti,
    pub trigger: Tr,
}

/// An event that is caused by device movement.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CoordsEvent<Ti, Co> {
    pub time: Ti,
    pub coords: Co,
}

impl<Ti, Sw> SwitchEvent<Ti, Sw> {
    /// Constructs a `SwitchEvent` from a given event time and switch.
    pub fn new(time: Ti, switch: Sw) -> Self {
        Self { time, switch }
    }
}

impl<Ti, Tr> TriggerEvent<Ti, Tr> {
    /// Constructs a `SwitchEvent` from a given event time and trigger.
    pub fn new(time: Ti, trigger: Tr) -> Self {
        Self { time, trigger }
    }
}

impl<Ti, Co> CoordsEvent<Ti, Co> {
    /// Constructs a `SwitchEvent` from a given event time and coordinates.
    pub fn new(time: Ti, coords: Co) -> Self {
        Self { time, coords }
    }
}
