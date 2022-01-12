use core::hash::Hash;
use std::sync::Arc;

use crate::{DeviceMapping, DeviceMappingCache, GlobalMapping, MappingModifiersCache};

#[derive(Clone, Debug)]
pub struct GlobalMappingCache<Ke, Ms, Mo> {
    keyboard: Ke,
    mouse: Ms,
    modifiers: Mo,
}

impl<Ke, Ms, Mo> GlobalMappingCache<Ke, Ms, Mo> {
    pub fn keyboard(&self) -> &Ke {
        &self.keyboard
    }

    pub fn mouse(&self) -> &Ms {
        &self.mouse
    }

    pub fn modifiers(&self) -> &Mo {
        &self.modifiers
    }
}

impl<
        Mo,
        KeSw,
        KeTr,
        KeEvPr,
        KeEvRe,
        KeEvLo,
        KeEvCl,
        MsSw,
        MsTr,
        MsEvPr,
        MsEvRe,
        MsEvLo,
        MsEvCl,
    >
    GlobalMappingCache<
        DeviceMappingCache<KeSw, KeTr, Mo, KeEvPr, KeEvRe, KeEvLo, KeEvCl>,
        DeviceMappingCache<MsSw, MsTr, Mo, MsEvPr, MsEvRe, MsEvLo, MsEvCl>,
        MappingModifiersCache<Mo>,
    >
where
    Mo: Clone + Eq + Hash,
    KeSw: Clone + Eq + Hash,
    MsSw: Clone + Eq + Hash,
    KeEvPr: Clone,
    KeEvRe: Clone,
    KeEvLo: Clone,
    KeEvCl: Clone,
    MsEvPr: Clone,
    MsEvRe: Clone,
    MsEvLo: Clone,
    MsEvCl: Clone,
{
    pub fn from_mapping<KeEvTr, KeEvCo, MsEvTr, MsEvCo>(
        mapping: GlobalMapping<
            DeviceMapping<KeSw, KeTr, Mo, KeEvPr, KeEvRe, KeEvLo, KeEvCl, KeEvTr, KeEvCo>,
            DeviceMapping<MsSw, MsTr, Mo, MsEvPr, MsEvRe, MsEvLo, MsEvCl, MsEvTr, MsEvCo>,
        >,
    ) -> Self {
        let keyboard_modifiers = mapping
            .keyboard
            .bindings()
            .iter()
            .map(|binding| binding.modifiers().switches())
            .flat_map(|switch| switch.iter());
        let mouse_modifiers = mapping
            .mouse
            .bindings()
            .iter()
            .map(|binding| binding.modifiers().switches())
            .flat_map(|switch| switch.iter());
        Self {
            keyboard: DeviceMappingCache::from_bindings(mapping.keyboard.bindings()),
            mouse: DeviceMappingCache::from_bindings(mapping.mouse.bindings()),
            modifiers: MappingModifiersCache::from_switches(
                keyboard_modifiers.chain(mouse_modifiers).cloned(),
            ),
        }
    }
}
