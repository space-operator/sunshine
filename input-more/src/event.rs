#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SwitchEvent<Ti, Sw, Co> {
    pub time: Ti,
    pub switch: Sw,
    pub coords: Co,
}

impl<Ti, Sw, Co> SwitchEvent<Ti, Sw, Co> {
    pub fn new(time: Ti, switch: Sw, coords: Co) -> Self {
        SwitchEvent {
            time: time,
            switch: switch,
            coords: coords,
        }
    }
}
