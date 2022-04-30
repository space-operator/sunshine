use core::hash::Hash;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct MappingModifiersCache<Mo> {
    switches: HashSet<Mo>,
}

impl<Mo> MappingModifiersCache<Mo>
where
    Mo: Clone + Eq + Hash,
{
    pub fn from_switches(switches: impl IntoIterator<Item = Mo>) -> Self {
        Self {
            switches: switches.into_iter().collect(),
        }
    }
}

impl<Mo> MappingModifiersCache<Mo> {
    pub fn switches(&self) -> &HashSet<Mo> {
        &self.switches
    }
}

impl<Mo> Default for MappingModifiersCache<Mo> {
    fn default() -> Self {
        Self {
            switches: HashSet::new(),
        }
    }
}
