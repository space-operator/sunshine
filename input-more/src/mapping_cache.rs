use core::hash::Hash;
use core::marker::PhantomData;

use input_core::{
    PointerChangeEventData, TimedClickExactEventData, TimedLongPressEventData,
    TimedReleaseEventData,
};

use crate::{Binding, DeviceMapping, SwitchMappingBySwitch, SwitchMappingCache};

#[derive(Clone, Debug)]
pub struct MappingCache<Pr, Re, Lo, Cl, Tr, Mo> {
    pub press: Pr,
    pub release: Re,
    pub long_press: Lo,
    pub click_exact: Cl,
    pub trigger: Tr,
    pub moves: Mo,
}

pub type DeviceMappingCache<Sw, Tr, Mo, Ev> = MappingCache<
    SwitchMappingCache<Sw, Mo, (), (), Ev>,
    SwitchMappingCache<Sw, Mo, Option<TimedReleaseEventData>, Option<PointerChangeEventData>, Ev>,
    SwitchMappingCache<Sw, Mo, TimedLongPressEventData, (), Ev>,
    SwitchMappingCache<Sw, Mo, TimedClickExactEventData, (), Ev>,
    PhantomData<Tr>,
    (),
>;

impl<Sw, Tr, Mo, Ev> DeviceMappingCache<Sw, Tr, Mo, Ev>
where
    Sw: Eq + Hash,
    Mo: Eq + Hash,
{
    pub fn from_mapping(mapping: DeviceMapping<Sw, Tr, Mo, Ev>) -> Self {
        let mut press = Vec::new();
        let mut release = Vec::new();
        let mut long_press = Vec::new();
        let mut click_exact = Vec::new();
        for binding in mapping.into_bindings() {
            match binding {
                Binding::Press(binding) => press.push(binding),
                Binding::Release(binding) => release.push(binding),
                Binding::LongPress(binding) => long_press.push(binding),
                Binding::ClickExact(binding) => click_exact.push(binding),
                Binding::Trigger(_) => todo!(),
                Binding::Move(_) => todo!(),
            }
        }
        Self {
            press: SwitchMappingCache::from_bindings(press),
            release: SwitchMappingCache::from_bindings(release),
            long_press: SwitchMappingCache::from_bindings(long_press),
            click_exact: SwitchMappingCache::from_bindings(click_exact),
            trigger: PhantomData,
            moves: (),
        }
    }
}

impl<Sw, Mo, TdPr, TdRe, TdLo, TdCl, PdPr, PdRe, PrLo, PrCl, Ev>
    MappingCache<
        SwitchMappingCache<Sw, Mo, TdPr, PdPr, Ev>,
        SwitchMappingCache<Sw, Mo, TdRe, PdRe, Ev>,
        SwitchMappingCache<Sw, Mo, TdLo, PrLo, Ev>,
        SwitchMappingCache<Sw, Mo, TdCl, PrCl, Ev>,
        (),
        (),
    >
where
    Sw: Eq + Hash,
    Mo: Eq + Hash,
{
    pub fn filter_by_switch(
        &self,
        switch: &Sw,
    ) -> Option<
        MappingCache<
            Option<SwitchMappingBySwitch<'_, Mo, TdPr, PdPr, Ev>>,
            Option<SwitchMappingBySwitch<'_, Mo, TdRe, PdRe, Ev>>,
            Option<SwitchMappingBySwitch<'_, Mo, TdLo, PrLo, Ev>>,
            Option<SwitchMappingBySwitch<'_, Mo, TdCl, PrCl, Ev>>,
            (),
            (),
        >,
    >
    where
        Sw: Eq + Hash,
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
                moves: (),
            }),
        }
    }
}
