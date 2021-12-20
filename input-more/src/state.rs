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
