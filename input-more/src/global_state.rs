use core::hash::Hash;
use core::marker::PhantomData;
use std::collections::HashMap;

use input_core::{
    ClickExactHandleRequest, CoordsState, LongPressHandleRequest, Modifiers, PointerState,
    SchedulerState, TimedState,
};

use crate::{
    define_markers, define_struct_take_and_with_field, DeviceMappingCache, DeviceState,
    GlobalMappingCache, MappingModifiersCache, StructTakeField, StructWithField, SwitchBindings,
    SwitchEvent,
};

#[derive(Clone, Debug, Default)]
pub struct GlobalState<Mo, CsKe, CsMo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl, PoKe, PoMo> {
    pub modifiers: Mo,
    pub keyboard_coords_state: CsKe,
    pub mouse_coords_state: CsMo,
    pub keyboard_timed_state: TsKe,
    pub mouse_timed_state: TsMo,
    pub keyboard_long_press_scheduler: ShKeLo,
    pub keyboard_click_exact_scheduler: ShKeCl,
    pub mouse_long_press_scheduler: ShMoLo,
    pub mouse_click_exact_scheduler: ShMoCl,
    pub keyboard_pointer_state: PoKe,
    pub mouse_pointer_state: PoMo,
}

define_markers!(
    GlobalModifiersMarker,
    KeyboardCoordsStateMarker,
    MouseCoordsStateMarker,
    KeyboardTimedStateMarker,
    MouseTimedStateMarker,
    KeyboardLongPressSchedulerMarker,
    KeyboardClickExactSchedulerMarker,
    MouseLongPressSchedulerMarker,
    MouseClickExactSchedulerMarker,
    KeyboardPointerStateMarker,
    MousePointerStateMarker,
);

define_struct_take_and_with_field!(GlobalState {
    modifiers: Mo + GlobalModifiersMarker,
    keyboard_coords_state: CsKe + KeyboardCoordsStateMarker,
    mouse_coords_state: CsMo + MouseCoordsStateMarker,
    keyboard_timed_state: TsKe + KeyboardTimedStateMarker,
    mouse_timed_state: TsMo + MouseTimedStateMarker,
    keyboard_long_press_scheduler: ShKeLo + KeyboardLongPressSchedulerMarker,
    keyboard_click_exact_scheduler: ShKeCl + KeyboardClickExactSchedulerMarker,
    mouse_long_press_scheduler: ShMoLo + MouseLongPressSchedulerMarker,
    mouse_click_exact_scheduler: ShMoCl + MouseClickExactSchedulerMarker,
    keyboard_pointer_state: PoKe + KeyboardPointerStateMarker,
    mouse_pointer_state: PoMo + MousePointerStateMarker,
});

impl<Mo, CsKe, CsMo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl, PoKe, PoMo>
    GlobalState<Mo, CsKe, CsMo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl, PoKe, PoMo>
{
    pub fn new(
        modifiers: Mo,
        keyboard_coords_state: CsKe,
        mouse_coords_state: CsMo,
        keyboard_timed_state: TsKe,
        mouse_timed_state: TsMo,
        keyboard_long_press_scheduler: ShKeLo,
        keyboard_click_exact_scheduler: ShKeCl,
        mouse_long_press_scheduler: ShMoLo,
        mouse_click_exact_scheduler: ShMoCl,
        keyboard_pointer_state: PoKe,
        mouse_pointer_state: PoMo,
    ) -> Self {
        Self {
            modifiers,
            keyboard_coords_state,
            mouse_coords_state,
            keyboard_timed_state,
            mouse_timed_state,
            keyboard_long_press_scheduler,
            keyboard_click_exact_scheduler,
            mouse_long_press_scheduler,
            mouse_click_exact_scheduler,
            keyboard_pointer_state,
            mouse_pointer_state,
        }
    }

    pub fn take_state<Cs, Ts, Sh, Po, Re1, Ma1, Re2, Ma2, Re3, Ma3, Re4, Ma4, Re5, Ma5>(
        self,
    ) -> (DeviceState<Mo, Cs, Ts, Sh, Po>, Re5)
    where
        Self: StructTakeField<Mo, Ma1, Rest = Re1>,
        Re1: StructTakeField<Cs, Ma2, Rest = Re2>,
        Re2: StructTakeField<Ts, Ma3, Rest = Re3>,
        Re3: StructTakeField<Sh, Ma4, Rest = Re4>,
        Re4: StructTakeField<Po, Ma5, Rest = Re5>,
    {
        let (modifiers, rest) = self.take_field();
        let (coords_state, rest) = rest.take_field();
        let (timed_state, rest) = rest.take_field();
        let (scheduler, rest) = rest.take_field();
        let (pointer_state, rest) = rest.take_field();
        (
            DeviceState::new(
                modifiers,
                coords_state,
                timed_state,
                scheduler,
                pointer_state,
            ),
            rest,
        )
    }
}

impl<Mo, CsKe, CsMo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl, PoKe, PoMo>
    GlobalState<PhantomData<Mo>, CsKe, CsMo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl, PoKe, PoMo>
{
    pub fn with_state<Cs, Ts, Sh, Po, Re1, Ma1, Re2, Ma2, Re3, Ma3, Re4, Ma4, Re5, Ma5>(
        self,
        state: DeviceState<Mo, Cs, Ts, Sh, Po>,
    ) -> Re5
    where
        Self: StructWithField<Mo, Ma1, Output = Re1>,
        Re1: StructWithField<Cs, Ma2, Output = Re2>,
        Re2: StructWithField<Ts, Ma3, Output = Re3>,
        Re3: StructWithField<Sh, Ma4, Output = Re4>,
        Re4: StructWithField<Po, Ma5, Output = Re5>,
    {
        self.with_field(state.modifiers)
            .with_field(state.coords_state)
            .with_field(state.timed_state)
            .with_field(state.scheduler)
            .with_field(state.pointer_state)
    }
}

impl<Mo, Ti, KeSw, MsSw, KeCo, MsCo>
    GlobalState<
        Modifiers<Mo>,
        CoordsState<KeCo>,
        CoordsState<MsCo>,
        TimedState<KeSw>,
        TimedState<MsSw>,
        SchedulerState<Ti, (SwitchEvent<Ti, KeSw>, Modifiers<Mo>, KeCo), LongPressHandleRequest>,
        SchedulerState<Ti, (SwitchEvent<Ti, KeSw>, Modifiers<Mo>, KeCo), ClickExactHandleRequest>,
        SchedulerState<Ti, (SwitchEvent<Ti, MsSw>, Modifiers<Mo>, MsCo), LongPressHandleRequest>,
        SchedulerState<Ti, (SwitchEvent<Ti, MsSw>, Modifiers<Mo>, MsCo), ClickExactHandleRequest>,
        PointerState<KeSw, KeCo>,
        PointerState<MsSw, MsCo>,
    >
{
    pub fn with_timeout<
        'a,
        KeTr,
        MsTr,
        MsMa,
        KeEvPr,
        KeEvRe,
        KeEvLo,
        KeEvCl,
        MsEvPr,
        MsEvRe,
        MsEvLo,
        MsEvCl,
    >(
        time: Ti,
        mapping: &'a GlobalMappingCache<
            DeviceMappingCache<KeSw, KeTr, Mo, KeEvPr, KeEvRe, KeEvLo, KeEvCl>,
            DeviceMappingCache<MsSw, MsTr, Mo, MsEvPr, MsEvRe, MsEvLo, MsEvCl>,
            MappingModifiersCache<Mo>,
        >,
    ) -> GlobalStateWithTimeoutResult<
        'a,
        Self,
        Mo,
        KeEvPr,
        KeEvRe,
        KeEvLo,
        KeEvCl,
        KeCo,
        MsEvPr,
        MsEvRe,
        MsEvLo,
        MsEvCl,
        MsCo,
    > {
        todo!();
    }
}

impl<Mo, Ti, Sw, Co, CsMo, TsMo, ShKeCl, ShMoLo, ShMoCl, PoMo>
    GlobalState<
        Modifiers<Mo>,
        CoordsState<Co>,
        CsMo,
        TimedState<Sw>,
        TsMo,
        SchedulerState<Ti, (SwitchEvent<Ti, Sw>, Modifiers<Mo>, Co), LongPressHandleRequest>,
        ShKeCl,
        ShMoLo,
        ShMoCl,
        PointerState<Sw, Co>,
        PoMo,
    >
{
    pub fn with_keyboard_press_event<'a, Tr, MsMa, EvPr, EvRe, EvLo, EvCl>(
        self,
        event: SwitchEvent<Ti, Sw>,
        mapping: &'a GlobalMappingCache<
            DeviceMappingCache<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl>,
            MsMa,
            MappingModifiersCache<Mo>,
        >,
    ) -> GlobalStateWithEventResult<'a, Self, Ti, Mo, EvPr, Co>
    where
        Sw: Clone + Eq + Hash,
        Mo: Clone + Eq + From<Sw> + Hash + Ord,
        Ti: Clone + Ord,
        Co: Clone,
    {
        let (state, global_state): (
            DeviceState<
                Modifiers<Mo>,
                CoordsState<Co>,
                TimedState<Sw>,
                SchedulerState<
                    Ti,
                    (SwitchEvent<Ti, Sw>, Modifiers<Mo>, Co),
                    LongPressHandleRequest,
                >,
                PointerState<Sw, Co>,
            >,
            _,
        ) = self.take_state();
        let (state, scheduled, bindings) =
            state.with_press_event(event, mapping.keyboard(), mapping.modifiers());

        GlobalStateWithEventResult {
            state: global_state.with_state(state),
            scheduled,
            bindings,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GlobalStateWithEventResult<'a, Gl, Ti, Mo, Ev, Co> {
    pub state: Gl,
    pub scheduled: Option<Ti>,
    pub bindings: Option<(SwitchBindings<'a, Mo, Ev>, Co)>,
}

#[derive(Clone, Debug)]
pub struct GlobalStateWithTimeoutResult<
    'a,
    Gl,
    Mo,
    KeEvPr,
    KeEvRe,
    KeEvLo,
    KeEvCl,
    KeCo,
    MsEvPr,
    MsEvRe,
    MsEvLo,
    MsEvCl,
    MsCo,
> {
    pub state: Gl,
    pub keyboard_press: Vec<(SwitchBindings<'a, Mo, KeEvPr>, KeCo)>,
    pub keyboard_release: Vec<(SwitchBindings<'a, Mo, KeEvRe>, KeCo)>,
    pub keyboard_long_press: Vec<(SwitchBindings<'a, Mo, KeEvLo>, KeCo)>,
    pub keyboard_click_exact: Vec<(SwitchBindings<'a, Mo, KeEvCl>, KeCo)>,
    pub mouse_press: Vec<(SwitchBindings<'a, Mo, MsEvPr>, MsCo)>,
    pub mouse_release: Vec<(SwitchBindings<'a, Mo, MsEvRe>, MsCo)>,
    pub mouse_long_press: Vec<(SwitchBindings<'a, Mo, MsEvLo>, MsCo)>,
    pub mouse_click_exact: Vec<(SwitchBindings<'a, Mo, MsEvCl>, MsCo)>,
}

#[test]
fn test1() {
    let global_state = GlobalState::new(1, (1, 2, 3), (), false, (), "123", (), (), (), (1, 2), ());

    let (state, global_state): (DeviceState<i32, (i8, i8, i8), bool, &str, (u8, u8)>, _) =
        global_state.take_state();
    let state = DeviceState {
        modifiers: state.modifiers + 10,
        coords_state: (
            state.coords_state.1,
            state.coords_state.2,
            state.coords_state.0,
        ),
        timed_state: !state.timed_state,
        scheduler: &state.scheduler[1..3],
        pointer_state: (state.pointer_state.1, state.pointer_state.0),
    };
    let global_state = global_state.with_state(state);

    assert_eq!(global_state.modifiers, 11);
    assert_eq!(global_state.keyboard_coords_state, (2, 3, 1));
    assert_eq!(global_state.keyboard_timed_state, true);
    assert_eq!(global_state.keyboard_long_press_scheduler, "23");
    assert_eq!(global_state.keyboard_pointer_state, (2, 1));
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
