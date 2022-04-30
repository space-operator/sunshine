use core::hash::Hash;
use std::collections::HashSet;

/// A structure used to check the use of the switch as a modifier for other switches.
///
/// This helps to avoid the processing of switches
/// that are not used as switches in switch events
/// and are not used as modifiers for other switch/trigger or move events.
#[derive(Clone, Debug)]
pub struct MappingModifiersCache<Mo> {
    switches: HashSet<Mo>,
}

impl<Mo> MappingModifiersCache<Mo>
where
    Mo: Clone + Eq + Hash,
{
    /// Builds `MappingModifiersCache` structure from mapping modifiers switches.
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
