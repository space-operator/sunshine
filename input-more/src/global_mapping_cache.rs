pub trait GlobalMappingCache<Sw> {
    type MappingDataBySwitch: GlobalMappingDataBySwitch;

    fn filter_by_switch(&self, switch: &Sw) -> Option<Self::MappingBySwitch>;
}

pub trait GlobalMappingDataBySwitch<Sw> {
    type MappingByModifiers: GlobalMappingByModifiers;

    fn filter_by_modifiers(
        &self,
        modifiers: &Modifiers<Mo>,
    ) -> Option<MappingByModifiers<'a, Mo, Ti, Ev>>;
}

pub trait GlobalMappingByModifiers<Sw> {
    type MappingByModifiers: GlobalMappingByModifiers;

    fn filter_by_timed_data(&self, timed_data: &Ti) -> Option<Bindings<'a, Mo, Ev>>;
}
