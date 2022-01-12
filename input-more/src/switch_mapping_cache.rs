use core::hash::Hash;
use std::collections::HashMap;

use input_core::Modifiers;

use crate::{CoordsBinding, SwitchBinding, TriggerBinding};

#[derive(Clone, Debug)]
pub struct SwitchMappingCache<Sw, Mo, Td, Pd, Bu>(SwitchMappingData<Sw, Mo, Td, Pd, Bu>);

#[derive(Clone, Debug)]
pub struct SwitchMappingBySwitch<'a, Mo, Td, Pd, Bu>(&'a SwitchMappingDataBySwitch<Mo, Td, Pd, Bu>);

#[derive(Clone, Debug)]
pub struct SwitchMappingByModifiers<'a, Mo, Td, Pd, Bu>(
    HashMap<&'a Modifiers<Mo>, &'a SwitchMappingDataByModifiers<Td, Pd, Bu>>,
);

#[derive(Clone, Debug)]
pub struct SwitchMappingByTimed<'a, Mo, Pd, Bu>(
    HashMap<&'a Modifiers<Mo>, &'a SwitchMappingDataByTimed<Pd, Bu>>,
);

type SwitchMappingData<Sw, Mo, Td, Pd, Bu> = HashMap<Sw, SwitchMappingDataBySwitch<Mo, Td, Pd, Bu>>;
type SwitchMappingDataBySwitch<Mo, Td, Pd, Bu> =
    HashMap<Modifiers<Mo>, SwitchMappingDataByModifiers<Td, Pd, Bu>>;
type SwitchMappingDataByModifiers<Td, Pd, Bu> = HashMap<Td, SwitchMappingDataByTimed<Pd, Bu>>;
type SwitchMappingDataByTimed<Pd, Bu> = HashMap<Pd, Vec<Bu>>;

#[derive(Clone, Debug)]
pub struct TriggerMappingCache<Tr, Mo, Bu>(TriggerMappingData<Tr, Mo, Bu>);

#[derive(Clone, Debug)]
pub struct TriggerMappingByTrigger<'a, Mo, Bu>(&'a TriggerMappingDataByTrigger<Mo, Bu>);

type TriggerMappingData<Tr, Mo, Bu> = HashMap<Tr, TriggerMappingDataByTrigger<Mo, Bu>>;
type TriggerMappingDataByTrigger<Mo, Bu> = HashMap<Modifiers<Mo>, Vec<Bu>>;

#[derive(Clone, Debug)]
pub struct CoordsMappingCache<Pd, Mo, Bu>(CoordsMappingData<Pd, Mo, Bu>);

#[derive(Clone, Debug)]
pub struct CoordsMappingByPointer<'a, Mo, Bu>(&'a CoordsMappingDataByPointer<Mo, Bu>);

type CoordsMappingData<Pd, Mo, Bu> = HashMap<Pd, CoordsMappingDataByPointer<Mo, Bu>>;
type CoordsMappingDataByPointer<Mo, Bu> = HashMap<Modifiers<Mo>, Vec<Bu>>;

#[derive(Clone, Debug)]
pub struct FilteredBindings<'a, Mo, Bu>(HashMap<&'a Modifiers<Mo>, &'a Vec<Bu>>);

impl<Sw, Mo, Td, Pd, Bu> SwitchMappingCache<Sw, Mo, Td, Pd, Bu>
where
    Sw: Eq + Hash,
    Mo: Eq + Hash,
    Td: Eq + Hash,
    Pd: Eq + Hash,
{
    pub fn from_bindings(
        mapping: impl IntoIterator<Item = SwitchBinding<Sw, Mo, Td, Pd, Bu>>,
    ) -> Self {
        let mut data: SwitchMappingData<Sw, Mo, Td, Pd, Bu> = HashMap::new();
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

impl<Tr, Mo, Bu> TriggerMappingCache<Tr, Mo, Bu>
where
    Tr: Eq + Hash,
    Mo: Eq + Hash,
{
    pub fn from_bindings(mapping: impl IntoIterator<Item = TriggerBinding<Tr, Mo, Bu>>) -> Self {
        let mut data: TriggerMappingData<Tr, Mo, Bu> = HashMap::new();
        for binding in mapping.into_iter() {
            let events: &mut _ = data
                .entry(binding.trigger)
                .or_default()
                .entry(binding.modifiers)
                .or_default();
            events.push(binding.event);
        }

        Self(data)
    }
}

impl<Pd, Mo, Bu> CoordsMappingCache<Pd, Mo, Bu>
where
    Pd: Eq + Hash,
    Mo: Eq + Hash,
{
    pub fn from_bindings(mapping: impl IntoIterator<Item = CoordsBinding<Pd, Mo, Bu>>) -> Self {
        let mut data: CoordsMappingData<Pd, Mo, Bu> = HashMap::new();
        for binding in mapping.into_iter() {
            let events: &mut _ = data
                .entry(binding.pointer_data)
                .or_default()
                .entry(binding.modifiers)
                .or_default();
            events.push(binding.event);
        }

        Self(data)
    }
}

impl<Sw, Mo, Td, Pd, Bu> Default for SwitchMappingCache<Sw, Mo, Td, Pd, Bu> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<Tr, Mo, Bu> Default for TriggerMappingCache<Tr, Mo, Bu> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<Pd, Mo, Bu> Default for CoordsMappingCache<Pd, Mo, Bu> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<Sw, Mo, Td, Pd, Bu> SwitchMappingCache<Sw, Mo, Td, Pd, Bu> {
    pub fn filter_by_switch(&self, switch: &Sw) -> Option<SwitchMappingBySwitch<'_, Mo, Td, Pd, Bu>>
    where
        Sw: Eq + Hash,
    {
        self.0.get(switch).map(SwitchMappingBySwitch)
    }
}

impl<'a, Mo, Td, Pd, Bu> SwitchMappingBySwitch<'a, Mo, Td, Pd, Bu> {
    pub fn filter_by_modifiers(
        &self,
        modifiers: &Modifiers<Mo>,
    ) -> Option<SwitchMappingByModifiers<'a, Mo, Td, Pd, Bu>>
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
impl<'a, Mo, Td, Pd, Bu> SwitchMappingByModifiers<'a, Mo, Td, Pd, Bu> {
    pub fn filter_by_timed_data(
        &self,
        timed_data: &Td,
    ) -> Option<SwitchMappingByTimed<'a, Mo, Pd, Bu>>
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

impl<'a, Mo, Pd, Bu> SwitchMappingByTimed<'a, Mo, Pd, Bu> {
    pub fn filter_by_pointer_data(&self, pointer_data: &Pd) -> Option<FilteredBindings<'a, Mo, Bu>>
    where
        Mo: Eq + Hash,
        Pd: Eq + Hash,
    {
        let mapping = FilteredBindings(
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

impl<Tr, Mo, Bu> TriggerMappingCache<Tr, Mo, Bu> {
    pub fn filter_by_switch(&self, trigger: &Tr) -> Option<TriggerMappingByTrigger<'_, Mo, Bu>>
    where
        Tr: Eq + Hash,
    {
        self.0.get(trigger).map(TriggerMappingByTrigger)
    }
}

impl<'a, Mo, Bu> TriggerMappingByTrigger<'a, Mo, Bu> {
    pub fn filter_by_modifiers(
        &self,
        modifiers: &Modifiers<Mo>,
    ) -> Option<FilteredBindings<'a, Mo, Bu>>
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
            Some(FilteredBindings(bindings))
        }
    }
}

impl<Pd, Mo, Bu> CoordsMappingCache<Pd, Mo, Bu> {
    pub fn filter_by_pointer_data(
        &self,
        pointer_data: &Pd,
    ) -> Option<CoordsMappingByPointer<'_, Mo, Bu>>
    where
        Pd: Eq + Hash,
    {
        self.0.get(pointer_data).map(CoordsMappingByPointer)
    }
}

impl<'a, Mo, Bu> CoordsMappingByPointer<'a, Mo, Bu> {
    pub fn filter_by_modifiers(
        &self,
        modifiers: &Modifiers<Mo>,
    ) -> Option<FilteredBindings<'a, Mo, Bu>>
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
            Some(FilteredBindings(bindings))
        }
    }
}

impl<'a, Mo, Bu> FilteredBindings<'a, Mo, Bu> {
    pub fn into_inner(self) -> HashMap<&'a Modifiers<Mo>, &'a Vec<Bu>> {
        self.0
    }

    pub fn inner(&self) -> &HashMap<&'a Modifiers<Mo>, &'a Vec<Bu>> {
        &self.0
    }

    pub fn build<F, Ev>(self, mut handler: F) -> Vec<Ev>
    where
        F: FnMut(&Bu) -> Option<Ev>,
        Mo: Eq + Hash + Ord,
    {
        let bindings: HashMap<_, _> = self
            .into_inner()
            .into_iter()
            .filter_map(|(modifiers, events)| {
                let events: Vec<_> = events
                    .into_iter()
                    .filter_map(|binding| handler(binding))
                    .collect();
                if events.is_empty() {
                    None
                } else {
                    Some((modifiers, events))
                }
            })
            .collect();

        let events_mask: Vec<_> = bindings
            .iter()
            .map(|(modifiers, _)| {
                bindings.iter().all(|(other_modifiers, _)| {
                    modifiers.switches().is_superset(other_modifiers.switches())
                })
            })
            .collect();

        bindings
            .into_iter()
            .enumerate()
            .filter_map(|(j, event)| if events_mask[j] { Some(event) } else { None })
            .flat_map(|(_, events)| events)
            .collect()
    }
}
