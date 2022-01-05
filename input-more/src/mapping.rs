use std::collections::HashSet;

use crate::{MoveBinding, SwitchBinding, TriggerBinding};

#[derive(Clone, Debug)]
pub struct SwitchMapping<Sw, Mo, Td, Pd, Ev> {
    pub bindings: HashSet<SwitchBinding<Sw, Mo, Td, Pd, Ev>>,
}

#[derive(Clone, Debug)]
pub struct TriggerMapping<Tr, Mo, Ev> {
    pub bindings: HashSet<TriggerBinding<Tr, Mo, Ev>>,
}

#[derive(Clone, Debug)]
pub struct MoveMapping<Mo, Pd, Ev> {
    pub bindings: HashSet<MoveBinding<Mo, Pd, Ev>>,
}

impl<Sw, Mo, Td, Pd, Ev> SwitchMapping<Sw, Mo, Td, Pd, Ev> {
    pub fn new(bindings: HashSet<SwitchBinding<Sw, Mo, Td, Pd, Ev>>) -> Self {
        Self { bindings }
    }

    pub fn bindings(&self) -> &HashSet<SwitchBinding<Sw, Mo, Td, Pd, Ev>> {
        &self.bindings
    }

    pub fn into_bindings(self) -> HashSet<SwitchBinding<Sw, Mo, Td, Pd, Ev>> {
        self.bindings
    }
}

impl<Sw, Mo, Td, Pd, Ev> Default for SwitchMapping<Sw, Mo, Td, Pd, Ev> {
    fn default() -> Self {
        Self {
            bindings: HashSet::default(),
        }
    }
}
