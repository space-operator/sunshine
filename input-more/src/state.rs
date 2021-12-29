use crate::{define_markers, define_struct_take_and_with_field, /*GlobalMapping, SwitchEvent*/};

#[derive(Clone, Debug, Default)]
pub struct State<Mo, Ts, Sh> {
    pub modifiers: Mo,
    pub timed_state: Ts,
    pub scheduler: Sh,
}

define_markers!(ModifiersMarker, TimedStateMarker, SchedulerMarker);

define_struct_take_and_with_field!(State {
    modifiers: Mo + ModifiersMarker,
    timed_state: Ts + TimedStateMarker,
    scheduler: Sh + SchedulerMarker,
});

impl<Mo, Ts, Sh> State<Mo, Ts, Sh> {
    pub fn new(modifiers: Mo, timed_state: Ts, scheduler: Sh) -> Self {
        Self {
            modifiers,
            timed_state,
            scheduler,
        }
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
}

#[test]
fn test() {
    use crate::{StructTakeField, StructWithField};
    let state = State::new(1, false, "123");

    let (modifiers, rest): (i32, _) = state.take_field();
    let state: State<_, _, _> = rest.with_field(modifiers + 10);

    let (timed, rest): (bool, _) = state.take_field();
    let state: State<_, _, _> = rest.with_field(!timed);

    let (scheduler, rest): (&str, _) = state.take_field();
    let state: State<_, _, _> = rest.with_field(&scheduler[1..3]);

    assert_eq!(state.modifiers, 11);
    assert_eq!(state.timed_state, true);
    assert_eq!(state.scheduler, "23");
}
