use core::fmt::Debug;
use core::hash::Hash;
use std::collections::HashMap;
/*
    TODO:
        when to call with_reset_click_count
        emit far-movement-while-pressed
        emit far-movement-while-released

    DragStartEvent   Pressed Released
    DragMoveEvent    Pressed Released
    DragEndEvent     Pressed Released

    pressed(De, Sw, Co)
    move(De, Co)
    released(De, Sw, Co)

    pointer-event

*/

#[derive(Clone, Debug)]
pub struct PointerState<De, Sw, Co> {
    devices: HashMap<De, DeviceState<Sw, Co>>,
}

#[derive(Clone, Debug)]
struct DeviceState<Sw, Co> {
    switches: HashMap<Sw, SwitchState<Co>>,
}

#[derive(Clone, Debug)]
enum SwitchState<Co> {
    Changed(Co),
    Moving,
}

#[derive(Clone, Debug)]
pub struct PointerEventData<Sw, Ki> {
    pub switch: Sw,
    pub kind: Ki,
}

#[derive(Clone, Copy, Debug)]
pub enum PointerChangeEventKind {
    DragEnd,
}

#[derive(Clone, Copy, Debug)]
pub enum PointerMoveEventKind {
    DragStart,
    DragMove,
}

pub type PointerChangeEventData<Sw> = PointerEventData<Sw, PointerChangeEventKind>;
pub type PointerMoveEventData<Sw> = PointerEventData<Sw, PointerMoveEventKind>;

impl<De, Sw, Co> PointerState<De, Sw, Co> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_change_event<F>(
        self,
        device_id: De,
        switch: Sw,
        coords: Co,
        is_drag_fn: F,
    ) -> (
        Self,
        Vec<PointerMoveEventData<Sw>>,
        Option<PointerChangeEventData<Sw>>,
        F,
    )
    where
        De: Eq + Hash,
        Sw: Clone + Eq + Hash,
        F: FnMut(&Co, &Co) -> bool,
    {
        use std::collections::hash_map::Entry;

        let (PointerState { mut devices }, move_events, is_drag_fn) =
            self.with_move_event(&device_id, &coords, is_drag_fn);
        let device = devices.entry(device_id).or_default();
        let event = match device.switches.entry(switch.clone()) {
            Entry::Occupied(entry) => match entry.get() {
                SwitchState::Changed(_) => None,
                SwitchState::Moving => Some(PointerEventData {
                    switch,
                    kind: PointerChangeEventKind::DragEnd,
                }),
            },
            Entry::Vacant(entry) => {
                let _ = entry.insert(SwitchState::Changed(coords));
                None
            }
        };
        (Self { devices }, move_events, event, is_drag_fn)
    }

    pub fn with_move_event<F>(
        self,
        device_id: &De,
        coords: &Co,
        mut is_drag_fn: F,
    ) -> (Self, Vec<PointerMoveEventData<Sw>>, F)
    where
        De: Eq + Hash,
        Sw: Clone + Eq + Hash,
        F: FnMut(&Co, &Co) -> bool,
    {
        let mut devices = self.devices;
        let device = devices.get_mut(device_id);
        let events = match device {
            Some(device) => device
                .switches
                .iter_mut()
                .filter_map(|(switch, state)| match state {
                    SwitchState::Changed(prev_coords) => {
                        if is_drag_fn(prev_coords, coords) {
                            *state = SwitchState::Moving;
                            Some(PointerMoveEventData {
                                switch: switch.clone(),
                                kind: PointerMoveEventKind::DragStart,
                            })
                        } else {
                            None
                        }
                    }
                    SwitchState::Moving => Some(PointerMoveEventData {
                        switch: switch.clone(),
                        kind: PointerMoveEventKind::DragMove,
                    }),
                })
                .collect(),
            None => vec![],
        };

        (Self { devices }, events, is_drag_fn)
    }
}

impl<De, Sw, Co> Default for PointerState<De, Sw, Co> {
    fn default() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }
}

impl<Sw, Co> Default for DeviceState<Sw, Co> {
    fn default() -> Self {
        Self {
            switches: HashMap::new(),
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
