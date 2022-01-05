use core::hash::Hash;
use std::collections::HashMap;

use input_core::Modifiers;

use crate::SwitchMapping;

type MappingData<Sw, Mo, Td, Pd, Ev> = HashMap<Sw, MappingDataBySwitch<Mo, Td, Pd, Ev>>;
type MappingDataBySwitch<Mo, Td, Pd, Ev> =
    HashMap<Modifiers<Mo>, MappingDataByModifiers<Td, Pd, Ev>>;
type MappingDataByModifiers<Td, Pd, Ev> = HashMap<Td, MappingDataByTimed<Pd, Ev>>;
type MappingDataByTimed<Pd, Ev> = HashMap<Pd, MappingDataByPointer<Ev>>;
type MappingDataByPointer<Ev> = Vec<Ev>;

#[derive(Clone, Debug)]
pub struct MappingCache<Sw, Mo, Td, Pd, Ev>(MappingData<Sw, Mo, Td, Pd, Ev>);

#[derive(Clone, Debug)]
pub struct MappingBySwitch<'a, Mo, Td, Pd, Ev>(&'a MappingDataBySwitch<Mo, Td, Pd, Ev>);

#[derive(Clone, Debug)]
pub struct MappingByModifiers<'a, Mo, Td, Pd, Ev>(
    HashMap<&'a Modifiers<Mo>, &'a MappingDataByModifiers<Td, Pd, Ev>>,
);

#[derive(Clone, Debug)]
pub struct MappingByTimed<'a, Mo, Pd, Ev>(
    HashMap<&'a Modifiers<Mo>, &'a MappingDataByTimed<Pd, Ev>>,
);

#[derive(Clone, Debug)]
pub struct MappingByPointer<'a, Mo, Ev>(HashMap<&'a Modifiers<Mo>, &'a MappingDataByPointer<Ev>>);

pub type Bindings<'a, Mo, Ev> = MappingByPointer<'a, Mo, Ev>;

impl<Sw, Mo, Td, Pd, Ev> From<SwitchMapping<Sw, Mo, Td, Pd, Ev>>
    for MappingCache<Sw, Mo, Td, Pd, Ev>
where
    Sw: Eq + Hash,
    Mo: Eq + Hash,
    Td: Eq + Hash,
    Pd: Eq + Hash,
{
    fn from(mapping: SwitchMapping<Sw, Mo, Td, Pd, Ev>) -> Self {
        let mut data: MappingData<Sw, Mo, Td, Pd, Ev> = HashMap::new();
        for binding in mapping.into_bindings() {
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

impl<Sw, Mo, Td, Pd, Ev> Default for MappingCache<Sw, Mo, Td, Pd, Ev> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<Sw, Mo, Td, Pd, Ev> MappingCache<Sw, Mo, Td, Pd, Ev> {
    pub fn filter_by_switch(&self, switch: &Sw) -> Option<MappingBySwitch<'_, Mo, Td, Pd, Ev>>
    where
        Sw: Eq + Hash,
    {
        self.0.get(switch).map(MappingBySwitch)
    }
}

/*
pub trait FilterBySwitch<Sw> {
    fn filter<'b, Mo, Td, Pd, Ev>(
        &self,
        mapping: &'b MappingCache<Sw, Mo, Td, Pd, Ev>,
    ) -> Option<MappingBySwitch<'b, Mo, Td, Pd, Ev>>;
}

impl<Sw> FilterBySwitch<Sw> for Sw
where
    Sw: Eq + Hash,
{
    fn filter<'b, Mo, Td, Pd, Ev>(
        &self,
        mapping: &'b MappingCache<Sw, Mo, Td, Pd, Ev>,
    ) -> Option<MappingBySwitch<'b, Mo, Td, Pd, Ev>> {
        mapping.filter_by_switch(&self)
    }
}*/

impl<'a, Mo, Td, Pd, Ev> MappingBySwitch<'a, Mo, Td, Pd, Ev> {
    pub fn filter_by_modifiers(
        &self,
        modifiers: &Modifiers<Mo>,
    ) -> Option<MappingByModifiers<'a, Mo, Td, Pd, Ev>>
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
            Some(MappingByModifiers(bindings))
        }
    }
}

// TODO: deduplicate these
impl<'a, Mo, Td, Pd, Ev> MappingByModifiers<'a, Mo, Td, Pd, Ev> {
    pub fn filter_by_timed_data(&self, timed_data: &Td) -> Option<MappingByTimed<'a, Mo, Pd, Ev>>
    where
        Mo: Eq + Hash,
        Td: Eq + Hash,
    {
        let mapping = MappingByTimed(
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

impl<'a, Mo, Pd, Ev> MappingByTimed<'a, Mo, Pd, Ev> {
    pub fn filter_by_pointer_data(&self, pointer_data: &Pd) -> Option<MappingByPointer<'a, Mo, Ev>>
    where
        Mo: Eq + Hash,
        Pd: Eq + Hash,
    {
        let mapping = MappingByPointer(
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

impl<'a, Mo, Ev> Bindings<'a, Mo, Ev> {
    pub fn into_inner(self) -> HashMap<&'a Modifiers<Mo>, &'a Vec<Ev>> {
        self.0
    }

    pub fn inner(&self) -> &HashMap<&'a Modifiers<Mo>, &'a Vec<Ev>> {
        &self.0
    }
}
