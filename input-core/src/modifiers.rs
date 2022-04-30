use core::borrow::Borrow;
use core::hash::Hash;
use std::collections::BTreeSet;
use std::sync::Arc;

use thiserror::Error;

/// A structure that stores pressed/enabled switches.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Modifiers<Sw> {
    switches: Arc<BTreeSet<Sw>>,
}

impl<Sw> Modifiers<Sw> {
    /// Constructs a new, empty `Modifiers` structure.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the contained modifiers set.
    pub fn switches(&self) -> &Arc<BTreeSet<Sw>> {
        &self.switches
    }

    /// Converts `Modifiers` structure into the contained modifiers set.
    pub fn into_switches(self) -> Arc<BTreeSet<Sw>> {
        self.switches
    }

    /// Sets the switch pressed/enabled.
    ///
    /// It is assumed that the application or other library itself calls this function
    /// when the user presses a button on the keyboard, mouse, or other device.
    pub fn on_press_event(&mut self, switch: Sw) -> Result<(), ModifiersPressError>
    where
        Sw: Clone + Eq + Hash + Ord,
    {
        let switches = Arc::make_mut(&mut self.switches);
        let is_added = switches.insert(switch);

        if is_added {
            Ok(())
        } else {
            Err(ModifiersPressError::AlreadyPressed)
        }
    }

    /// Sets the switch released/disabled.
    ///
    /// It is assumed that the application or other library itself calls this function
    /// when the user releases a button on the keyboard, mouse, or other device.
    pub fn on_release_event(&mut self, switch: &Sw) -> Result<(), ModifiersReleaseError>
    where
        Sw: Clone + Eq + Hash + Ord,
    {
        let switches = Arc::make_mut(&mut self.switches);
        let is_removed = switches.remove(switch.borrow());
        if is_removed {
            Ok(())
        } else {
            Err(ModifiersReleaseError::AlreadyReleased)
        }
    }
}

impl<Sw> Default for Modifiers<Sw> {
    fn default() -> Self {
        Self {
            switches: Arc::new(BTreeSet::new()),
        }
    }
}

impl<Sw> From<Arc<BTreeSet<Sw>>> for Modifiers<Sw> {
    fn from(switches: Arc<BTreeSet<Sw>>) -> Self {
        Self { switches }
    }
}

/// The error type which is returned from `Modifiers::on_release_event`.
#[derive(Clone, Copy, Debug, Error)]
pub enum ModifiersPressError {
    #[error("Button is pressed while in Pressed state")]
    AlreadyPressed,
}

/// The error type which is returned from `Modifiers::on_press_event`.
#[derive(Clone, Copy, Debug, Error)]
pub enum ModifiersReleaseError {
    #[error("Button is released while in Released state")]
    AlreadyReleased,
}
