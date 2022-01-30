use crate::{DeviceMappingCache, ModifiersCache};

#[derive(Clone, Debug)]
pub struct GlobalMappingCache<Mo, KeSw, KeTr, KeEv> {
    pub keyboard: DeviceMappingCache<KeSw, KeTr, Mo, KeEv>,
    pub modifiers: ModifiersCache<Mo>,
}
