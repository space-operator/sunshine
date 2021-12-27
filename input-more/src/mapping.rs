use core::hash::Hash;
use std::collections::HashMap;

use input_core::Modifiers;

type MappingData<Sw, Mo, Ti, Bi> = HashMap<Sw, MappingDataBySwitch<Mo, Ti, Bi>>;
type MappingDataBySwitch<Mo, Ti, Bi> = HashMap<Modifiers<Mo>, MappingDataByModifiers<Ti, Bi>>;
type MappingDataByModifiers<Ti, Bi> = HashMap<Ti, MappingDataByTimed<Bi>>;
type MappingDataByTimed<Bi> = Vec<Bi>;

#[derive(Clone, Debug)]
pub struct Mapping<Sw, Mo, Ti, Bi>(MappingData<Sw, Mo, Ti, Bi>);

#[derive(Clone, Debug)]
pub struct MappingBySwitch<'a, Mo, Ti, Bi>(&'a MappingDataBySwitch<Mo, Ti, Bi>);

#[derive(Clone, Debug)]
pub struct MappingByModifiers<'a, Mo, Ti, Bi>(
    HashMap<&'a Modifiers<Mo>, &'a MappingDataByModifiers<Ti, Bi>>,
);

#[derive(Clone, Debug)]
pub struct MappingByTimed<'a, Mo, Bi>(HashMap<&'a Modifiers<Mo>, &'a MappingDataByTimed<Bi>>);

impl<Sw, Mo, Ti, Bi> Mapping<Sw, Mo, Ti, Bi> {
    pub fn new(data: MappingData<Sw, Mo, Ti, Bi>) -> Self {
        Self(data)
    }
}

impl<Sw, Mo, Ti, Bi> Default for Mapping<Sw, Mo, Ti, Bi> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<Sw, Mo, Ti, Bi> From<MappingData<Sw, Mo, Ti, Bi>> for Mapping<Sw, Mo, Ti, Bi> {
    fn from(data: MappingData<Sw, Mo, Ti, Bi>) -> Self {
        Self(data)
    }
}

impl<Sw, Mo, Ti, Bi> Mapping<Sw, Mo, Ti, Bi> {
    pub fn filter_by_switch(&self, switch: &Sw) -> Option<MappingBySwitch<'_, Mo, Ti, Bi>>
    where
        Sw: Eq + Hash,
    {
        self.0.get(switch).map(MappingBySwitch)
    }
}

impl<'a, Mo, Ti, Bi> MappingBySwitch<'a, Mo, Ti, Bi> {
    pub fn filter_by_modifiers(
        &self,
        modifiers: &Modifiers<Mo>,
    ) -> Option<MappingByModifiers<'a, Mo, Ti, Bi>>
    where
        Mo: Eq + Hash,
    {
        // TODO: Fixme, partial cmp, longest modifiers, priority, cancellation
        self.0
            .get_key_value(modifiers)
            .map(|(modifiers, filtered)| [(modifiers, filtered)].into_iter().collect())
            .map(MappingByModifiers)
    }
}

impl<'a, Mo, Ti, Bi> MappingByModifiers<'a, Mo, Ti, Bi> {
    pub fn filter_by_timed_data(&self, timed_data: &Ti) -> Option<MappingByTimed<'a, Mo, Bi>>
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

impl<'a, Mo, Bi> MappingByTimed<'a, Mo, Bi> {
    pub fn into_inner(self) -> HashMap<&'a Modifiers<Mo>, &'a MappingDataByTimed<Bi>> {
        self.0
    }
}
