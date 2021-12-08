use core::hash::Hash;
use std::{collections::BTreeSet, sync::Arc};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EventWithModifiers<Ev, Sw> {
    pub event: Ev,
    pub modifiers: Arc<BTreeSet<Sw>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModifiedState<Sw> {
    modifiers: Arc<BTreeSet<Sw>>,
}

impl<Ev, Sw> EventWithModifiers<Ev, Sw> {
    pub fn to_state(&self) -> ModifiedState<Sw> {
        ModifiedState {
            modifiers: Arc::clone(&self.modifiers),
        }
    }
}

impl<Sw> ModifiedState<Sw> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn modifiers(&self) -> &Arc<BTreeSet<Sw>> {
        &self.modifiers
    }

    pub fn with_press_event<Ev>(self, event: Ev, switch: Sw) -> EventWithModifiers<Ev, Sw>
    where
        Sw: Clone + Eq + Hash + Ord,
    {
        let mut modifiers = self.modifiers;
        let modifiers_mut = Arc::make_mut(&mut modifiers);
        let is_added = modifiers_mut.insert(switch);
        assert!(is_added);
        EventWithModifiers { event, modifiers }
    }

    pub fn with_release_event<Ev>(self, event: Ev, switch: Sw) -> EventWithModifiers<Ev, Sw>
    where
        Sw: Clone + Eq + Hash + Ord,
    {
        let mut modifiers = self.modifiers;
        let modifiers_mut = Arc::make_mut(&mut modifiers);
        let is_removed = modifiers_mut.remove(&switch);
        assert!(is_removed);
        EventWithModifiers { event, modifiers }
    }

    pub fn with_trigger_event<Ev>(self, event: Ev) -> EventWithModifiers<Ev, Sw>
    where
        Sw: Clone + Eq + Hash + Ord,
    {
        let modifiers = self.modifiers;
        EventWithModifiers { event, modifiers }
    }
}

impl<Sw> Default for ModifiedState<Sw> {
    fn default() -> Self {
        Self {
            modifiers: Arc::new(BTreeSet::new()),
        }
    }
}

impl<Ev, Sw> From<EventWithModifiers<Ev, Sw>> for ModifiedState<Sw> {
    fn from(event: EventWithModifiers<Ev, Sw>) -> Self {
        Self {
            modifiers: event.modifiers,
        }
    }
}

/*
impl<T> From<T> for T {
    fn from(t: T) -> T {
        t
    }
}
*/
