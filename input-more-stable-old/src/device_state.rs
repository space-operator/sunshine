use core::hash::Hash;

use input_core::{
    ClickExactHandleRequest, CoordsState, LongPressHandleRequest, Modifiers, PointerState,
    SchedulerState, TimedEventData, TimedState,
};

use crate::{
    define_markers, define_struct_take_and_with_field, CoordsEvent, DeviceMappingCache,
    FilteredBindings, MappingModifiersCache, SwitchEvent, SwitchMappingCache, TriggerEvent,
};

#[derive(Clone, Debug, Default)]
pub struct DeviceState<Mo, Cs, Ts, ShLo, ShCl, Po> {
    pub modifiers: Mo,
    pub coords_state: Cs,
    pub timed_state: Ts,
    pub long_press_scheduler: ShLo,
    pub click_exact_scheduler: ShCl,
    pub pointer_state: Po,
}

define_markers!(
    ModifiersMarker,
    CoordStateMarker,
    TimedStateMarker,
    SchedulerPressMarker,
    SchedulerReleaseMarker,
    PointerMarker
);

define_struct_take_and_with_field!(DeviceState {
    modifiers: Mo + ModifiersMarker,
    coords_state: Cs + CoordStateMarker,
    timed_state: Ts + TimedStateMarker,
    long_press_scheduler: ShLo + SchedulerPressMarker,
    click_exact_scheduler: ShCl + SchedulerReleaseMarker,
    pointer_state: Po + PointerMarker,
});

impl<Mo, Cs, Ts, ShLo, ShCl, Po> DeviceState<Mo, Cs, Ts, ShLo, ShCl, Po> {
    pub fn new(
        modifiers: Mo,
        coords_state: Cs,
        timed_state: Ts,
        long_press_scheduler: ShLo,
        click_exact_scheduler: ShCl,
        pointer_state: Po,
    ) -> Self {
        Self {
            modifiers,
            coords_state,
            timed_state,
            long_press_scheduler,
            click_exact_scheduler,
            pointer_state,
        }
    }
}

pub type DeviceSchedulerState<Ti, Sw, Mo, Co, Re> =
    SchedulerState<Ti, (SwitchEvent<Ti, Sw>, Modifiers<Mo>, Co), Re>;

impl<Sw, Mo, Ti, Co>
    DeviceState<
        Modifiers<Mo>,
        CoordsState<Co>,
        TimedState<Sw>,
        DeviceSchedulerState<Ti, Sw, Mo, Co, LongPressHandleRequest>,
        DeviceSchedulerState<Ti, Sw, Mo, Co, ClickExactHandleRequest>,
        PointerState<Sw, Co>,
    >
{
    pub fn with_press_event<'a, Tr, Ev>(
        mut self,
        event: SwitchEvent<Ti, Sw>,
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
        mapping_modifiers: &MappingModifiersCache<Mo>,
    ) -> (Self, Option<Ti>, Option<(FilteredBindings<'a, Mo, Ev>, Co)>)
    where
        Sw: Clone + Eq + Hash,
        Mo: Clone + Eq + From<Sw> + Hash + Ord,
        Ti: Clone + Ord,
        Co: Clone,
        // TODO: Remove after debugging
        Ev: std::fmt::Debug,
        Tr: std::fmt::Debug,
        Mo: std::fmt::Debug,
        Ev: std::fmt::Debug,
    {
        use crate::{unwrap_or_return, StructTakeField, StructWithField};

        let mapping = mapping.filter_by_switch(&event.switch);

        let modifier = Mo::from(event.switch.clone());
        let is_used_as_modifier = mapping_modifiers.switches().contains(&modifier);

        if mapping.is_none() && !is_used_as_modifier {
            return (self, None, None);
        }

        let (modifiers, rest): (Modifiers<Mo>, _) = self.take_field();
        let modifiers = if is_used_as_modifier {
            let (modifiers, result) = modifiers.with_press_event(modifier);
            // result.unwrap(); // FIXME
            modifiers
        } else {
            modifiers
        };
        self = rest.with_field(modifiers.clone());

        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (self, None, None));

        let mapping = mapping.filter_by_modifiers(&modifiers);

        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (self, None, None));

        let (timed_state, rest): (TimedState<Sw>, _) = self.take_field();
        let (timed_state, result) = timed_state.with_press_event(event.switch.clone());
        let request = result.unwrap();
        self = rest.with_field(timed_state);

        let (scheduler, rest): (
            DeviceSchedulerState<Ti, Sw, Mo, Co, LongPressHandleRequest>,
            _,
        ) = self.take_field();
        let (coords_state, rest): (CoordsState<Co>, _) = rest.take_field();
        let scheduler = scheduler.schedule(
            event.time.clone(),
            (
                event.clone(),
                modifiers.clone(),
                coords_state.coords().clone(),
            ),
            request,
        );
        let rest = rest.with_field(coords_state);
        let next_scheduled = scheduler.next_scheduled().cloned();
        self = rest.with_field(scheduler);

        let mapping = mapping
            .press
            .and_then(|mapping| mapping.filter_by_timed_data(&()));

        let (pointer_state, rest): (PointerState<Sw, Co>, _) = self.take_field();
        let (coords_state, rest): (CoordsState<Co>, _) = rest.take_field();
        let (pointer_state, result) =
            pointer_state.with_press_event(event.switch, coords_state.coords().clone());
        //result.unwrap(); // FIXME
        let rest = rest.with_field(coords_state);
        self = rest.with_field(pointer_state);
        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (self, next_scheduled, None)); // FIXME

        let mapping = mapping.filter_by_pointer_data(&());
        let mapping = mapping.expect("filtering should never fail");

        let (coords_state, rest): (CoordsState<Co>, _) = self.take_field();
        let coords = coords_state.coords().clone();
        self = rest.with_field(coords_state);

        println!("{:?}", mapping);
        (self, next_scheduled, Some((mapping, coords)))
    }

    pub fn with_press_timeout<'a, Tr, Ev>(
        self,
        time_minus_long_press_duration: Ti, // TODO: Time at Long press handling event already happend for time before that
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
    ) -> (Self, Vec<(FilteredBindings<'a, Mo, Ev>, Co)>)
    where
        Sw: Eq + Hash,
        Mo: Eq + Hash + Ord,
        Ti: Ord,
    {
        use crate::{StructTakeField, StructWithField};

        let (mut timed_state, rest): (TimedState<Sw>, _) = self.take_field();

        let (scheduler, rest): (
            DeviceSchedulerState<Ti, Sw, Mo, Co, LongPressHandleRequest>,
            _,
        ) = rest.take_field();
        let (pointer_state, rest): (PointerState<Sw, Co>, _) = rest.take_field();
        let (scheduler, requests) = scheduler.take_scheduled(&time_minus_long_press_duration);
        let rest = rest.with_field(pointer_state);
        let rest = rest.with_field(scheduler);

        let mut delayed_bindings = Vec::new();
        for (_, requests) in requests {
            for ((event, modifiers, coords), request) in requests {
                let (new_timed_state, result) = with_timeout_event(
                    &mapping.long_press,
                    timed_state,
                    event,
                    modifiers,
                    coords,
                    request,
                    |timed_state, switch, request| {
                        let (timed_state, result) =
                            timed_state.with_long_press_event(switch, request);
                        (timed_state, result.unwrap())
                    },
                );
                timed_state = new_timed_state;
                if let Some((bindings, coords)) = result {
                    delayed_bindings.push((bindings, coords));
                }
            }
        }
        (rest.with_field(timed_state), delayed_bindings)
    }

    pub fn with_release_event<'a, Tr, Ev>(
        mut self,
        event: SwitchEvent<Ti, Sw>,
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
        mapping_modifiers: &MappingModifiersCache<Mo>,
    ) -> (Self, Option<Ti>, Option<(FilteredBindings<'a, Mo, Ev>, Co)>)
    where
        Sw: Clone + Eq + Hash,
        Mo: Clone + Eq + From<Sw> + Hash + Ord,
        Ti: Clone + Ord,
        Co: Clone,
        // TODO: Remove after debugging
        Ev: std::fmt::Debug,
        Tr: std::fmt::Debug,
        Mo: std::fmt::Debug,
        Ev: std::fmt::Debug,
    {
        use crate::{unwrap_or_return, StructTakeField, StructWithField};

        let mapping = mapping.filter_by_switch(&event.switch);

        let modifier = Mo::from(event.switch.clone());
        let is_used_as_modifier = mapping_modifiers.switches().contains(&modifier);

        if mapping.is_none() && !is_used_as_modifier {
            return (self, None, None);
        }

        let (modifiers, rest): (Modifiers<Mo>, _) = self.take_field();
        let modifiers = if is_used_as_modifier {
            dbg!();
            let (modifiers, result) = modifiers.with_release_event(&modifier);
            result.unwrap();
            modifiers
        } else {
            modifiers
        };
        self = rest.with_field(modifiers.clone());

        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (self, None, None));

        let mapping = mapping.filter_by_modifiers(&modifiers);

        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (self, None, None));

        let (timed_state, rest): (TimedState<Sw>, _) = self.take_field();
        let (timed_state, result) = timed_state.with_release_event(event.switch.clone());
        let timed_data = result.unwrap();
        self = rest.with_field(timed_state);

        let (timed_data, next_scheduled) = match timed_data {
            Some((timed_data, request)) => {
                let (scheduler, rest): (
                    DeviceSchedulerState<Ti, Sw, Mo, Co, ClickExactHandleRequest>,
                    _,
                ) = self.take_field();
                let (coords_state, rest): (CoordsState<Co>, _) = rest.take_field();
                let scheduler = scheduler.schedule(
                    event.time.clone(),
                    (
                        event.clone(),
                        modifiers.clone(),
                        coords_state.coords().clone(),
                    ),
                    request,
                );
                let next_scheduled = scheduler.next_scheduled().cloned();
                let rest = rest.with_field(coords_state);
                self = rest.with_field(scheduler);
                (Some(timed_data), next_scheduled)
            }
            None => (None, None),
        };

        let mapping = mapping
            .release
            .and_then(|mapping| mapping.filter_by_timed_data(&timed_data));

        let (pointer_state, rest): (PointerState<Sw, Co>, _) = self.take_field();
        let (pointer_state, result) = pointer_state.with_release_event(&event.switch);
        let pointer_data = result.unwrap();
        self = rest.with_field(pointer_state);
        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (self, next_scheduled, None));

        let mapping = mapping.filter_by_pointer_data(&pointer_data);
        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (self, next_scheduled, None));

        let (coords_state, rest): (CoordsState<Co>, _) = self.take_field();
        let coords = coords_state.coords().clone();
        self = rest.with_field(coords_state);

        println!("{:?}", mapping);
        (self, next_scheduled, Some((mapping, coords)))
    }

    pub fn with_release_timeout<'a, Tr, Ev>(
        self,
        time_minus_click_exact_duration: Ti, // TODO: Time at Long press handling event already happend for time before that
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
    ) -> (Self, Vec<(FilteredBindings<'a, Mo, Ev>, Co)>)
    where
        Sw: Eq + Hash,
        Mo: Eq + Hash + Ord,
        Ti: Ord,
    {
        use crate::{StructTakeField, StructWithField};

        let (mut timed_state, rest): (TimedState<Sw>, _) = self.take_field();
        let (scheduler, rest): (
            DeviceSchedulerState<Ti, Sw, Mo, Co, ClickExactHandleRequest>,
            _,
        ) = rest.take_field();
        let (pointer_state, rest): (PointerState<Sw, Co>, _) = rest.take_field();
        let (scheduler, requests) = scheduler.take_scheduled(&time_minus_click_exact_duration);
        let rest = rest.with_field(pointer_state);
        let rest = rest.with_field(scheduler);

        let mut delayed_bindings = Vec::new();
        for (_, requests) in requests {
            for ((event, modifiers, coords), request) in requests {
                let result = timed_state.with_reset_click_count(&event.switch);
                timed_state = result.0;
                result.1.unwrap();

                let (new_timed_state, result) = with_timeout_event(
                    &mapping.click_exact,
                    timed_state,
                    event,
                    modifiers,
                    coords,
                    request,
                    |timed_state, switch, request| {
                        let (timed_state, result) =
                            timed_state.with_click_exact_event(switch, request);
                        (timed_state, result.unwrap())
                    },
                );
                timed_state = new_timed_state;
                if let Some((bindings, coords)) = result {
                    delayed_bindings.push((bindings, coords));
                }
            }
        }
        (rest.with_field(timed_state), delayed_bindings)
    }

    pub fn with_trigger_event<'a, Tr, Ev>(
        mut self,
        event: TriggerEvent<Ti, Tr>,
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
    ) -> (Self, Option<(FilteredBindings<'a, Mo, Ev>, Co)>)
    where
        Tr: Eq + Hash,
        Mo: Clone + Hash + Ord,
        Co: Clone,
    {
        use crate::{unwrap_or_return, StructTakeField, StructWithField};

        let (modifiers, rest): (Modifiers<Mo>, _) = self.take_field();
        self = rest.with_field(modifiers.clone());

        let mapping = &mapping.trigger;
        let mapping = mapping.filter_by_switch(&event.trigger);
        let mapping = unwrap_or_return!(mapping, (self, None));
        let mapping = mapping.filter_by_modifiers(&modifiers);
        let bindings = unwrap_or_return!(mapping, (self, None));

        let (coords_state, rest): (CoordsState<Co>, _) = self.take_field();
        let coords = coords_state.coords().clone();
        self = rest.with_field(coords_state);
        (self, Some((bindings, coords)))
    }

    pub fn with_coords_event<'a, Tr, Ev>(
        mut self,
        event: CoordsEvent<Ti, Co>,
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
    ) -> (Self, Vec<(FilteredBindings<'a, Mo, Ev>, Co)>)
    where
        Sw: Clone + Eq + Hash,
        Mo: Clone + Hash + Ord,
        Co: Clone + Eq,
    {
        use crate::{unwrap_or_continue, StructTakeField, StructWithField};

        let (modifiers, rest): (Modifiers<Mo>, _) = self.take_field();
        self = rest.with_field(modifiers.clone());

        let (_, rest): (CoordsState<Co>, _) = self.take_field();
        self = rest.with_field(CoordsState::with_coords(event.coords.clone()));

        let (pointer_state, rest): (PointerState<Sw, Co>, _) = self.take_field();
        let (pointer_state, (events, _)) =
            pointer_state.with_move_event(|coords| coords != &event.coords); // TODO : Add margins
        self = rest.with_field(pointer_state);

        let mut all_bindings = vec![];
        let mapping = &mapping.coords;
        for event in events {
            let mapping = mapping.filter_by_pointer_data(&event);
            let mapping = unwrap_or_continue!(mapping);
            let mapping = mapping.filter_by_modifiers(&modifiers);
            let bindings = unwrap_or_continue!(mapping);

            let (coords_state, rest): (CoordsState<Co>, _) = self.take_field();
            let coords = coords_state.coords().clone();
            self = rest.with_field(coords_state);
            all_bindings.push((bindings, coords));
        }

        (self, all_bindings)
    }
}

fn with_timeout_event<'a, Sw, Mo, Ti, Co, Rq, Td, Bi>(
    mapping: &'a SwitchMappingCache<Sw, Mo, TimedEventData<Td>, (), Bi>,
    timed_state: TimedState<Sw>,
    event: SwitchEvent<Ti, Sw>,
    modifiers: Modifiers<Mo>,
    coords: Co,
    request: Rq,
    timed_processing: impl FnOnce(
        TimedState<Sw>,
        Sw,
        Rq,
    ) -> (TimedState<Sw>, Option<TimedEventData<Td>>),
) -> (TimedState<Sw>, Option<(FilteredBindings<'a, Mo, Bi>, Co)>)
where
    Sw: Eq + Hash,
    Mo: Eq + Hash + Ord,
    Td: 'a + Eq + Hash,
{
    use crate::unwrap_or_return;

    let mapping = unwrap_or_return!(mapping.filter_by_switch(&event.switch), (timed_state, None));

    let mapping = unwrap_or_return!(mapping.filter_by_modifiers(&modifiers), (timed_state, None));
    //let (new_timed_state, result) = timed_state.with_long_press_event(event.switch, request);
    let (timed_state, result) = timed_processing(timed_state, event.switch, request);

    let timed_data = unwrap_or_return!(result, (timed_state, None));
    let mapping = unwrap_or_return!(
        mapping.filter_by_timed_data(&timed_data),
        (timed_state, None)
    );
    let bindings = unwrap_or_return!(mapping.filter_by_pointer_data(&()), (timed_state, None));

    (timed_state, Some((bindings, coords)))
}
/*pub fn with_press_event<Ti, Sw, Co, KePrMa, KeReMa, KeLoMa, KeClMa>(
    self,
    event: SwitchEvent<Ti, Sw, Co, (), ()>,
    mapping: &GlobalMapping<KePrMa, KeReMa, KeLoMa, KeClMa>,
) -> Self {
    self
}

pub fn with_release_event<Ti, Sw, Co, KePrMa, KeReMa, KeLoMa, KeClMa>(
    self,
    event: SwitchEvent<Ti, Sw, Co, (), ()>,
    mapping: &GlobalMapping<KePrMa, KeReMa, KeLoMa, KeClMa>,
) -> Self {
    self
}*/

#[test]
fn test() {
    use crate::{StructTakeField, StructWithField};
    let state = DeviceState::new(1, (false,), false, "123", (1, 2, 3), (1, 2));

    let (modifiers, rest): (i32, _) = state.take_field();
    let state: DeviceState<_, _, _, _, _, _> = rest.with_field(modifiers + 10);

    let (timed, rest): (bool, _) = state.take_field();
    let state: DeviceState<_, _, _, _, _, _> = rest.with_field(!timed);

    let (scheduler, rest): (&str, _) = state.take_field();
    let state: DeviceState<_, _, _, _, _, _> = rest.with_field(&scheduler[1..3]);

    assert_eq!(state.modifiers, 11);
    assert_eq!(state.timed_state, true);
    assert_eq!(state.long_press_scheduler, "23");
}
