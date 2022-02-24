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

    pub fn on_press_event(&mut self, switch: Sw, coords: Co) -> Result<(), PointerPressError>
    where
        Sw: Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        match self.switches.entry(switch) {
            Entry::Occupied(_) => Err(PointerPressError::AlreadyPressed),
            Entry::Vacant(entry) => {
                let _ = entry.insert(SwitchState::Pressed(coords));
                Ok(())
            }
        }
    }

    pub fn on_release_event(
        &mut self,
        switch: &Sw,
    ) -> Result<Option<PointerChangeEventData>, PointerReleaseError>
    where
        Sw: Eq + Hash,
    {
        match self.switches.remove(switch) {
            Some(state) => {
                let data = match state {
                    SwitchState::Pressed(_) => None,
                    SwitchState::Moving => Some(PointerChangeEventData::DragEnd),
                };
                Ok(data)
            }
            None => Err(PointerReleaseError::AlreadyReleased),
        }
    }

    pub fn on_move_event<F>(&mut self, mut is_dragged_fn: F) -> Vec<PointerMoveEventData<Sw>>
    where
        Sw: Clone + Eq + Hash,
        F: FnMut(&Co) -> bool,
    {
        let events = self
            .switches
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

        events
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
