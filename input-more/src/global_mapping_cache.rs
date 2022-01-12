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

impl<Mo, KeSw, KeTr, KeEv, MsSw, MsTr, MsEv>
    GlobalMappingCache<
        DeviceMappingCache<KeSw, KeTr, Mo, KeEv>,
        DeviceMappingCache<MsSw, MsTr, Mo, MsEv>,
        MappingModifiersCache<Mo>,
    >
where
    Mo: Clone + Eq + Hash,
    KeSw: Clone + Eq + Hash,
    MsSw: Clone + Eq + Hash,
    KeEv: Clone,
    MsEv: Clone,
{
    pub fn from_mapping<KeEvTr, KeEvCo, MsEvTr, MsEvCo>(
        mapping: GlobalMapping<
            DeviceMapping<KeSw, KeTr, Mo, KeEv>,
            DeviceMapping<MsSw, MsTr, Mo, MsEv>,
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
