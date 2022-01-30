use core::fmt::Debug;
use core::hash::Hash;
use std::collections::HashMap;

use thiserror::Error;

#[derive(Clone, Debug)]
pub struct PointerState<Sw, Co> {
    switches: HashMap<Sw, SwitchState<Co>>,
}

#[derive(Clone, Debug)]
enum SwitchState<Co> {
    Pressed(Co),
    Moving,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PointerMoveEventKind {
    DragStart,
    DragMove,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PointerChangeEventData {
    DragEnd,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PointerMoveEventData<Sw> {
    pub switch: Sw,
    pub kind: PointerMoveEventKind,
}

impl<Sw, Co> PointerState<Sw, Co> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_press_event(self, switch: Sw, coords: Co) -> (Self, Result<(), PointerPressError>)
    where
        Sw: Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        let mut switches = self.switches;
        match switches.entry(switch) {
            Entry::Occupied(_) => (Self { switches }, Err(PointerPressError::AlreadyPressed)),
            Entry::Vacant(entry) => {
                let _ = entry.insert(SwitchState::Pressed(coords));
                (Self { switches }, Ok(()))
            }
        }
    }

    pub fn with_release_event(
        self,
        switch: &Sw,
    ) -> (
        Self,
        Result<Option<PointerChangeEventData>, PointerReleaseError>,
    )
    where
        Sw: Eq + Hash,
    {
        let mut switches = self.switches;
        match switches.remove(switch) {
            Some(state) => {
                let data = match state {
                    SwitchState::Pressed(_) => None,
                    SwitchState::Moving => Some(PointerChangeEventData::DragEnd),
                };
                (Self { switches }, Ok(data))
            }
            None => (Self { switches }, Err(PointerReleaseError::AlreadyReleased)),
        }
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
                SwitchState::Pressed(coords) => {
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

#[derive(Clone, Copy, Debug, Error)]
pub enum PointerPressError {
    #[error("Button is pressed while in Pressed state")]
    AlreadyPressed,
}

#[derive(Clone, Copy, Debug, Error)]
pub enum PointerReleaseError {
    #[error("Button is released while in Released state")]
    AlreadyReleased,
}
