use std::collections::HashSet;

use crate::Binding;

#[derive(Clone, Debug)]
pub struct Mapping<Sw, Mo, Td, Pd, Ev> {
    pub bindings: HashSet<Binding<Sw, Mo, Td, Pd, Ev>>,
}

impl<Sw, Mo, Td, Pd, Ev> Mapping<Sw, Mo, Td, Pd, Ev> {
    pub fn new(bindings: HashSet<Binding<Sw, Mo, Td, Pd, Ev>>) -> Self {
        Self { bindings }
    }

    pub fn bindings(&self) -> &HashSet<Binding<Sw, Mo, Td, Pd, Ev>> {
        &self.bindings
    }

    pub fn into_bindings(self) -> HashSet<Binding<Sw, Mo, Td, Pd, Ev>> {
        self.bindings
    }
}

impl<Sw, Mo, Td, Pd, Ev> Default for Mapping<Sw, Mo, Td, Pd, Ev> {
    fn default() -> Self {
        Self {
            bindings: HashSet::default(),
        }
    }
}
