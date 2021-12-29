use std::collections::{BTreeSet, HashSet};

use crate::Binding;

#[derive(Clone, Debug)]
pub struct Mapping<Sw, Mo, Ti, Ev> {
    pub bindings: HashSet<Binding<Sw, Mo, Ti, Ev>>,
}

impl<Sw, Mo, Ti, Ev> Mapping<Sw, Mo, Ti, Ev> {
    pub fn new(bindings: HashSet<Binding<Sw, Mo, Ti, Ev>>) -> Self {
        Self { bindings }
    }

    pub fn bindings(&self) -> &HashSet<Binding<Sw, Mo, Ti, Ev>> {
        &self.bindings
    }

    pub fn into_bindings(self) -> HashSet<Binding<Sw, Mo, Ti, Ev>> {
        self.bindings
    }
}

impl<Sw, Mo, Ti, Ev> Default for Mapping<Sw, Mo, Ti, Ev> {
    fn default() -> Self {
        Self {
            bindings: HashSet::default(),
        }
    }
}
