use input_core::{Modifiers, SchedulerState, TimedState};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct State<Mo, Ts, Sh> {
    pub modifiers: Mo,
    pub timed_state: Ts,
    pub scheduler_state: Sh,
}

impl<Mo, Ts, Sh> State<Mo, Ts, Sh> {
    pub fn new(modifiers: Mo, timed_state: Ts, scheduler_state: Sh) -> Self {
        Self {
            modifiers: modifiers,
            timed_state: timed_state,
            scheduler_state: scheduler_state,
        }
    }
}

pub trait TakeState<St> {
    type Rest;

    fn take_state(self) -> (St, Self::Rest);
}

pub trait WithState<St> {
    type Output;

    fn with_state(self, state: St) -> Self::Output;
}

impl<St> TakeState<St> for St {
    type Rest = ();

    fn take_state(self) -> (St, Self::Rest) {
        (self, ())
    }
}

impl<St> WithState<St> for () {
    type Output = St;

    fn with_state(self, state: St) -> Self::Output {
        state
    }
}

impl<Ts, Sh, Sw> TakeState<Modifiers<Sw>> for State<Modifiers<Sw>, Ts, Sh> {
    type Rest = State<(), Ts, Sh>;

    fn take_state(self) -> (Modifiers<Sw>, Self::Rest) {
        (
            self.modifiers,
            State::new((), self.timed_state, self.scheduler_state),
        )
    }
}

impl<Ts, Sh, Sw> WithState<Modifiers<Sw>> for State<(), Ts, Sh> {
    type Output = State<Modifiers<Sw>, Ts, Sh>;

    fn with_state(self, state: Modifiers<Sw>) -> Self::Output {
        State::new(state, self.timed_state, self.scheduler_state)
    }
}

impl<Mo, Sh, Sw> TakeState<TimedState<Sw>> for State<Mo, TimedState<Sw>, Sh> {
    type Rest = State<Mo, (), Sh>;

    fn take_state(self) -> (TimedState<Sw>, Self::Rest) {
        (
            self.timed_state,
            State::new(self.modifiers, (), self.scheduler_state),
        )
    }
}

impl<Mo, Sh, Sw> WithState<TimedState<Sw>> for State<Mo, (), Sh> {
    type Output = State<Mo, TimedState<Sw>, Sh>;

    fn with_state(self, state: TimedState<Sw>) -> Self::Output {
        State::new(self.modifiers, state, self.scheduler_state)
    }
}

impl<Mo, Ts, Ti, Sw, Re> TakeState<SchedulerState<Ti, Sw, Re>>
    for State<Mo, Ts, SchedulerState<Ti, Sw, Re>>
{
    type Rest = State<Mo, Ts, ()>;

    fn take_state(self) -> (SchedulerState<Ti, Sw, Re>, Self::Rest) {
        (
            self.scheduler_state,
            State::new(self.modifiers, self.timed_state, ()),
        )
    }
}

impl<Mo, Ts, Ti, Sw, Re> WithState<SchedulerState<Ti, Sw, Re>> for State<Mo, Ts, ()> {
    type Output = State<Mo, Ts, SchedulerState<Ti, Sw, Re>>;

    fn with_state(self, state: SchedulerState<Ti, Sw, Re>) -> Self::Output {
        State::new(self.modifiers, self.timed_state, state)
    }
}

/*use crate::{ConsWithLast, Processor};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PopFirstState;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PushLastState;

impl<Da, St, Re, Ar> Processor<(((), Da), ((St, Re), Ar))> for PopFirstState {
    type Output = ((St, Da), (Re, Ar));
    fn exec(
        &self,
        (((), data), ((state, rest), args)): (((), Da), ((St, Re), Ar)),
    ) -> Self::Output {
        ((state, data), (rest, args))
    }
}

impl<Da, St, Re, Ar> Processor<((St, Da), (Re, Ar))> for PushLastState
where
    Re: ConsWithLast<St>,
{
    type Output = (((), Da), (Re::Output, Ar));
    fn exec(&self, ((state, data), (rest, args)): ((St, Da), (Re, Ar))) -> Self::Output {
        (((), data), (rest.with_last(state), args))
    }
}


impl<Da, St, Re, Ar> Processor<((), Da, (St, Re), Ar)> for  {
    type Output = (St, Da, Re, Ar);
    fn exec(&self, ((), data, (state, rest), args): ((), Da, (St, Re), T)) -> Self::Output {
        (state, data, rest, args)
    }
}

impl<Da, St, Re, Ar> Processor<(St, Da, Re, Ar)> for StoreState {
    type Output = (St, Da, Re, Ar);
    fn exec(&self, ((), data, (state, rest), args): ((), Da, (St, Re), T)) -> Self::Output {
        (state, data, rest, args)
    }
}*/

/*
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct States {
    modifiers: Mo,
    timed_state: Ts,
    scheduler_state: Sh,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TakeModifiers;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StoreModifiers;

impl<Da, Mo, Ts, Sh> Processor<((), Da, State<Mo, Ts, Sh>)> for TakeModifiers {
    type Output = (Mo);
    fn exec(&self, (modifiers, switch, args): (Modifiers<Sw>, Sw, Args)) -> Self::Output {
        let (modifiers, result) = modifiers.with_press_event(switch);
        (modifiers, result, args)
    }
}
*/
/*
(A, (B, (C, ())))

(A, (B, (C, ())))


((), (C, (B, (A, ()))))
*/
