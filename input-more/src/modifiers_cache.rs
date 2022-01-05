use core::hash::Hash;
use std::collections::HashSet;

use crate::SwitchMapping;

#[derive(Clone, Debug)]
pub struct ModifiersCache<Mo> {
    switches: HashSet<Mo>,
}

impl<'a, Sw, Mo, Td, Pd, Ev> From<&'a SwitchMapping<Sw, Mo, Td, Pd, Ev>> for ModifiersCache<&'a Mo>
where
    Mo: 'a + Eq + Hash,
{
    fn from(mapping: &'a SwitchMapping<Sw, Mo, Td, Pd, Ev>) -> Self {
        let switches = mapping
            .bindings()
            .iter()
            .flat_map(|binding| binding.modifiers.switches().as_ref())
            /*// TODO: without vec
            .flat_map(|binding| {
                let switches: Vec<_> = binding.modifiers.switches().iter().collect();
                switches
            })*/
            .collect();
        Self { switches }
    }
}

impl<Mo> FromIterator<Mo> for ModifiersCache<Mo>
where
    Mo: Eq + Hash,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Mo>,
    {
        Self {
            switches: iter.into_iter().collect(),
        }
    }
}

impl<Mo> ModifiersCache<Mo> {
    pub fn switches(&self) -> &HashSet<Mo> {
        &self.switches
    }
}

impl<Mo> Default for ModifiersCache<Mo> {
    fn default() -> Self {
        Self {
            switches: HashSet::new(),
        }
    }
}
