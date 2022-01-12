use std::collections::HashSet;

use crate::Binding;

#[derive(Clone, Debug)]
pub struct DeviceMapping<Sw, Tr, Mo, Ev> {
    pub bindings: HashSet<Binding<Sw, Tr, Mo, Ev>>,
}

impl<Sw, Tr, Mo, Ev> DeviceMapping<Sw, Tr, Mo, Ev> {
    pub fn new(bindings: HashSet<Binding<Sw, Tr, Mo, Ev>>) -> Self {
        Self { bindings }
    }

    pub fn bindings(&self) -> &HashSet<Binding<Sw, Tr, Mo, Ev>> {
        &self.bindings
    }

    pub fn into_bindings(self) -> HashSet<Binding<Sw, Tr, Mo, Ev>> {
        self.bindings
    }
}

impl<Sw, Tr, Mo, Ev> Default for DeviceMapping<Sw, Tr, Mo, Ev> {
    fn default() -> Self {
        Self {
            bindings: HashSet::default(),
        }
    }
}
