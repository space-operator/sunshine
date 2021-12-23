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

    pressed(Sw, Co)
    move(Co)
    released(Sw, Co)

    pointer-event


*/

#[derive(Clone, Debug)]
pub struct PointerState<Sw, Co> {
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

impl<Sw, Co> PointerState<Sw, Co> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_change_event(
        self,
        switch: Sw,
        coords: Co,
    ) -> (Self, Option<PointerChangeEventData<Sw>>)
    where
        Sw: Clone + Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        let mut switches = self.switches;
        let event = match switches.entry(switch.clone()) {
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
        (Self { switches }, event)
    }

    pub fn with_move_event<F>(
        self,
        mut is_dragged_fn: F,
    ) -> (Self, (Vec<PointerMoveEventData<Sw>>, F))
    where
        Sw: Clone + Eq + Hash,
        F: FnMut(&Co) -> bool,
    {
        let mut switches = self.switches;

        let events = switches
            .iter_mut()
            .filter_map(|(switch, state)| match state {
                SwitchState::Changed(coords) => {
                    if is_dragged_fn(coords) {
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
            .collect();

        (Self { switches }, (events, is_dragged_fn))
    }
}

impl<Sw, Co> Default for PointerState<Sw, Co> {
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
