use core::hash::Hash;
use std::collections::HashSet;

use crate::Binding;

#[derive(Clone, Debug)]
pub struct MappingModifiersCache<Mo> {
    switches: HashSet<Mo>,
}

impl<Mo> MappingModifiersCache<Mo>
where
    Mo: Clone + Eq + Hash,
{
    pub fn from_bindings<'a, Sw, Tr, Ev>(
        mapping: impl IntoIterator<Item = &'a Binding<Sw, Tr, Mo, Ev>>,
    ) -> Self
    where
        Sw: 'a,
        Tr: 'a,
        Ev: 'a,
        Mo: 'a,
    {
        let switches = mapping
            .into_iter()
            .flat_map(|binding| binding.modifiers().switches().as_ref())
            .cloned()
            /*// TODO: without vec
            .flat_map(|binding| {
                let switches: Vec<_> = binding.modifiers.switches().iter().collect();
                switches
            })*/
            .collect();
        Self { switches }
    }
}

/*
impl<Mo> FromIterator<Mo> for MappingModifiersCache<Mo>
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
*/

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
