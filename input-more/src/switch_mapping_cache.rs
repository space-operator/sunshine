use core::hash::Hash;
use std::collections::HashMap;

use input_core::Modifiers;

use crate::SwitchBinding;

type SwitchMappingData<Sw, Mo, Td, Pd, Ev> = HashMap<Sw, SwitchMappingDataBySwitch<Mo, Td, Pd, Ev>>;
type SwitchMappingDataBySwitch<Mo, Td, Pd, Ev> =
    HashMap<Modifiers<Mo>, SwitchMappingDataByModifiers<Td, Pd, Ev>>;
type SwitchMappingDataByModifiers<Td, Pd, Ev> = HashMap<Td, SwitchMappingDataByTimed<Pd, Ev>>;
type SwitchMappingDataByTimed<Pd, Ev> = HashMap<Pd, SwitchMappingDataByPointer<Ev>>;
type SwitchMappingDataByPointer<Ev> = Vec<Ev>;

#[derive(Clone, Debug)]
pub struct SwitchMappingCache<Sw, Mo, Td, Pd, Ev>(SwitchMappingData<Sw, Mo, Td, Pd, Ev>);

#[derive(Clone, Debug)]
pub struct SwitchMappingBySwitch<'a, Mo, Td, Pd, Ev>(&'a SwitchMappingDataBySwitch<Mo, Td, Pd, Ev>);

#[derive(Clone, Debug)]
pub struct SwitchMappingByModifiers<'a, Mo, Td, Pd, Ev>(
    HashMap<&'a Modifiers<Mo>, &'a SwitchMappingDataByModifiers<Td, Pd, Ev>>,
);

#[derive(Clone, Debug)]
pub struct SwitchMappingByTimed<'a, Mo, Pd, Ev>(
    HashMap<&'a Modifiers<Mo>, &'a SwitchMappingDataByTimed<Pd, Ev>>,
);

#[derive(Clone, Debug)]
pub struct SwitchMappingByPointer<'a, Mo, Ev>(
    HashMap<&'a Modifiers<Mo>, &'a SwitchMappingDataByPointer<Ev>>,
);

pub type SwitchBindings<'a, Mo, Ev> = SwitchMappingByPointer<'a, Mo, Ev>;

impl<Sw, Mo, Td, Pd, Ev> SwitchMappingCache<Sw, Mo, Td, Pd, Ev>
where
    Sw: Eq + Hash,
    Mo: Eq + Hash,
    Td: Eq + Hash,
    Pd: Eq + Hash,
{
    pub fn from_bindings(
        mapping: impl IntoIterator<Item = SwitchBinding<Sw, Mo, Td, Pd, Ev>>,
    ) -> Self {
        let mut data: SwitchMappingData<Sw, Mo, Td, Pd, Ev> = HashMap::new();
        for binding in mapping.into_iter() {
            let events: &mut _ = data
                .entry(binding.switch)
                .or_default()
                .entry(binding.modifiers)
                .or_default()
                .entry(binding.timed_data)
                .or_default()
                .entry(binding.pointer_data)
                .or_default();
            events.push(binding.event);
        }

        Self(data)
    }
}

impl<Sw, Mo, Td, Pd, Ev> Default for SwitchMappingCache<Sw, Mo, Td, Pd, Ev> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<Sw, Mo, Td, Pd, Ev> SwitchMappingCache<Sw, Mo, Td, Pd, Ev> {
    pub fn filter_by_switch(&self, switch: &Sw) -> Option<SwitchMappingBySwitch<'_, Mo, Td, Pd, Ev>>
    where
        Sw: Eq + Hash,
    {
        self.0.get(switch).map(SwitchMappingBySwitch)
    }
}

impl<'a, Mo, Td, Pd, Ev> SwitchMappingBySwitch<'a, Mo, Td, Pd, Ev> {
    pub fn filter_by_modifiers(
        &self,
        modifiers: &Modifiers<Mo>,
    ) -> Option<SwitchMappingByModifiers<'a, Mo, Td, Pd, Ev>>
    where
        Mo: Eq + Hash + Ord,
    {
        let bindings: HashMap<_, _> = self
            .0
            .iter()
            .filter(|(binding_modifiers, _)| {
                binding_modifiers.switches().is_subset(modifiers.switches())
            })
            .collect();
        if bindings.is_empty() {
            None
        } else {
            Some(SwitchMappingByModifiers(bindings))
        }
    }
}

// TODO: deduplicate these
impl<'a, Mo, Td, Pd, Ev> SwitchMappingByModifiers<'a, Mo, Td, Pd, Ev> {
    pub fn filter_by_timed_data(
        &self,
        timed_data: &Td,
    ) -> Option<SwitchMappingByTimed<'a, Mo, Pd, Ev>>
    where
        Mo: Eq + Hash,
        Td: Eq + Hash,
    {
        let mapping = SwitchMappingByTimed(
            self.0
                .iter()
                .filter_map(|(&modifiers, &filtered)| {
                    filtered
                        .get(timed_data)
                        .map(|filtered| (modifiers, filtered))
                })
                .collect(),
        );
        if mapping.0.is_empty() {
            None
        } else {
            Some(mapping)
        }
    }
}

impl<'a, Mo, Pd, Ev> SwitchMappingByTimed<'a, Mo, Pd, Ev> {
    pub fn filter_by_pointer_data(
        &self,
        pointer_data: &Pd,
    ) -> Option<SwitchMappingByPointer<'a, Mo, Ev>>
    where
        Mo: Eq + Hash,
        Pd: Eq + Hash,
    {
        let mapping = SwitchMappingByPointer(
            self.0
                .iter()
                .filter_map(|(&modifiers, &filtered)| {
                    filtered
                        .get(pointer_data)
                        .map(|filtered| (modifiers, filtered))
                })
                .collect(),
        );
        if mapping.0.is_empty() {
            None
        } else {
            Some(mapping)
        }
    }
}

impl<'a, Mo, Ev> SwitchBindings<'a, Mo, Ev> {
    pub fn into_inner(self) -> HashMap<&'a Modifiers<Mo>, &'a Vec<Ev>> {
        self.0
    }

    pub fn inner(&self) -> &HashMap<&'a Modifiers<Mo>, &'a Vec<Ev>> {
        &self.0
    }
}
