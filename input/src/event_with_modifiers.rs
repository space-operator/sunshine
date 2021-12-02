use core::hash::Hash;
use std::{collections::BTreeSet, sync::Arc};

use crate::{Action, EventWithAction};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModifiedEvent<Ev, Sw> {
    event: Ev,
    modifiers: Arc<BTreeSet<Sw>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModifiedState<Sw> {
    modifiers: Arc<BTreeSet<Sw>>,
}

impl<Ev, Sw> ModifiedEvent<Ev, Sw> {
    fn to_state(&self) -> ModifiedState<Sw> {
        ModifiedState {
            modifiers: Arc::clone(&self.modifiers),
        }
    }

    pub fn modifiers(&self) -> &Arc<BTreeSet<Sw>> {
        &self.modifiers
    }
}

impl<Sw> ModifiedState<Sw> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn modifiers(&self) -> &Arc<BTreeSet<Sw>> {
        &self.modifiers
    }

    pub fn with_event<Ev>(self, event: Ev) -> ModifiedEvent<Ev, Sw>
    where
        Ev: EventWithAction<Switch = Sw>,
        Sw: Clone + Eq + Hash + Ord,
    {
        let mut modifiers = self.modifiers;
        match event.action() {
            Some(Action::Enable(switch)) => {
                let modifiers_mut = Arc::make_mut(&mut modifiers);
                let is_added = modifiers_mut.insert(switch);
                assert!(is_added);
                ModifiedEvent { event, modifiers }
            }
            Some(Action::Disable(switch)) => {
                let modifiers_mut = Arc::make_mut(&mut modifiers);
                let is_removed = modifiers_mut.remove(&switch);
                assert!(is_removed);
                ModifiedEvent { event, modifiers }
            }
            None => ModifiedEvent { event, modifiers },
        }
    }
}

impl<Sw> Default for ModifiedState<Sw> {
    fn default() -> Self {
        Self {
            modifiers: Arc::new(BTreeSet::new()),
        }
    }
}

impl<Ev, Sw> From<ModifiedEvent<Ev, Sw>> for ModifiedState<Sw> {
    fn from(event: ModifiedEvent<Ev, Sw>) -> Self {
        Self {
            modifiers: event.modifiers,
        }
    }
}
