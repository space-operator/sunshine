use core::hash::Hash;

use input_core::{
    Modifiers, PointerMoveEventData, PointerReleaseEventData, TimedClickExactEventData,
    TimedLongPressEventData, TimedReleaseEventData,
};

use crate::{
    Binding, CoordsMappingCache, SwitchMappingByModifiers, SwitchMappingBySwitch,
    SwitchMappingCache, TriggerMappingCache,
};

/// A generic structure that stores input mapping cache.
///
/// The structure is used to speed up and optimize the input bindings lookup.
#[derive(Clone, Debug)]
pub struct MappingCache<Pr, Re, Lo, Cl, Tr, Co> {
    /// Mapping cache for press events.
    pub press: Pr,
    /// Mapping cache for release events.
    pub release: Re,
    /// Mapping cache for long press events.
    pub long_press: Lo,
    /// Mapping cache for click exact events.
    pub click_exact: Cl,
    /// Mapping cache for trigger events.
    pub trigger: Tr,
    /// Mapping cache for move events.
    pub coords: Co,
}

/// A structure that stores input mapping cache for a specified device.
pub type DeviceMappingCache<Sw, Tr, Mo, Ev> = MappingCache<
    SwitchMappingCache<Sw, Mo, (), (), Ev>,
    SwitchMappingCache<Sw, Mo, Option<TimedReleaseEventData>, Option<PointerReleaseEventData>, Ev>,
    SwitchMappingCache<Sw, Mo, TimedLongPressEventData, (), Ev>,
    SwitchMappingCache<Sw, Mo, TimedClickExactEventData, (), Ev>,
    TriggerMappingCache<Tr, Mo, Ev>,
    CoordsMappingCache<PointerMoveEventData<Sw>, Mo, Ev>,
>;

impl<Sw, Tr, Mo, Ev> DeviceMappingCache<Sw, Tr, Mo, Ev> {
    /// Builds `DeviceMappingCache` structure from specified device mapping.
    pub fn from_bindings<'a>(mapping: impl IntoIterator<Item = &'a Binding<Sw, Tr, Mo, Ev>>) -> Self
    where
        Sw: 'a + Clone + Eq + Hash,
        Tr: 'a + Clone + Eq + Hash,
        Mo: 'a + Clone + Eq + Hash,
        Ev: 'a + Clone,
    {
        let mut press = Vec::new();
        let mut release = Vec::new();
        let mut long_press = Vec::new();
        let mut click_exact = Vec::new();
        let mut trigger = Vec::new();
        let mut coords = Vec::new();
        for binding in mapping.into_iter() {
            match binding {
                Binding::Press(binding) => press.push(binding.clone()),
                Binding::Release(binding) => release.push(binding.clone()),
                Binding::LongPress(binding) => long_press.push(binding.clone()),
                Binding::ClickExact(binding) => click_exact.push(binding.clone()),
                Binding::Trigger(binding) => trigger.push(binding.clone()),
                Binding::Coords(binding) => coords.push(binding.clone()),
            }
        }
        Self {
            press: SwitchMappingCache::from_bindings(press),
            release: SwitchMappingCache::from_bindings(release),
            long_press: SwitchMappingCache::from_bindings(long_press),
            click_exact: SwitchMappingCache::from_bindings(click_exact),
            trigger: TriggerMappingCache::from_bindings(trigger),
            coords: CoordsMappingCache::from_bindings(coords),
        }
    }
}

impl<Sw, Mo, TdPr, TdRe, TdLo, TdCl, PdPr, PdRe, PrLo, PrCl, Ev, TrCa, CoCa>
    MappingCache<
        SwitchMappingCache<Sw, Mo, TdPr, PdPr, Ev>,
        SwitchMappingCache<Sw, Mo, TdRe, PdRe, Ev>,
        SwitchMappingCache<Sw, Mo, TdLo, PrLo, Ev>,
        SwitchMappingCache<Sw, Mo, TdCl, PrCl, Ev>,
        TrCa,
        CoCa,
    >
{
    /// Filters `DeviceMappingCache` by a specified switch.
    ///
    /// If there are no bindings for the given switch, the method returns None.
    /// Otherwise, the method returns a temporary structure
    /// with matching bindings intended for further filtering.
    pub fn filter_by_switch<'a>(
        &'a self,
        switch: &Sw,
    ) -> Option<
        MappingCache<
            Option<SwitchMappingBySwitch<'a, Mo, TdPr, PdPr, Ev>>,
            Option<SwitchMappingBySwitch<'a, Mo, TdRe, PdRe, Ev>>,
            Option<SwitchMappingBySwitch<'a, Mo, TdLo, PrLo, Ev>>,
            Option<SwitchMappingBySwitch<'a, Mo, TdCl, PrCl, Ev>>,
            (),
            (),
        >,
    >
    where
        Sw: Eq + Hash,
        Mo: Eq + Hash,
    {
        let press = self.press.filter_by_switch(switch);
        let release = self.release.filter_by_switch(switch);
        let long_press = self.long_press.filter_by_switch(switch);
        let click_exact = self.click_exact.filter_by_switch(switch);
        match (&press, &release, &long_press, &click_exact) {
            (None, None, None, None) => None,
            _ => Some(MappingCache {
                press,
                release,
                long_press,
                click_exact,
                trigger: (),
                coords: (),
            }),
        }
    }
}

impl<'a, Mo, TdPr, TdRe, TdLo, TdCl, PdPr, PdRe, PrLo, PrCl, TrCa, CoCa, Ev>
    MappingCache<
        Option<SwitchMappingBySwitch<'a, Mo, TdPr, PdPr, Ev>>,
        Option<SwitchMappingBySwitch<'a, Mo, TdRe, PdRe, Ev>>,
        Option<SwitchMappingBySwitch<'a, Mo, TdLo, PrLo, Ev>>,
        Option<SwitchMappingBySwitch<'a, Mo, TdCl, PrCl, Ev>>,
        TrCa,
        CoCa,
    >
{
    /// Filters `DeviceMappingCache` by a currently enabled/pressed modifiers.
    ///
    /// If there are no bindings for the given switch, the method returns None.
    /// Otherwise, the method returns a temporary structure
    /// with matching bindings intended for further filtering.
    pub fn filter_by_modifiers(
        &self,
        modifiers: &Modifiers<Mo>,
    ) -> Option<
        MappingCache<
            Option<SwitchMappingByModifiers<'a, Mo, TdPr, PdPr, Ev>>,
            Option<SwitchMappingByModifiers<'a, Mo, TdRe, PdRe, Ev>>,
            Option<SwitchMappingByModifiers<'a, Mo, TdLo, PrLo, Ev>>,
            Option<SwitchMappingByModifiers<'a, Mo, TdCl, PrCl, Ev>>,
            (),
            (),
        >,
    >
    where
        Mo: Eq + Hash + Ord,
    {
        let press = self
            .press
            .as_ref()
            .and_then(|mapping| mapping.filter_by_modifiers(modifiers));
        let release = self
            .release
            .as_ref()
            .and_then(|mapping| mapping.filter_by_modifiers(modifiers));
        let long_press = self
            .long_press
            .as_ref()
            .and_then(|mapping| mapping.filter_by_modifiers(modifiers));
        let click_exact = self
            .click_exact
            .as_ref()
            .and_then(|mapping| mapping.filter_by_modifiers(modifiers));
        match (&press, &release, &long_press, &click_exact) {
            (None, None, None, None) => None,
            _ => Some(MappingCache {
                press,
                release,
                long_press,
                click_exact,
                trigger: (),
                coords: (),
            }),
        }
    }
}
