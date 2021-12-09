use std::collections::HashMap;

/*
    TODO:
        when to call with_reset_click_count
        emit far-movement-while-pressed
        emit far-movement-while-released
*/

// touch 1 start at (100, 100)         | #100 -> #100
// touch 1 end at (100, 100)           |
// touch 1 start at (200, 200) -> #2   | #101 -> #101
// touch 2 start at (100, 100) -> #1   | #102 -> #100
// provide temp ids that can be used

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct SystemId<T>(T);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct LocalId<T>(T);

#[derive(Clone, Debug)]
pub struct PointerState<Id, Sw, Co> {
    devices: HashMap<SystemId<Id>, DeviceState<Id, Sw, Co>>,
    last_coords: HashMap<LocalId<Id>, Co>,
}

#[derive(Clone, Debug)]
struct DeviceState<Id, Sw, Co> {
    local_id: LocalId<Id>,
    switches: HashMap<Sw, SwitchState<Co>>,
}

#[derive(Clone, Debug)]
enum SwitchState<Co> {
    Pressed(Co),
    PressedAndMoved,
    Released(Co),
    ReleasedAndMoved,
}

impl<Id, Sw, Co> PointerState<Id, Sw, Co> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_press_event(self, device_id: Id, switch: Sw, coords: Co) -> Self {
        todo!()
    }
    pub fn with_release_event(self, device_id: Id, switch: Sw, coords: Co) -> Self {
        todo!()
    }
    pub fn with_move_event(self, device_id: Id, coords: Co) -> Self {
        todo!()
    }
}

impl<Id, Sw, Co> Default for PointerState<Id, Sw, Co> {
    fn default() -> Self {
        Self {
            devices: HashMap::new(),
            last_coords: HashMap::new(),
        }
    }
}

/*
    space-down
    mouse-move
    space-up
    mouse-move
    space-down
    mouse-move
    space-up
        space-dbl-click

    Device -> PositionSincePressed | MovedWhilePressed | PositionSinceReleased | MovedWhileReleased | CachedPosition

*/

// Mouse
// Mouse while LMB
// Mouse while RMB
// Mouse while LMB+RMB
// Touch
