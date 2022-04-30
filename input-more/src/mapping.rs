use std::collections::HashSet;

use crate::Binding;

/// A structure that stores device input mapping rules.
#[derive(Clone, Debug)]
pub struct Mapping<Sw, Tr, Mo, Ev> {
    pub bindings: HashSet<Binding<Sw, Tr, Mo, Ev>>,
}

impl<Sw, Tr, Mo, Ev> Mapping<Sw, Tr, Mo, Ev> {
    /// Constructs a `Mapping` structure from specified device bindings.
    pub fn new(bindings: HashSet<Binding<Sw, Tr, Mo, Ev>>) -> Self {
        Self { bindings }
    }

    /// Returns a reference to the contained device bindings.
    pub fn bindings(&self) -> &HashSet<Binding<Sw, Tr, Mo, Ev>> {
        &self.bindings
    }

    /// Converts `Mapping` structure into the contained device bindings.
    pub fn into_bindings(self) -> HashSet<Binding<Sw, Tr, Mo, Ev>> {
        self.bindings
    }
}

impl<Sw, Tr, Mo, Ev> Default for Mapping<Sw, Tr, Mo, Ev> {
    fn default() -> Self {
        Self {
            bindings: HashSet::default(),
        }
    }
}
