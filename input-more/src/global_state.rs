use core::marker::PhantomData;

use crate::{
    define_markers, define_struct_take_and_with_field, State, StructTakeField, StructWithField,
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

    pub fn take_state<Ts, Sh, Re1, Ma1, Re2, Ma2, Re3, Ma3>(self) -> (State<Mo, Ts, Sh>, Re3)
    where
        Self: StructTakeField<Mo, Ma1, Rest = Re1>,
        Re1: StructTakeField<Ts, Ma2, Rest = Re2>,
        Re2: StructTakeField<Sh, Ma3, Rest = Re3>,
    {
        let (modifiers, rest) = self.take_field();
        let (timed_state, rest) = rest.take_field();
        let (scheduler, rest) = rest.take_field();
        (State::new(modifiers, timed_state, scheduler), rest)
    }
}

impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    GlobalState<PhantomData<Mo>, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
{
    pub fn with_state<Ts, Sh, Re1, Ma1, Re2, Ma2, Re3, Ma3>(self, state: State<Mo, Ts, Sh>) -> Re3
    where
        Self: StructWithField<Mo, Ma1, Output = Re1>,
        Re1: StructWithField<Ts, Ma2, Output = Re2>,
        Re2: StructWithField<Sh, Ma3, Output = Re3>,
    {
        self.with_field(state.modifiers)
            .with_field(state.timed_state)
            .with_field(state.scheduler)
    }
}

#[test]
fn test1() {
    let global_state = GlobalState::new(1, false, (), "123", (), (), ());

    let (state, global_state): (State<i32, bool, &str>, _) = global_state.take_state();
    let state = State {
        modifiers: state.modifiers + 10,
        timed_state: !state.timed_state,
        scheduler: &state.scheduler[1..3],
    };
    let global_state = global_state.with_state(state);

    assert_eq!(global_state.modifiers, 11);
    assert_eq!(global_state.keyboard_timed_state, true);
    assert_eq!(global_state.keyboard_long_press_scheduler, "23");
}

/*
#[test]
fn test2() {
    let mut global_state = GlobalState::new(1, false, (), "123", (), (), ());

    let (state, global_state): (State<i32, bool, &str>, _) = global_state.take_state();
    let state = State {
        modifiers: state.modifiers + 10,
        timed_state: !state.timed_state,
        scheduler: &state.scheduler[1..3],
    };
    global_state.with_field(state.modifiers);
}*/
