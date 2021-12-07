SHOULD BE REMOVED OR COMMENTED

use core::hash::Hash;
use std::{collections::BTreeSet, sync::Arc};

use crate::{Action, EventWithAction};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EventWithModifiers<Ev, Sw> {
    event: Ev,
    modifiers: Arc<BTreeSet<Sw>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModifiersState<Sw> {
    modifiers: Arc<BTreeSet<Sw>>,
}

impl<Ev, Sw> EventWithModifiers<Ev, Sw> {
    fn to_state(&self) -> ModifiersState<Sw> {
        ModifiersState {
            modifiers: Arc::clone(&self.modifiers),
        }
    }

    pub fn modifiers(&self) -> &Arc<BTreeSet<Sw>> {
        &self.modifiers
    }
}

impl<Sw> ModifiersState<Sw> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn modifiers(&self) -> &Arc<BTreeSet<Sw>> {
        &self.modifiers
    }

    pub fn with_event<Ev>(self, event: Ev) -> EventWithModifiers<Ev, Sw>
    where
        Ev: EventWithAction<Switch = Sw>,
        Sw: Clone + Eq + Hash + Ord,
    {
        let mut modifiers = self.modifiers;
        match event.action() {
            Some(Action::Enable(switch)) => {
                let mut modifiers_mut = Arc::make_mut(&mut modifiers);
                let is_added = modifiers_mut.insert(switch);
                assert!(is_added);
                EventWithModifiers { event, modifiers }
            }
            Some(Action::Disable(switch)) => {
                let mut modifiers_mut = Arc::make_mut(&mut modifiers);
                let is_removed = modifiers_mut.remove(&switch);
                assert!(is_removed);
                EventWithModifiers { event, modifiers }
            }
            None => EventWithModifiers { event, modifiers },
        }
    }
}

impl<Sw> Default for ModifiersState<Sw> {
    fn default() -> Self {
        Self {
            modifiers: Arc::new(BTreeSet::new()),
        }
    }
}

impl<Ev, Sw> From<EventWithModifiers<Ev, Sw>> for ModifiersState<Sw> {
    fn from(event: EventWithModifiers<Ev, Sw>) -> Self {
        Self {
            modifiers: event.modifiers,
        }
    }
}
