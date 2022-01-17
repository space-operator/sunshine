use core::hash::Hash;

use input_core::{
    ClickExactHandleRequest, CoordsState, LongPressHandleRequest, Modifiers, PointerState,
    SchedulerState, TimedEventData, TimedState,
};

use crate::{
    CoordsEvent, DeviceMappingCache, FilteredBindings, MappingModifiersCache, SwitchEvent,
    SwitchMappingCache, TriggerEvent,
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

/*
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
*/

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
        &mut self,
        event: SwitchEvent<Ti, Sw>,
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
        mapping_modifiers: &MappingModifiersCache<Mo>,
    ) -> (Option<Ti>, Option<(FilteredBindings<'a, Mo, Ev>, Co)>)
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
        use crate::unwrap_or_return;

        let mapping = mapping.filter_by_switch(&event.switch);

        let modifier = Mo::from(event.switch.clone());
        let is_used_as_modifier = mapping_modifiers.switches().contains(&modifier);

        if mapping.is_none() && !is_used_as_modifier {
            return (None, None);
        }

        if is_used_as_modifier {
            let result = self.modifiers.on_press_event(modifier);
            // result.unwrap(); // FIXME
        }

        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (None, None));

        let mapping = mapping.filter_by_modifiers(&self.modifiers);

        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (None, None));

        let result = self.timed_state.on_press_event(event.switch.clone());
        let request = result.unwrap();

        self.long_press_scheduler.schedule(
            event.time.clone(),
            (
                event.clone(),
                self.modifiers.clone(),
                self.coords_state.coords().clone(),
            ),
            request,
        );
        let next_scheduled = self.long_press_scheduler.next_scheduled().cloned();

        let mapping = mapping
            .press
            .and_then(|mapping| mapping.filter_by_timed_data(&()));

        let result = self
            .pointer_state
            .with_press_event(event.switch, self.coords_state.coords().clone());
        //result.unwrap(); // FIXME
        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (next_scheduled, None)); // FIXME

        let mapping = mapping.filter_by_pointer_data(&());
        let mapping = mapping.expect("filtering should never fail");

        let coords = self.coords_state.coords().clone();

        println!("{:?}", mapping);
        (next_scheduled, Some((mapping, coords)))
    }

    pub fn with_press_timeout<'a, Tr, Ev>(
        &mut self,
        time_minus_long_press_duration: Ti, // TODO: Time at Long press handling event already happend for time before that
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
    ) -> Vec<(FilteredBindings<'a, Mo, Ev>, Co)>
    where
        Sw: Eq + Hash,
        Mo: Eq + Hash + Ord,
        Ti: Ord,
    {
        let requests = self
            .long_press_scheduler
            .take_scheduled(&time_minus_long_press_duration);

        let mut delayed_bindings = Vec::new();
        for (_, requests) in requests {
            for ((event, modifiers, coords), request) in requests {
                let result = with_timeout_event(
                    &mapping.long_press,
                    event.switch,
                    &modifiers,
                    coords,
                    |switch| {
                        let result = self.timed_state.on_long_press_event(switch, request);
                        result.unwrap()
                    },
                );
                if let Some((bindings, coords)) = result {
                    delayed_bindings.push((bindings, coords));
                }
            }
        }
        delayed_bindings
    }

    pub fn with_release_event<'a, Tr, Ev>(
        &mut self,
        event: SwitchEvent<Ti, Sw>,
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
        mapping_modifiers: &MappingModifiersCache<Mo>,
    ) -> (Option<Ti>, Option<(FilteredBindings<'a, Mo, Ev>, Co)>)
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
        use crate::unwrap_or_return;

        let mapping = mapping.filter_by_switch(&event.switch);

        let modifier = Mo::from(event.switch.clone());
        let is_used_as_modifier = mapping_modifiers.switches().contains(&modifier);

        if mapping.is_none() && !is_used_as_modifier {
            return (None, None);
        }

        if is_used_as_modifier {
            let result = self.modifiers.on_release_event(&modifier);
            result.unwrap();
        }

        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (None, None));

        let mapping = mapping.filter_by_modifiers(&self.modifiers);

        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (None, None));

        let timed_data = self
            .timed_state
            .on_release_event(event.switch.clone())
            .unwrap();

        let (timed_data, next_scheduled) = match timed_data {
            Some((timed_data, request)) => {
                self.click_exact_scheduler.schedule(
                    event.time.clone(),
                    (
                        event.clone(),
                        self.modifiers.clone(),
                        self.coords_state.coords().clone(),
                    ),
                    request,
                );
                let next_scheduled = self.click_exact_scheduler.next_scheduled().cloned();
                (Some(timed_data), next_scheduled)
            }
            None => (None, None),
        };

        let mapping = mapping
            .release
            .and_then(|mapping| mapping.filter_by_timed_data(&timed_data));

        let pointer_data = self
            .pointer_state
            .with_release_event(&event.switch)
            .unwrap();
        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (next_scheduled, None));

        let mapping = mapping.filter_by_pointer_data(&pointer_data);
        println!("{:?}", mapping);
        let mapping = unwrap_or_return!(mapping, (next_scheduled, None));

        let coords = self.coords_state.coords().clone();

        println!("{:?}", mapping);
        (next_scheduled, Some((mapping, coords)))
    }

    pub fn with_release_timeout<'a, Tr, Ev>(
        &mut self,
        time_minus_click_exact_duration: Ti, // TODO: Time at Long press handling event already happend for time before that
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
    ) -> Vec<(FilteredBindings<'a, Mo, Ev>, Co)>
    where
        Sw: Eq + Hash,
        Mo: Eq + Hash + Ord,
        Ti: Ord,
    {
        let requests = self
            .click_exact_scheduler
            .take_scheduled(&time_minus_click_exact_duration);

        let mut delayed_bindings = Vec::new();
        for (_, requests) in requests {
            for ((event, modifiers, coords), request) in requests {
                self.timed_state
                    .on_reset_click_count(&event.switch)
                    .unwrap();

                let result = with_timeout_event(
                    &mapping.click_exact,
                    event.switch,
                    &modifiers,
                    coords,
                    |switch| {
                        let result = self.timed_state.on_click_exact_event(switch, request);
                        result.unwrap()
                    },
                );
                if let Some((bindings, coords)) = result {
                    delayed_bindings.push((bindings, coords));
                }
            }
        }
        delayed_bindings
    }

    pub fn with_trigger_event<'a, Tr, Ev>(
        &mut self,
        event: TriggerEvent<Ti, Tr>,
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
    ) -> Option<(FilteredBindings<'a, Mo, Ev>, Co)>
    where
        Tr: Eq + Hash,
        Mo: Clone + Hash + Ord,
        Co: Clone,
    {
        use crate::unwrap_or_return;

        let mapping = &mapping.trigger;
        let mapping = mapping.filter_by_switch(&event.trigger);
        let mapping = unwrap_or_return!(mapping, None);
        let mapping = mapping.filter_by_modifiers(&self.modifiers);
        let bindings = unwrap_or_return!(mapping, None);

        let coords = self.coords_state.coords().clone();
        Some((bindings, coords))
    }

    pub fn with_coords_event<'a, F, Tr, Ev>(
        &mut self,
        event: CoordsEvent<Ti, Co>,
        mapping: &'a DeviceMappingCache<Sw, Tr, Mo, Ev>,
        mut is_dragged_fn: F,
    ) -> Vec<(FilteredBindings<'a, Mo, Ev>, Co)>
    where
        F: FnMut(&Co, &Co) -> bool,
        Sw: Clone + Eq + Hash,
        Mo: Clone + Hash + Ord,
        Co: Clone + Eq,
    {
        use crate::unwrap_or_continue;

        self.coords_state.set_coords(event.coords.clone());

        let data = self
            .pointer_state
            .with_move_event(|coords| is_dragged_fn(coords, &event.coords));

        let mut all_bindings = vec![];
        let mapping = &mapping.coords;
        for pointer_data in data {
            let mapping = mapping.filter_by_pointer_data(&pointer_data);
            let mapping = unwrap_or_continue!(mapping);
            let mapping = mapping.filter_by_modifiers(&self.modifiers);
            let bindings = unwrap_or_continue!(mapping);

            let coords = event.coords.clone();
            all_bindings.push((bindings, coords));
        }

        all_bindings
    }
}

fn with_timeout_event<'a, Sw, Mo, Co, Td, Bi>(
    mapping: &'a SwitchMappingCache<Sw, Mo, TimedEventData<Td>, (), Bi>,
    switch: Sw,
    modifiers: &Modifiers<Mo>,
    coords: Co,
    timed_processing: impl FnOnce(Sw) -> Option<TimedEventData<Td>>,
) -> Option<(FilteredBindings<'a, Mo, Bi>, Co)>
where
    Sw: Eq + Hash,
    Mo: Eq + Hash + Ord,
    Td: 'a + Eq + Hash,
{
    let mapping = mapping.filter_by_switch(&switch)?;
    let mapping = mapping.filter_by_modifiers(&modifiers)?;
    let timed_data = timed_processing(switch)?;
    let mapping = mapping.filter_by_timed_data(&timed_data)?;
    let bindings = mapping.filter_by_pointer_data(&())?;

    Some((bindings, coords))
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

/*
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
*/
