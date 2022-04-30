use core::fmt::Debug;
use core::hash::Hash;
use std::collections::HashMap;

use thiserror::Error;

/// A structure that stores drag-and-drop-related input state.
#[derive(Clone, Debug)]
pub struct PointerState<Sw, Co> {
    switches: HashMap<Sw, SwitchState<Co>>,
}

/// A enumeration that specifies drag-and-drop-related input state for switch.
#[derive(Clone, Debug)]
enum SwitchState<Co> {
    Pressed(Co),
    Moving,
}

/// A enumeration that specifies switch pointer move event kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PointerMoveEventKind {
    DragStart,
    DragMove,
}

/// A enumeration that specifies switch pointer release event kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PointerReleaseEventData {
    DragEnd,
}

/// A enumeration that specifies switch pointer move event kind.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PointerMoveEventData<Sw> {
    pub switch: Sw,
    pub kind: PointerMoveEventKind,
}

impl<Sw, Co> PointerState<Sw, Co> {
    /// Constructs a new, empty `PointerState` structure.
    pub fn new() -> Self {
        Self::default()
    }

    /// The callback to be called once the switch has been pressed/activated.
    ///
    /// It is assumed that the application or other library itself calls this function
    /// when the user presses a button on the keyboard, mouse, or other device.
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

    /// The callback to be called once the switch has been released/disabled.
    ///
    /// It is assumed that the application or other library itself calls this function
    /// when the user releases a button on the keyboard, mouse, or other device.
    ///
    /// This function optionally returns `PointerReleaseEventData`
    /// if drop event has been recognized.
    pub fn on_release_event(
        &mut self,
        switch: &Sw,
    ) -> Result<Option<PointerReleaseEventData>, PointerReleaseError>
    where
        Sw: Eq + Hash,
    {
        match self.switches.remove(switch) {
            Some(state) => {
                let data = match state {
                    SwitchState::Pressed(_) => None,
                    SwitchState::Moving => Some(PointerReleaseEventData::DragEnd),
                };
                Ok(data)
            }
            None => Err(PointerReleaseError::AlreadyReleased),
        }
    }

    /// The callback to be called once the device has been moved.
    ///
    /// It is assumed that the application or other library itself calls this function
    /// when the user releases a button on the keyboard, mouse, or other device.
    ///
    /// This function returns a vector of `PointerMoveEventData` events
    /// for all switches with drag event recognized.
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

/// The error type which is returned from `PointerState::on_press_event`.
#[derive(Clone, Copy, Debug, Error)]
pub enum PointerPressError {
    #[error("Button is pressed while in Pressed state")]
    AlreadyPressed,
}

/// The error type which is returned from `PointerState::on_release_event`.
#[derive(Clone, Copy, Debug, Error)]
pub enum PointerReleaseError {
    #[error("Button is released while in Released state")]
    AlreadyReleased,
}
