#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SwitchEvent<Ti, Sw, Co, Mo, Td> {
    time: Ti,
    switch: Sw,
    coords: Co,
    modifiers: Mo,
    timed_data: Td,
}

impl<Ti, Sw, Co> SwitchEvent<Ti, Sw, Co, (), ()> {
    pub fn new(time: Ti, switch: Sw, coords: Co) -> Self {
        SwitchEvent {
            time: time,
            switch: switch,
            coords: coords,
            modifiers: (),
            timed_data: (),
        }
    }
}

impl<Ti, Sw, Co, Td> SwitchEvent<Ti, Sw, Co, (), Td> {
    pub fn with_modifiers<Mo>(self, modifiers: Mo) -> SwitchEvent<Ti, Sw, Co, Mo, Td> {
        SwitchEvent {
            time: self.time,
            switch: self.switch,
            coords: self.coords,
            modifiers,
            timed_data: self.timed_data,
        }
    }
}

impl<Ti, Sw, Co, Mo> SwitchEvent<Ti, Sw, Co, Mo, ()> {
    pub fn with_timed_data<Td>(self, timed_data: Td) -> SwitchEvent<Ti, Sw, Co, Mo, Td> {
        SwitchEvent {
            time: self.time,
            switch: self.switch,
            coords: self.coords,
            modifiers: self.modifiers,
            timed_data,
        }
    }
}
