use crate::{
    define_markers, define_struct_from_into_cons_and_take_put, define_struct_take_and_with_field,
};

#[derive(Clone, Debug, Default)]
pub struct GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl> {
    pub modifiers: Mo,
    pub keyboard_timed_state: TsKe,
    pub mouse_timed_state: TsMo,
    pub keyboard_long_press_scheduler: ShKeLo,
    pub keyboard_click_exact_scheduler: ShKeCl,
    pub mouse_long_press_scheduler: ShMoLo,
    pub mouse_click_exact_scheduler: ShMoCl,
}

define_markers!(
    GlobalModifiersMarker,
    KeyboardTimedStateMarker,
    MouseTimedStateMarker,
    KeyboardLongPressSchedulerMarker,
    KeyboardClickExactSchedulerMarker,
    MouseLongPressSchedulerMarker,
    MouseClickExactSchedulerMarker,
);

define_struct_take_and_with_field!(GlobalState {
    modifiers: Mo + GlobalModifiersMarker,
    keyboard_timed_state: TsKe + KeyboardTimedStateMarker,
    mouse_timed_state: TsMo + MouseTimedStateMarker,
    keyboard_long_press_scheduler: ShKeLo + KeyboardLongPressSchedulerMarker,
    keyboard_click_exact_scheduler: ShKeCl + KeyboardClickExactSchedulerMarker,
    mouse_long_press_scheduler: ShMoLo + MouseLongPressSchedulerMarker,
    mouse_click_exact_scheduler: ShMoCl + MouseClickExactSchedulerMarker,
});

define_struct_from_into_cons_and_take_put!(
    GlobalState,
    modifiers: Mo + GlobalModifiersMarker,
    keyboard_timed_state: TsKe + KeyboardTimedStateMarker,
    mouse_timed_state: TsMo + MouseTimedStateMarker,
    keyboard_long_press_scheduler: ShKeLo + KeyboardLongPressSchedulerMarker,
    keyboard_click_exact_scheduler: ShKeCl + KeyboardClickExactSchedulerMarker,
    mouse_long_press_scheduler: ShMoLo + MouseLongPressSchedulerMarker,
    mouse_click_exact_scheduler: ShMoCl + MouseClickExactSchedulerMarker,
);

impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
{
    pub fn new(
        modifiers: Mo,
        keyboard_timed_state: TsKe,
        mouse_timed_state: TsMo,
        keyboard_long_press_scheduler: ShKeLo,
        keyboard_click_exact_scheduler: ShKeCl,
        mouse_long_press_scheduler: ShMoLo,
        mouse_click_exact_scheduler: ShMoCl,
    ) -> Self {
        Self {
            modifiers,
            keyboard_timed_state,
            mouse_timed_state,
            keyboard_long_press_scheduler,
            keyboard_click_exact_scheduler,
            mouse_long_press_scheduler,
            mouse_click_exact_scheduler,
        }
    }
}
