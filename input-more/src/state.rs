use crate::{define_markers, define_struct_from_into_cons_and_take_put};

#[derive(Clone, Debug, Default)]
pub struct State<Mo, Ts, Sh> {
    pub modifiers: Mo,
    pub timed_state: Ts,
    pub scheduler: Sh,
}

define_markers!(ModifiersMarker, TimedStateMarker, SchedulerMarker);

define_struct_from_into_cons_and_take_put!(
    State,
    modifiers: Mo + ModifiersMarker,
    timed_state: Ts + TimedStateMarker,
    scheduler: Sh + SchedulerMarker,
);

impl<Mo, Ts, Sh> State<Mo, Ts, Sh> {
    pub fn new(modifiers: Mo, timed_state: Ts, scheduler: Sh) -> Self {
        Self {
            modifiers,
            timed_state,
            scheduler,
        }
    }
}

#[test]
fn test() {
    use crate::{Put, Take};
    let state = State::new(1, false, "123");

    let (modifiers, rest): (i32, _) = state.take();
    let state: State<_, _, _> = rest.put(modifiers + 10);

    let (timed, rest): (bool, _) = state.take();
    let state: State<_, _, _> = rest.put(!timed);

    let (scheduler, rest): (&str, _) = state.take();
    let state: State<_, _, _> = rest.put(&scheduler[1..3]);

    assert_eq!(state.modifiers, 11);
    assert_eq!(state.timed_state, true);
    assert_eq!(state.scheduler, "23");
}
