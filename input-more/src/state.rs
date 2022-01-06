use input_core::SchedulerState;

use crate::{
    define_markers, define_struct_take_and_with_field, /*GlobalMapping, SwitchEvent*/
    SwitchBindings,
};

#[derive(Clone, Debug, Default)]
pub struct State<Mo, Ts, Sh, Po> {
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

define_struct_take_and_with_field!(State {
    modifiers: Mo + ModifiersMarker,
    timed_state: Ts + TimedStateMarker,
    scheduler: Sh + SchedulerMarker,
    pointer_state: Po + PointerMarker,
});

impl<Mo, Ts, Sh, Po> State<Mo, Ts, Sh, Po> {
    pub fn new(modifiers: Mo, timed_state: Ts, scheduler: Sh, pointer_state: Po) -> Self {
        Self {
            modifiers,
            timed_state,
            scheduler,
            pointer_state,
        }
    }
}

/* TODO:
impl<Mo, Ts, Ti, ShDa, Rq> State<Mo, Ts, SchedulerState<Ti, ShDa, Rq>> {
    pub fn with_press_event<'a, Ev, Sw, Bi, Bu, EvDa>(
        self,
        event: Ev,
        bindings: &'a Bi,
    ) -> (Self, Rq, Option<(Bindings<'a, Sw, Bu>, EvDa)>) {
        let State {
            modifiers,
            timed_state,
            scheduler,
        } = self;

        todo!();
    }
}*/

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
    let state = State::new(1, false, "123", (1, 2));

    let (modifiers, rest): (i32, _) = state.take_field();
    let state: State<_, _, _, _> = rest.with_field(modifiers + 10);

    let (timed, rest): (bool, _) = state.take_field();
    let state: State<_, _, _, _> = rest.with_field(!timed);

    let (scheduler, rest): (&str, _) = state.take_field();
    let state: State<_, _, _, _> = rest.with_field(&scheduler[1..3]);

    assert_eq!(state.modifiers, 11);
    assert_eq!(state.timed_state, true);
    assert_eq!(state.scheduler, "23");
}
