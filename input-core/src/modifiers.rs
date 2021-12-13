use core::hash::Hash;
use std::{collections::BTreeSet, sync::Arc};

use crate::EventWithModifiers;

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

    pub fn with_press_event(self, switch: Sw) -> Modifiers<Sw>
    where
        Sw: Clone + Eq + Hash + Ord,
    {
        let mut switches = self.switches;
        let switches_mut = Arc::make_mut(&mut switches);
        let is_added = switches_mut.insert(switch);
        assert!(is_added);
        Modifiers { switches }
    }

    pub fn with_release_event(self, switch: Sw) -> Modifiers<Sw>
    where
        Sw: Clone + Eq + Hash + Ord,
    {
        let mut switches = self.switches;
        let switches_mut = Arc::make_mut(&mut switches);
        let is_removed = switches_mut.remove(&switch);
        assert!(is_removed);
        Modifiers { switches }
    }
}

impl<Sw> Default for Modifiers<Sw> {
    fn default() -> Self {
        Self {
            switches: Arc::new(BTreeSet::new()),
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
