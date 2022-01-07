use core::hash::Hash;

use input_core::{
    ClickExactHandleRequest, LongPressHandleRequest, Modifiers, PointerState, SchedulerState,
    TimedEventData, TimedState,
};

use crate::{
    define_markers, define_struct_take_and_with_field, DeviceMappingCache, MappingModifiersCache,
    SwitchBindings, SwitchEvent, SwitchMappingCache,
};

#[derive(Clone, Debug, Default)]
pub struct DeviceState<Mo, Ts, Sh, Po> {
    pub modifiers: Mo,
    pub timed_state: Ts,
    pub scheduler: Sh,
    pub pointer_state: Po,
}

define_markers!(
    ModifiersMarker,
    TimedStateMarker,
    SchedulerMarker,
    PointerMarker
);

define_struct_take_and_with_field!(DeviceState {
    modifiers: Mo + ModifiersMarker,
    timed_state: Ts + TimedStateMarker,
    scheduler: Sh + SchedulerMarker,
    pointer_state: Po + PointerMarker,
});

impl<Mo, Ts, Sh, Po> DeviceState<Mo, Ts, Sh, Po> {
    pub fn new(modifiers: Mo, timed_state: Ts, scheduler: Sh, pointer_state: Po) -> Self {
        Self {
            modifiers,
            timed_state,
            scheduler,
            pointer_state,
        }
    }
}

impl<Sw, Mo, Ti, Co>
    DeviceState<
        Modifiers<Mo>,
        TimedState<Sw>,
        SchedulerState<Ti, (SwitchEvent<Ti, Sw, Co>, Modifiers<Mo>), LongPressHandleRequest>,
        PointerState<Sw, Co>,
    >
{
    pub fn with_press_event<'a, Tr, Bi>(
        mut self,
        event: SwitchEvent<Ti, Sw, Co>,
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Bi>,
        mapping_modifiers: &MappingModifiersCache<Mo>,
    ) -> (Self, Option<Ti>, Option<(SwitchBindings<'a, Mo, Bi>, Co)>)
    where
        Sw: Clone + Eq + Hash,
        Mo: Clone + Eq + From<Sw> + Hash + Ord,
        Ti: Clone + Ord,
        Co: Clone,
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

        let mapping = unwrap_or_return!(mapping, (self, None, None));

        let mapping = mapping.filter_by_modifiers(&modifiers);

        let mapping = unwrap_or_return!(mapping, (self, None, None));

        let (timed_state, rest): (TimedState<Sw>, _) = self.take_field();
        let (timed_state, result) = timed_state.with_press_event(event.switch.clone());
        let request = result.unwrap();
        self = rest.with_field(timed_state);

        let (scheduler, rest): (
            SchedulerState<Ti, (SwitchEvent<Ti, Sw, Co>, Modifiers<Mo>), LongPressHandleRequest>,
            _,
        ) = self.take_field();
        let scheduler = scheduler.schedule(
            event.time.clone(),
            (event.clone(), modifiers.clone()),
            request,
        );
        let next_scheduled = scheduler.next_scheduled().cloned();
        self = rest.with_field(scheduler);

        // FIXME: filter all
        let mapping = mapping
            .press
            .and_then(|mapping| mapping.filter_by_timed_data(&()));

        //let mapping = unwrap_or_return!(mapping, (self, next_scheduled, None)); // FIXME

        let (pointer_state, rest): (PointerState<Sw, Co>, _) = self.take_field();
        let (pointer_state, result) =
            pointer_state.with_press_event(event.switch, event.coords.clone());
        //result.unwrap(); // FIXME
        self = rest.with_field(pointer_state);
        let mapping = unwrap_or_return!(mapping, (self, next_scheduled, None)); // FIXME

        let mapping = mapping.filter_by_pointer_data(&());
        let mapping = mapping.expect("filtering should never fails");

        (self, next_scheduled, Some((mapping, event.coords)))
    }

    pub fn with_press_timeout<'a, Tr, Bi>(
        mut self,
        time_minus_long_press_duration: Ti, // TODO: Time at Long press handling event already happend for time before that
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Bi>,
    ) -> (Self, Vec<(SwitchBindings<'a, Mo, Bi>, Co)>)
    where
        Sw: Eq + Hash,
        Mo: Eq + Hash + Ord,
        Ti: Ord,
    {
        use crate::{StructTakeField, StructWithField};

        let (mut timed_state, rest): (TimedState<Sw>, _) = self.take_field();

        let (scheduler, rest): (
            SchedulerState<Ti, (SwitchEvent<Ti, Sw, Co>, Modifiers<Mo>), LongPressHandleRequest>,
            _,
        ) = rest.take_field();
        let (pointer_state, rest): (PointerState<Sw, Co>, _) = rest.take_field();
        let (scheduler, requests) = scheduler.take_scheduled(&time_minus_long_press_duration);
        let rest = rest.with_field(pointer_state);
        let rest = rest.with_field(scheduler);

        let mut delayed_bindings = Vec::new();
        for (_, requests) in requests {
            for ((event, modifiers), request) in requests {
                let (new_timed_state, result) = with_timeout_event(
                    &mapping.long_press,
                    timed_state,
                    event,
                    modifiers,
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
}

impl<Sw, Mo, Ti, Co>
    DeviceState<
        Modifiers<Mo>,
        TimedState<Sw>,
        SchedulerState<Ti, (SwitchEvent<Ti, Sw, Co>, Modifiers<Mo>), ClickExactHandleRequest>,
        PointerState<Sw, Co>,
    >
{
    pub fn with_release_event<'a, Tr, Bi>(
        mut self,
        event: SwitchEvent<Ti, Sw, Co>,
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Bi>,
        mapping_modifiers: &MappingModifiersCache<Mo>,
    ) -> (Self, Option<Ti>, Option<(SwitchBindings<'a, Mo, Bi>, Co)>)
    where
        Sw: Clone + Eq + Hash,
        Mo: Clone + Eq + From<Sw> + Hash + Ord,
        Ti: Clone + Ord,
        Co: Clone,
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

        let mapping = unwrap_or_return!(mapping, (self, None, None));

        let mapping = mapping.filter_by_modifiers(&modifiers);

        let mapping = unwrap_or_return!(mapping, (self, None, None));

        let (timed_state, rest): (TimedState<Sw>, _) = self.take_field();
        let (timed_state, result) = timed_state.with_release_event(event.switch.clone());
        let timed_data = result.unwrap();
        self = rest.with_field(timed_state);

        let (timed_data, next_scheduled) = match timed_data {
            Some((timed_data, request)) => {
                let (scheduler, rest): (
                    SchedulerState<
                        Ti,
                        (SwitchEvent<Ti, Sw, Co>, Modifiers<Mo>),
                        ClickExactHandleRequest,
                    >,
                    _,
                ) = self.take_field();
                let scheduler = scheduler.schedule(
                    event.time.clone(),
                    (event.clone(), modifiers.clone()),
                    request,
                );
                let next_scheduled = scheduler.next_scheduled().cloned();
                self = rest.with_field(scheduler);
                (Some(timed_data), next_scheduled)
            }
            None => (None, None),
        };

        // FIXME: filter all
        let mapping = mapping
            .release
            .and_then(|mapping| mapping.filter_by_timed_data(&timed_data));

        //let mapping = unwrap_or_return!(mapping, (self, next_scheduled, None)); // FIXME

        let (pointer_state, rest): (PointerState<Sw, Co>, _) = self.take_field();
        let (pointer_state, result) = pointer_state.with_release_event(&event.switch);
        let pointer_data = result.unwrap();
        self = rest.with_field(pointer_state);
        let mapping = unwrap_or_return!(mapping, (self, next_scheduled, None)); // FIXME

        let mapping = mapping.filter_by_pointer_data(&pointer_data);
        let mapping = unwrap_or_return!(mapping, (self, next_scheduled, None));

        (self, next_scheduled, Some((mapping, event.coords)))
    }

    pub fn with_release_timeout<'a, Tr, Bi>(
        mut self,
        time_minus_click_exact_duration: Ti, // TODO: Time at Long press handling event already happend for time before that
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Bi>,
    ) -> (Self, Vec<(SwitchBindings<'a, Mo, Bi>, Co)>)
    where
        Sw: Eq + Hash,
        Mo: Eq + Hash + Ord,
        Ti: Ord,
    {
        use crate::{StructTakeField, StructWithField};

        let (mut timed_state, rest): (TimedState<Sw>, _) = self.take_field();

        let (scheduler, rest): (
            SchedulerState<Ti, (SwitchEvent<Ti, Sw, Co>, Modifiers<Mo>), ClickExactHandleRequest>,
            _,
        ) = rest.take_field();
        let (pointer_state, rest): (PointerState<Sw, Co>, _) = rest.take_field();
        let (scheduler, requests) = scheduler.take_scheduled(&time_minus_click_exact_duration);
        let rest = rest.with_field(pointer_state);
        let rest = rest.with_field(scheduler);

        let mut delayed_bindings = Vec::new();
        for (_, requests) in requests {
            for ((event, modifiers), request) in requests {
                let (new_timed_state, result) = with_timeout_event(
                    &mapping.click_exact,
                    timed_state,
                    event,
                    modifiers,
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
}

fn with_timeout_event<'a, Sw, Mo, Ti, Co, Rq, Td, Bi>(
    mapping: &'a SwitchMappingCache<Sw, Mo, TimedEventData<Td>, (), Bi>,
    timed_state: TimedState<Sw>,
    event: SwitchEvent<Ti, Sw, Co>,
    modifiers: Modifiers<Mo>,
    request: Rq,
    timed_processing: impl FnOnce(
        TimedState<Sw>,
        Sw,
        Rq,
    ) -> (TimedState<Sw>, Option<TimedEventData<Td>>),
) -> (TimedState<Sw>, Option<(SwitchBindings<'a, Mo, Bi>, Co)>)
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

    (timed_state, Some((bindings, event.coords)))
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
    let state = DeviceState::new(1, false, "123", (1, 2));

    let (modifiers, rest): (i32, _) = state.take_field();
    let state: DeviceState<_, _, _, _> = rest.with_field(modifiers + 10);

    let (timed, rest): (bool, _) = state.take_field();
    let state: DeviceState<_, _, _, _> = rest.with_field(!timed);

    let (scheduler, rest): (&str, _) = state.take_field();
    let state: DeviceState<_, _, _, _> = rest.with_field(&scheduler[1..3]);

    assert_eq!(state.modifiers, 11);
    assert_eq!(state.timed_state, true);
    assert_eq!(state.scheduler, "23");
}
