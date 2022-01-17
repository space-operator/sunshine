use core::borrow::Borrow;
use core::hash::Hash;
use std::collections::BTreeSet;
use std::sync::Arc;

use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Modifiers<Sw> {
    switches: Arc<BTreeSet<Sw>>,
}

impl<Sw> Modifiers<Sw> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn switches(&self) -> &Arc<BTreeSet<Sw>> {
        &self.switches
    }

    pub fn into_switches(self) -> Arc<BTreeSet<Sw>> {
        self.switches
    }

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

#[derive(Clone, Copy, Debug, Error)]
pub enum ModifiersPressError {
    #[error("Button is pressed while in Pressed state")]
    AlreadyPressed,
}

#[derive(Clone, Copy, Debug, Error)]
pub enum ModifiersReleaseError {
    #[error("Button is released while in Released state")]
    AlreadyReleased,
}
