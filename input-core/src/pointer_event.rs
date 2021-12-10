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

    pressed(De, St, Co)
    move(De, Co)
    released(De, St, Co)

    pointer-event

*/

#[derive(Clone, Debug)]
pub struct PointerState<De, St, Co, F> {
    devices: HashMap<De, DeviceState<St, Co>>,
    is_drag_fn: F,
}

#[derive(Clone, Debug)]
struct DeviceState<St, Co> {
    states: HashMap<St, StateState<Co>>,
}

#[derive(Clone, Debug)]
enum StateState<Co> {
    Changed(Co),
    Moving,
}

#[derive(Clone, Debug)]
pub struct PointerEventData<St, Ki> {
    pub state: St,
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

pub type PointerChangeEventData<St> = PointerEventData<St, PointerChangeEventKind>;
pub type PointerMoveEventData<St> = PointerEventData<St, PointerMoveEventKind>;

impl<De, St, Co, F> PointerState<De, St, Co, F>
where
    F: FnMut(&Co, &Co) -> bool,
{
    pub fn new(is_drag_fn: F) -> Self {
        Self {
            devices: HashMap::new(),
            is_drag_fn,
        }
    }

    pub fn with_change_event(
        self,
        device_id: De,
        state: St,
        coords: Co,
    ) -> (
        Self,
        Vec<PointerMoveEventData<St>>,
        Option<PointerChangeEventData<St>>,
    )
    where
        De: Eq + Hash,
        St: Clone + Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        let (sm_state, move_events) = self.with_move_event(&device_id, &coords);
        let is_drag_fn = sm_state.is_drag_fn;
        let mut devices = sm_state.devices;
        let device = devices.entry(device_id).or_default();
        let event = match device.states.entry(state.clone()) {
            Entry::Occupied(entry) => match entry.get() {
                StateState::Changed(_) => None,
                StateState::Moving => Some(PointerEventData {
                    state,
                    kind: PointerChangeEventKind::DragEnd,
                }),
            },
            Entry::Vacant(entry) => {
                let _ = entry.insert(StateState::Changed(coords));
                None
            }
        };
        (
            Self {
                devices,
                is_drag_fn,
            },
            move_events,
            event,
        )
    }

    pub fn with_move_event(
        self,
        device_id: &De,
        coords: &Co,
    ) -> (Self, Vec<PointerMoveEventData<St>>)
    where
        De: Eq + Hash,
        St: Clone + Eq + Hash,
    {
        let mut is_drag_fn = self.is_drag_fn;

        let mut devices = self.devices;
        let device = devices.get_mut(device_id);
        let events = match device {
            Some(device) => device
                .states
                .iter_mut()
                .filter_map(|(state, state_state)| match state_state {
                    StateState::Changed(prev_coords) => {
                        if is_drag_fn(prev_coords, coords) {
                            *state_state = StateState::Moving;
                            Some(PointerMoveEventData {
                                state: state.clone(),
                                kind: PointerMoveEventKind::DragStart,
                            })
                        } else {
                            None
                        }
                    }
                    StateState::Moving => Some(PointerMoveEventData {
                        state: state.clone(),
                        kind: PointerMoveEventKind::DragMove,
                    }),
                })
                .collect(),
            None => vec![],
        };

        (
            Self {
                devices,
                is_drag_fn,
            },
            events,
        )
    }
}

/*
impl<De, St, Co> Default for PointerState<De, St, Co> {
    fn default() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }
}*/

impl<St, Co> Default for DeviceState<St, Co> {
    fn default() -> Self {
        Self {
            states: HashMap::new(),
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
