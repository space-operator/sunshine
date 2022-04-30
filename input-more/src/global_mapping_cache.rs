use core::hash::Hash;

use crate::{DeviceMappingCache, GlobalMapping, Mapping, MappingModifiersCache};

/// A generic structure that stores input mapping cache
/// for keyboard and mouse devices and the corresponding modifiers cache.
#[derive(Clone, Debug)]
pub struct GlobalMappingCache<Ke, Ms, Mo> {
    keyboard: Ke,
    mouse: Ms,
    modifiers: Mo,
}

impl<Ke, Ms, Mo> GlobalMappingCache<Ke, Ms, Mo> {
    /// Returns a mapping cache for keyboard device.
    pub fn keyboard(&self) -> &Ke {
        &self.keyboard
    }

    /// Returns a mapping cache for mouse device.
    pub fn mouse(&self) -> &Ms {
        &self.mouse
    }

    /// Returns a modifiers cache.
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
    KeTr: Clone + Eq + Hash,
    MsTr: Clone + Eq + Hash,
    KeEv: Clone,
    MsEv: Clone,
{
    /// Builds `GlobalMappingCache` structure from a given `GlobalMapping`.
    pub fn from_mapping(
        mapping: GlobalMapping<Mapping<KeSw, KeTr, Mo, KeEv>, Mapping<MsSw, MsTr, Mo, MsEv>>,
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
