use core::hash::Hash;
use std::collections::HashMap;

use input_core::Modifiers;

use crate::Mapping;

type MappingData<Sw, Mo, Ti, Ev> = HashMap<Sw, MappingDataBySwitch<Mo, Ti, Ev>>;
type MappingDataBySwitch<Mo, Ti, Ev> = HashMap<Modifiers<Mo>, MappingDataByModifiers<Ti, Ev>>;
type MappingDataByModifiers<Ti, Ev> = HashMap<Ti, MappingDataByTimed<Ev>>;
type MappingDataByTimed<Ev> = Vec<Ev>;

#[derive(Clone, Debug)]
pub struct MappingCache<Sw, Mo, Ti, Ev>(MappingData<Sw, Mo, Ti, Ev>);

#[derive(Clone, Debug)]
pub struct MappingBySwitch<'a, Mo, Ti, Ev>(&'a MappingDataBySwitch<Mo, Ti, Ev>);

#[derive(Clone, Debug)]
pub struct MappingByModifiers<'a, Mo, Ti, Ev>(
    HashMap<&'a Modifiers<Mo>, &'a MappingDataByModifiers<Ti, Ev>>,
);

#[derive(Clone, Debug)]
pub struct MappingByTimed<'a, Mo, Ev>(HashMap<&'a Modifiers<Mo>, &'a MappingDataByTimed<Ev>>);

impl<Sw, Mo, Ti, Ev> From<Mapping<Sw, Mo, Ti, Ev>> for MappingCache<Sw, Mo, Ti, Ev>
where
    Sw: Eq + Hash,
    Mo: Eq + Hash,
    Ti: Eq + Hash,
{
    fn from(mapping: Mapping<Sw, Mo, Ti, Ev>) -> Self {
        let mut data: MappingData<Sw, Mo, Ti, Ev> = HashMap::new();
        for binding in mapping.into_bindings() {
            let events: &mut _ = data
                .entry(binding.switch)
                .or_default()
                .entry(binding.modifiers)
                .or_default()
                .entry(binding.timed_data)
                .or_default();
            events.push(binding.event);
        }

        Self(data)
    }
}

impl<Sw, Mo, Ti, Ev> Default for MappingCache<Sw, Mo, Ti, Ev> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<Sw, Mo, Ti, Ev> MappingCache<Sw, Mo, Ti, Ev> {
    pub fn filter_by_switch(&self, switch: &Sw) -> Option<MappingBySwitch<'_, Mo, Ti, Ev>>
    where
        Sw: Eq + Hash,
    {
        self.0.get(switch).map(MappingBySwitch)
    }
}

/*
pub trait FilterBySwitch<Sw> {
    fn filter<'b, Mo, Ti, Ev>(
        &self,
        mapping: &'b MappingCache<Sw, Mo, Ti, Ev>,
    ) -> Option<MappingBySwitch<'b, Mo, Ti, Ev>>;
}

impl<Sw> FilterBySwitch<Sw> for Sw
where
    Sw: Eq + Hash,
{
    fn filter<'b, Mo, Ti, Ev>(
        &self,
        mapping: &'b MappingCache<Sw, Mo, Ti, Ev>,
    ) -> Option<MappingBySwitch<'b, Mo, Ti, Ev>> {
        mapping.filter_by_switch(&self)
    }
}*/

impl<'a, Mo, Ti, Ev> MappingBySwitch<'a, Mo, Ti, Ev> {
    pub fn filter_by_modifiers(
        &self,
        modifiers: &Modifiers<Mo>,
    ) -> Option<MappingByModifiers<'a, Mo, Ti, Ev>>
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

impl<'a, Mo, Ti, Ev> MappingByModifiers<'a, Mo, Ti, Ev> {
    pub fn filter_by_timed_data(&self, timed_data: &Ti) -> Option<MappingByTimed<'a, Mo, Ev>>
    where
        Mo: Eq + Hash,
        Ti: Eq + Hash,
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

impl<'a, Mo, Ev> MappingByTimed<'a, Mo, Ev> {
    pub fn into_inner(self) -> HashMap<&'a Modifiers<Mo>, &'a MappingDataByTimed<Ev>> {
        self.0
    }

    pub fn inner(&self) -> &HashMap<&'a Modifiers<Mo>, &'a MappingDataByTimed<Ev>> {
        &self.0
    }
}
