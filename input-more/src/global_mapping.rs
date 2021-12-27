use crate::{define_markers, define_struct_from_into_cons_and_take_put};

#[derive(Clone, Debug, Default)]
pub struct GlobalMapping<KePe, KeRe, KeLo, KeCl> {
    pub keyboard_press: KePe,
    pub keyboard_release: KeRe,
    pub keyboard_long_press: KeLo,
    pub keyboard_click_exact: KeCl,
}

define_markers!(
    KeyboardPressMappingMarker,
    KeyboardReleaseMappingMarker,
    KeyboardLongPressMappingMarker,
    KeyboardClickExactMappingMarker,
);

define_struct_from_into_cons_and_take_put!(
    GlobalMapping,
    keyboard_press: KePe + KeyboardPressMappingMarker,
    keyboard_release: KeRe + KeyboardReleaseMappingMarker,
    keyboard_long_press: KeLo + KeyboardLongPressMappingMarker,
    keyboard_click_exact: KeCl + KeyboardClickExactMappingMarker,
);

impl<KePe, KeRe, KeLo, KeCl> GlobalMapping<KePe, KeRe, KeLo, KeCl> {
    pub fn new(
        keyboard_press: KePe,
        keyboard_release: KeRe,
        keyboard_long_press: KeLo,
        keyboard_click_exact: KeCl,
    ) -> Self {
        Self {
            keyboard_press,
            keyboard_release,
            keyboard_long_press,
            keyboard_click_exact,
        }
    }
}
