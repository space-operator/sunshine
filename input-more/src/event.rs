#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SwitchEvent<Ti, Sw> {
    pub time: Ti,
    pub switch: Sw,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TriggerEvent<Ti, Tr> {
    pub time: Ti,
    pub trigger: Tr,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CoordsEvent<Ti, Co> {
    pub time: Ti,
    pub coords: Co,
}

impl<Ti, Sw> SwitchEvent<Ti, Sw> {
    pub fn new(time: Ti, switch: Sw) -> Self {
        Self { time, switch }
    }
}

impl<Ti, Tr> TriggerEvent<Ti, Tr> {
    pub fn new(time: Ti, trigger: Tr) -> Self {
        Self { time, trigger }
    }
}

impl<Ti, Co> CoordsEvent<Ti, Co> {
    pub fn new(time: Ti, coords: Co) -> Self {
        Self { time, coords }
    }
}
