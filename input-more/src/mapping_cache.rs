use core::hash::Hash;
use core::marker::PhantomData;

use input_core::{
    Modifiers, PointerChangeEventData, TimedClickExactEventData, TimedLongPressEventData,
    TimedReleaseEventData,
};

use crate::{Binding, SwitchMappingByModifiers, SwitchMappingBySwitch, SwitchMappingCache};

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
    Sw: Clone + Eq + Hash,
    Mo: Clone + Eq + Hash,
    Ev: Clone,
{
    pub fn from_bindings<'a>(mapping: impl IntoIterator<Item = &'a Binding<Sw, Tr, Mo, Ev>>) -> Self
    where
        Sw: 'a,
        Tr: 'a,
        Ev: 'a,
        Mo: 'a,
    {
        let mut press = Vec::new();
        let mut release = Vec::new();
        let mut long_press = Vec::new();
        let mut click_exact = Vec::new();
        for binding in mapping.into_iter() {
            match binding {
                Binding::Press(binding) => press.push(binding.clone()),
                Binding::Release(binding) => release.push(binding.clone()),
                Binding::LongPress(binding) => long_press.push(binding.clone()),
                Binding::ClickExact(binding) => click_exact.push(binding.clone()),
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

impl<Sw, Tr, Mo, TdPr, TdRe, TdLo, TdCl, PdPr, PdRe, PrLo, PrCl, EvPr, EvRe, EvLo, EvCl>
    MappingCache<
        SwitchMappingCache<Sw, Mo, TdPr, PdPr, EvPr>,
        SwitchMappingCache<Sw, Mo, TdRe, PdRe, EvRe>,
        SwitchMappingCache<Sw, Mo, TdLo, PrLo, EvLo>,
        SwitchMappingCache<Sw, Mo, TdCl, PrCl, EvCl>,
        PhantomData<Tr>,
        (),
    >
{
    pub fn filter_by_switch<'a>(
        &'a self,
        switch: &Sw,
    ) -> Option<
        MappingCache<
            Option<SwitchMappingBySwitch<'a, Mo, TdPr, PdPr, EvPr>>,
            Option<SwitchMappingBySwitch<'a, Mo, TdRe, PdRe, EvRe>>,
            Option<SwitchMappingBySwitch<'a, Mo, TdLo, PrLo, EvLo>>,
            Option<SwitchMappingBySwitch<'a, Mo, TdCl, PrCl, EvCl>>,
            PhantomData<Tr>,
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
                trigger: PhantomData,
                moves: (),
            }),
        }
    }
}

impl<'a, Tr, Mo, TdPr, TdRe, TdLo, TdCl, PdPr, PdRe, PrLo, PrCl, EvPr, EvRe, EvLo, EvCl>
    MappingCache<
        Option<SwitchMappingBySwitch<'a, Mo, TdPr, PdPr, EvPr>>,
        Option<SwitchMappingBySwitch<'a, Mo, TdRe, PdRe, EvRe>>,
        Option<SwitchMappingBySwitch<'a, Mo, TdLo, PrLo, EvLo>>,
        Option<SwitchMappingBySwitch<'a, Mo, TdCl, PrCl, EvCl>>,
        PhantomData<Tr>,
        (),
    >
{
    pub fn filter_by_modifiers(
        &self,
        modifiers: &Modifiers<Mo>,
    ) -> Option<
        MappingCache<
            Option<SwitchMappingByModifiers<'a, Mo, TdPr, PdPr, EvPr>>,
            Option<SwitchMappingByModifiers<'a, Mo, TdRe, PdRe, EvRe>>,
            Option<SwitchMappingByModifiers<'a, Mo, TdLo, PrLo, EvLo>>,
            Option<SwitchMappingByModifiers<'a, Mo, TdCl, PrCl, EvCl>>,
            PhantomData<Tr>,
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
                trigger: PhantomData,
                moves: (),
            }),
        }
    }
}
