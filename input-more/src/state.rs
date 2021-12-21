use input_core::Modifiers;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct State<Mo, Ti, Sh> {
    modifiers: Mo,
    timed_state: Ti,
    scheduler_state: Sh,
}

impl<Mo, Ti, Sh> State<Mo, Ti, Sh> {
    fn new(modifiers: Mo, timed_state: Ti, scheduler_state: Sh) -> Self {
        Self {
            modifiers: modifiers,
            timed_state: timed_state,
            scheduler_state: scheduler_state,
        }
    }
}

pub trait TakeModifiers<Sw> {
    type Output;

    fn take_modifiers(self) -> (Modifiers<Sw>, Self::Output);
}

pub trait StoreModifiers<Sw> {
    type Output;

    fn store_modifiers(self, modifiers: Modifiers<Sw>) -> Self::Output;
}

impl<Sw, Ti, Sh> TakeModifiers<Sw> for State<Modifiers<Sw>, Ti, Sh> {
    type Output = State<(), Ti, Sh>;

    fn take_modifiers(self) -> (Modifiers<Sw>, Self::Output) {
        (
            self.modifiers,
            State::new((), self.timed_state, self.scheduler_state),
        )
    }
}

impl<Sw, Ti, Sh> StoreModifiers<Sw> for State<(), Ti, Sh> {
    type Output = State<Modifiers<Sw>, Ti, Sh>;

    fn store_modifiers(self, modifiers: Modifiers<Sw>) -> Self::Output {
        State::new(modifiers, self.timed_state, self.scheduler_state)
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
    timed_state: Ti,
    scheduler_state: Sh,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TakeModifiers;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StoreModifiers;

impl<Da, Mo, Ti, Sh> Processor<((), Da, State<Mo, Ti, Sh>)> for TakeModifiers {
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
