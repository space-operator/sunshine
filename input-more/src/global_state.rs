use core::hash::Hash;

use input_core::{
    ClickExactHandleRequest, CoordsState, LongPressHandleRequest, Modifiers, PointerState,
    SchedulerState, TimedState,
};

use crate::{
    CoordsEvent, DeviceMappingCache, DeviceSchedulerState, DeviceState, FilteredBindings,
    GlobalMappingCache, MappingModifiersCache, SwitchEvent, TriggerEvent,
};

#[derive(Clone, Debug, Default)]
pub struct GlobalState<Mo, CsKe, CsMs, TsKe, TsMs, ShKeLo, ShKeCl, ShMsLo, ShMsCl, PoKe, PoMs> {
    pub modifiers: Mo,
    pub keyboard_coords_state: CsKe,
    pub mouse_coords_state: CsMs,
    pub keyboard_timed_state: TsKe,
    pub mouse_timed_state: TsMs,
    pub keyboard_long_press_scheduler: ShKeLo,
    pub keyboard_click_exact_scheduler: ShKeCl,
    pub mouse_long_press_scheduler: ShMsLo,
    pub mouse_click_exact_scheduler: ShMsCl,
    pub keyboard_pointer_state: PoKe,
    pub mouse_pointer_state: PoMs,
}

/*
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
    mouse_coords_state: CsMs + MouseCoordsStateMarker,
    keyboard_timed_state: TsKe + KeyboardTimedStateMarker,
    mouse_timed_state: TsMs + MouseTimedStateMarker,
    keyboard_long_press_scheduler: ShKeLo + KeyboardLongPressSchedulerMarker,
    keyboard_click_exact_scheduler: ShKeCl + KeyboardClickExactSchedulerMarker,
    mouse_long_press_scheduler: ShMsLo + MouseLongPressSchedulerMarker,
    mouse_click_exact_scheduler: ShMsCl + MouseClickExactSchedulerMarker,
    keyboard_pointer_state: PoKe + KeyboardPointerStateMarker,
    mouse_pointer_state: PoMs + MousePointerStateMarker,
});
*/

impl<Mo, CsKe, CsMs, TsKe, TsMs, ShKeLo, ShKeCl, ShMsLo, ShMsCl, PoKe, PoMs>
    GlobalState<Mo, CsKe, CsMs, TsKe, TsMs, ShKeLo, ShKeCl, ShMsLo, ShMsCl, PoKe, PoMs>
{
    pub fn new(
        modifiers: Mo,
        keyboard_coords_state: CsKe,
        mouse_coords_state: CsMs,
        keyboard_timed_state: TsKe,
        mouse_timed_state: TsMs,
        keyboard_long_press_scheduler: ShKeLo,
        keyboard_click_exact_scheduler: ShKeCl,
        mouse_long_press_scheduler: ShMsLo,
        mouse_click_exact_scheduler: ShMsCl,
        keyboard_pointer_state: PoKe,
        mouse_pointer_state: PoMs,
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

    /*
        pub fn take_state<
            Cs,
            Ts,
            ShLo,
            ShCl,
            Po,
            Re1,
            Ma1,
            Re2,
            Ma2,
            Re3,
            Ma3,
            Re4,
            Ma4,
            Re5,
            Ma5,
            Re6,
            Ma6,
        >(
            self,
        ) -> (DeviceState<Mo, Cs, Ts, ShLo, ShCl, Po>, Re6)
        where
            Self: StructTakeField<Mo, Ma1, Rest = Re1>,
            Re1: StructTakeField<Cs, Ma2, Rest = Re2>,
            Re2: StructTakeField<Ts, Ma3, Rest = Re3>,
            Re3: StructTakeField<ShLo, Ma4, Rest = Re4>,
            Re4: StructTakeField<ShCl, Ma5, Rest = Re5>,
            Re5: StructTakeField<Po, Ma6, Rest = Re6>,
        {
            let (modifiers, rest) = self.take_field();
            let (coords_state, rest) = rest.take_field();
            let (timed_state, rest) = rest.take_field();
            let (press_scheduler, rest) = rest.take_field();
            let (release_scheduler, rest) = rest.take_field();
            let (pointer_state, rest) = rest.take_field();
            (
                DeviceState::new(
                    modifiers,
                    coords_state,
                    timed_state,
                    press_scheduler,
                    release_scheduler,
                    pointer_state,
                ),
                rest,
            )
        }
    }

    impl<Mo, CsKe, CsMs, TsKe, TsMs, ShKeLo, ShKeCl, ShMsLo, ShMsCl, PoKe, PoMs>
        GlobalState<PhantomData<Mo>, CsKe, CsMs, TsKe, TsMs, ShKeLo, ShKeCl, ShMsLo, ShMsCl, PoKe, PoMs>
    {
        pub fn with_state<
            Cs,
            Ts,
            ShLo,
            ShCl,
            Po,
            Re1,
            Ma1,
            Re2,
            Ma2,
            Re3,
            Ma3,
            Re4,
            Ma4,
            Re5,
            Ma5,
            Re6,
            Ma6,
        >(
            self,
            state: DeviceState<Mo, Cs, Ts, ShLo, ShCl, Po>,
        ) -> Re6
        where
            Self: StructWithField<Mo, Ma1, Output = Re1>,
            Re1: StructWithField<Cs, Ma2, Output = Re2>,
            Re2: StructWithField<Ts, Ma3, Output = Re3>,
            Re3: StructWithField<ShLo, Ma4, Output = Re4>,
            Re4: StructWithField<ShCl, Ma5, Output = Re5>,
            Re5: StructWithField<Po, Ma6, Output = Re6>,
        {
            self.with_field(state.modifiers)
                .with_field(state.coords_state)
                .with_field(state.timed_state)
                .with_field(state.long_press_scheduler)
                .with_field(state.click_exact_scheduler)
                .with_field(state.pointer_state)
        }
        */
}

impl<Mo, Ti, KeSw, MsSw, KeCo, MsCo>
    GlobalState<
        Modifiers<Mo>,
        CoordsState<KeCo>,
        CoordsState<MsCo>,
        TimedState<KeSw>,
        TimedState<MsSw>,
        DeviceSchedulerState<Ti, KeSw, Mo, KeCo, LongPressHandleRequest>,
        DeviceSchedulerState<Ti, KeSw, Mo, KeCo, ClickExactHandleRequest>,
        DeviceSchedulerState<Ti, MsSw, Mo, MsCo, LongPressHandleRequest>,
        DeviceSchedulerState<Ti, MsSw, Mo, MsCo, ClickExactHandleRequest>,
        PointerState<KeSw, KeCo>,
        PointerState<MsSw, MsCo>,
    >
{
    pub fn with_timeout<'a, KeTr, MsTr, KeEv, MsEv>(
        &mut self,
        time_minus_long_press_duration: Ti,
        time_minus_click_exact_duration: Ti,
        mapping: &'a GlobalMappingCache<
            DeviceMappingCache<KeSw, KeTr, Mo, KeEv>,
            DeviceMappingCache<MsSw, MsTr, Mo, MsEv>,
            MappingModifiersCache<Mo>,
        >,
    ) -> GlobalStateWithTimeoutResult<'a, Mo, KeEv, KeCo, MsEv, MsCo>
    where
        KeSw: Eq + Hash,
        MsSw: Eq + Hash,
        Mo: Eq + Hash + Ord,
        Ti: Clone + Ord,
        //
        Ti: std::fmt::Debug,
        Mo: std::fmt::Debug,
        KeEv: std::fmt::Debug,
        KeTr: std::fmt::Debug,
        KeSw: Clone + std::fmt::Debug,
        MsEv: std::fmt::Debug,
        MsTr: std::fmt::Debug,
        MsSw: Clone + std::fmt::Debug,
    {
        let mut state = self.as_keyboard_state_mut();
        let keyboard_long_press =
            state.with_press_timeout(time_minus_long_press_duration.clone(), mapping.keyboard());
        let keyboard_click_exact =
            state.with_release_timeout(time_minus_click_exact_duration.clone(), mapping.keyboard());

        let mut state = self.as_mouse_state_mut();
        let mouse_long_press =
            state.with_press_timeout(time_minus_long_press_duration.clone(), mapping.mouse());
        let mouse_click_exact =
            state.with_release_timeout(time_minus_click_exact_duration.clone(), mapping.mouse());

        GlobalStateWithTimeoutResult {
            keyboard_long_press,
            keyboard_click_exact,
            mouse_long_press,
            mouse_click_exact,
        }
    }
}

impl<Mo, Ti, Sw, Co, CsMs, TsMs, ShMsLo, ShMsCl, PoMs>
    GlobalState<
        Modifiers<Mo>,
        CoordsState<Co>,
        CsMs,
        TimedState<Sw>,
        TsMs,
        DeviceSchedulerState<Ti, Sw, Mo, Co, LongPressHandleRequest>,
        DeviceSchedulerState<Ti, Sw, Mo, Co, ClickExactHandleRequest>,
        ShMsLo,
        ShMsCl,
        PointerState<Sw, Co>,
        PoMs,
    >
{
    fn as_keyboard_state_mut(
        &mut self,
    ) -> DeviceState<
        &mut Modifiers<Mo>,
        &mut CoordsState<Co>,
        &mut TimedState<Sw>,
        &mut DeviceSchedulerState<Ti, Sw, Mo, Co, LongPressHandleRequest>,
        &mut DeviceSchedulerState<Ti, Sw, Mo, Co, ClickExactHandleRequest>,
        &mut PointerState<Sw, Co>,
    > {
        DeviceState::new(
            &mut self.modifiers,
            &mut self.keyboard_coords_state,
            &mut self.keyboard_timed_state,
            &mut self.keyboard_long_press_scheduler,
            &mut self.keyboard_click_exact_scheduler,
            &mut self.keyboard_pointer_state,
        )
    }

    pub fn with_keyboard_press_event<'a, Tr, MsMa, Ev>(
        &mut self,
        event: SwitchEvent<Ti, Sw>,
        mapping: &'a GlobalMappingCache<
            DeviceMappingCache<Sw, Tr, Mo, Ev>,
            MsMa,
            MappingModifiersCache<Mo>,
        >,
    ) -> GlobalStateWithEventResult<Option<Ti>, Option<(FilteredBindings<'a, Mo, Ev>, Co)>>
    where
        Sw: Clone + Eq + Hash,
        Mo: Clone + Eq + From<Sw> + Hash + Ord,
        Ti: Clone + Ord,
        Co: Clone,
        // TODO: Remove after debugging
        Ev: std::fmt::Debug,
        Ti: std::fmt::Debug,
        Sw: std::fmt::Debug,
        Tr: std::fmt::Debug,
        Mo: std::fmt::Debug,
        Ev: std::fmt::Debug,
    {
        let mut state = self.as_keyboard_state_mut();
        let (scheduled, bindings) =
            state.with_press_event(event, mapping.keyboard(), mapping.modifiers());

        GlobalStateWithEventResult {
            scheduled,
            bindings,
        }
    }

    pub fn with_keyboard_release_event<'a, Tr, MsMa, Ev>(
        &mut self,
        event: SwitchEvent<Ti, Sw>,
        mapping: &'a GlobalMappingCache<
            DeviceMappingCache<Sw, Tr, Mo, Ev>,
            MsMa,
            MappingModifiersCache<Mo>,
        >,
    ) -> GlobalStateWithEventResult<Option<Ti>, Option<(FilteredBindings<'a, Mo, Ev>, Co)>>
    where
        Sw: Clone + Eq + Hash,
        Mo: Clone + Eq + From<Sw> + Hash + Ord,
        Ti: Clone + Ord,
        Co: Clone,
        // TODO: Remove after debugging
        Ev: std::fmt::Debug,
        Ti: std::fmt::Debug,
        Sw: std::fmt::Debug,
        Tr: std::fmt::Debug,
        Mo: std::fmt::Debug,
        Ev: std::fmt::Debug,
    {
        let mut state = self.as_keyboard_state_mut();
        let (scheduled, bindings) =
            state.with_release_event(event, mapping.keyboard(), mapping.modifiers());

        GlobalStateWithEventResult {
            scheduled,
            bindings,
        }
    }

    pub fn with_keyboard_trigger_event<'a, Tr, MsMa, Ev>(
        &mut self,
        event: TriggerEvent<Ti, Tr>,
        mapping: &'a GlobalMappingCache<
            DeviceMappingCache<Sw, Tr, Mo, Ev>,
            MsMa,
            MappingModifiersCache<Mo>,
        >,
    ) -> GlobalStateWithEventResult<(), Option<(FilteredBindings<'a, Mo, Ev>, Co)>>
    where
        Tr: Eq + Hash,
        Mo: Clone + Hash + Ord,
        Co: Clone,
    {
        let mut state = self.as_keyboard_state_mut();
        let bindings = state.with_trigger_event(event, mapping.keyboard());

        GlobalStateWithEventResult {
            scheduled: (),
            bindings,
        }
    }

    pub fn with_keyboard_coords_event<'a, F, Tr, MsMa, Ev>(
        &mut self,
        event: CoordsEvent<Ti, Co>,
        mapping: &'a GlobalMappingCache<
            DeviceMappingCache<Sw, Tr, Mo, Ev>,
            MsMa,
            MappingModifiersCache<Mo>,
        >,
        is_dragged_fn: F,
    ) -> GlobalStateWithEventResult<(), Vec<(FilteredBindings<'a, Mo, Ev>, Co)>>
    where
        F: FnMut(&Co, &Co) -> bool,
        Sw: Clone + Eq + Hash,
        Mo: Clone + Hash + Ord,
        Co: Clone,
    {
        let mut state = self.as_keyboard_state_mut();
        let bindings = state.with_coords_event(event, mapping.keyboard(), is_dragged_fn);

        GlobalStateWithEventResult {
            scheduled: (),
            bindings,
        }
    }
}

impl<Mo, Ti, Sw, Co, CsKe, TsKe, ShKeLo, ShKeCl, PoKe>
    GlobalState<
        Modifiers<Mo>,
        CsKe,
        CoordsState<Co>,
        TsKe,
        TimedState<Sw>,
        ShKeLo,
        ShKeCl,
        DeviceSchedulerState<Ti, Sw, Mo, Co, LongPressHandleRequest>,
        DeviceSchedulerState<Ti, Sw, Mo, Co, ClickExactHandleRequest>,
        PoKe,
        PointerState<Sw, Co>,
    >
{
    fn as_mouse_state_mut(
        &mut self,
    ) -> DeviceState<
        &mut Modifiers<Mo>,
        &mut CoordsState<Co>,
        &mut TimedState<Sw>,
        &mut DeviceSchedulerState<Ti, Sw, Mo, Co, LongPressHandleRequest>,
        &mut DeviceSchedulerState<Ti, Sw, Mo, Co, ClickExactHandleRequest>,
        &mut PointerState<Sw, Co>,
    > {
        DeviceState::new(
            &mut self.modifiers,
            &mut self.mouse_coords_state,
            &mut self.mouse_timed_state,
            &mut self.mouse_long_press_scheduler,
            &mut self.mouse_click_exact_scheduler,
            &mut self.mouse_pointer_state,
        )
    }

    pub fn with_mouse_press_event<'a, Tr, KeMa, Ev>(
        &mut self,
        event: SwitchEvent<Ti, Sw>,
        mapping: &'a GlobalMappingCache<
            KeMa,
            DeviceMappingCache<Sw, Tr, Mo, Ev>,
            MappingModifiersCache<Mo>,
        >,
    ) -> GlobalStateWithEventResult<Option<Ti>, Option<(FilteredBindings<'a, Mo, Ev>, Co)>>
    where
        Sw: Clone + Eq + Hash,
        Mo: Clone + Eq + From<Sw> + Hash + Ord,
        Ti: Clone + Ord,
        Co: Clone,
        // TODO: Remove after debugging
        Ev: std::fmt::Debug,
        Ti: std::fmt::Debug,
        Sw: std::fmt::Debug,
        Tr: std::fmt::Debug,
        Mo: std::fmt::Debug,
        Ev: std::fmt::Debug,
    {
        let mut state = self.as_mouse_state_mut();
        let (scheduled, bindings) =
            state.with_press_event(event, mapping.mouse(), mapping.modifiers());

        GlobalStateWithEventResult {
            scheduled,
            bindings,
        }
    }

    pub fn with_mouse_release_event<'a, Tr, KeMa, Ev>(
        &mut self,
        event: SwitchEvent<Ti, Sw>,
        mapping: &'a GlobalMappingCache<
            KeMa,
            DeviceMappingCache<Sw, Tr, Mo, Ev>,
            MappingModifiersCache<Mo>,
        >,
    ) -> GlobalStateWithEventResult<Option<Ti>, Option<(FilteredBindings<'a, Mo, Ev>, Co)>>
    where
        Sw: Clone + Eq + Hash,
        Mo: Clone + Eq + From<Sw> + Hash + Ord,
        Ti: Clone + Ord,
        Co: Clone,
        // TODO: Remove after debugging
        Ev: std::fmt::Debug,
        Ti: std::fmt::Debug,
        Sw: std::fmt::Debug,
        Tr: std::fmt::Debug,
        Mo: std::fmt::Debug,
        Ev: std::fmt::Debug,
    {
        let mut state = self.as_mouse_state_mut();
        let (scheduled, bindings) =
            state.with_release_event(event, mapping.mouse(), mapping.modifiers());

        GlobalStateWithEventResult {
            scheduled,
            bindings,
        }
    }

    pub fn with_mouse_trigger_event<'a, Tr, KeMa, Ev>(
        &mut self,
        event: TriggerEvent<Ti, Tr>,
        mapping: &'a GlobalMappingCache<
            KeMa,
            DeviceMappingCache<Sw, Tr, Mo, Ev>,
            MappingModifiersCache<Mo>,
        >,
    ) -> GlobalStateWithEventResult<(), Option<(FilteredBindings<'a, Mo, Ev>, Co)>>
    where
        Tr: Eq + Hash,
        Mo: Clone + Hash + Ord,
        Co: Clone,
    {
        let mut state = self.as_mouse_state_mut();
        let bindings = state.with_trigger_event(event, mapping.mouse());

        GlobalStateWithEventResult {
            scheduled: (),
            bindings,
        }
    }

    pub fn with_mouse_coords_event<'a, F, Tr, KeMa, Ev>(
        &mut self,
        event: CoordsEvent<Ti, Co>,
        mapping: &'a GlobalMappingCache<
            KeMa,
            DeviceMappingCache<Sw, Tr, Mo, Ev>,
            MappingModifiersCache<Mo>,
        >,
        is_dragged_fn: F,
    ) -> GlobalStateWithEventResult<(), Vec<(FilteredBindings<'a, Mo, Ev>, Co)>>
    where
        F: FnMut(&Co, &Co) -> bool,
        Sw: Clone + Eq + Hash,
        Mo: Clone + Hash + Ord,
        Co: Clone,
    {
        let mut state = self.as_mouse_state_mut();
        let bindings = state.with_coords_event(event, mapping.mouse(), is_dragged_fn);

        GlobalStateWithEventResult {
            scheduled: (),
            bindings,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GlobalStateWithEventResult<Ti, Bi> {
    pub scheduled: Ti,
    pub bindings: Bi,
}

#[derive(Clone, Debug)]
pub struct GlobalStateWithTimeoutResult<'a, Mo, KeEv, KeCo, MsEv, MsCo> {
    pub keyboard_long_press: Vec<(FilteredBindings<'a, Mo, KeEv>, KeCo)>,
    pub keyboard_click_exact: Vec<(FilteredBindings<'a, Mo, KeEv>, KeCo)>,
    pub mouse_long_press: Vec<(FilteredBindings<'a, Mo, MsEv>, MsCo)>,
    pub mouse_click_exact: Vec<(FilteredBindings<'a, Mo, MsEv>, MsCo)>,
}

/*
#[test]
fn test1() {
    let global_state = GlobalState::new(
        1,
        (1, 2, 3),
        (),
        false,
        (),
        "123",
        "123".to_owned(),
        (),
        (),
        (1, 2),
        (),
    );

    let (mut state, global_state): (
        DeviceState<i32, (i8, i8, i8), bool, &str, String, (u8, u8)>,
        _,
    ) = global_state.take_state();
    let state = DeviceState {
        modifiers: state.modifiers + 10,
        coords_state: (
            state.coords_state.1,
            state.coords_state.2,
            state.coords_state.0,
        ),
        timed_state: !state.timed_state,
        long_press_scheduler: &state.long_press_scheduler[1..3],
        click_exact_scheduler: state.click_exact_scheduler[1..3].to_owned(),
        pointer_state: (state.pointer_state.1, state.pointer_state.0),
    };
    let global_state = global_state.with_state(state);

    assert_eq!(global_state.modifiers, 11);
    assert_eq!(global_state.keyboard_coords_state, (2, 3, 1));
    assert_eq!(global_state.keyboard_timed_state, true);
    assert_eq!(global_state.keyboard_long_press_scheduler, "23");
    assert_eq!(global_state.keyboard_pointer_state, (2, 1));
}
*/
/*
#[test]
fn test2() {
    let mut global_state = GlobalState::new(1, false, (), "123", (), (), ());

    let (mut state, global_state): (State<i32, bool, &str>, _) = global_state.take_state();
    let state = State {
        modifiers: state.modifiers + 10,
        timed_state: !state.timed_state,
        scheduler: &state.scheduler[1..3],
    };
    global_state.with_field(state.modifiers);
}*/
