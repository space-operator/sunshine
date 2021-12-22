pub trait TakeSwitch<Sw> {
    type Rest;

    fn take_switch(self) -> (Sw, Self::Rest);
}

pub trait TakeTime<Ti> {
    type Rest;

    fn take_time(self) -> (Ti, Self::Rest);
}

pub trait TakeRequest<Rq> {
    type Rest;

    fn take_request(self) -> (Rq, Self::Rest);
}

impl<Sw> TakeSwitch<Sw> for Sw {
    type Rest = ();

    fn take_switch(self) -> (Sw, Self::Rest) {
        (self, ())
    }
}

impl<Ti> TakeTime<Ti> for Ti {
    type Rest = ();

    fn take_time(self) -> (Ti, Self::Rest) {
        (self, ())
    }
}

impl<Eq> TakeRequest<Eq> for Eq {
    type Rest = ();

    fn take_request(self) -> (Eq, Self::Rest) {
        (self, ())
    }
}

/*
pub trait TakeRequestTime<Ti> {
    type Rest;

    fn take_request_time(self) -> (Ti, Self::Rest);
}
*/

/*
use core::fmt::Debug;

use crate::Processor;


pub trait SplitEvent {
    type Data;
    type Event;

    fn split(self) -> (Self::Data, Self::Event);
}

pub trait UpgradeEvent<St, Da> {
    type State;
    type Event;

    fn upgrade(self, state: St, data: Da) -> (Self::State, Self::Event);
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithSplittedEvent;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithUpgradedEvent;

impl<St, Re, Ev> Processor<((St, ()), (Re, Ev))> for WithSplittedEvent
where
    Ev: SplitEvent,
{
    type Output = ((St, Ev::Data), (Re, Ev::Event));
    fn exec(&self, ((state, ()), (rest, event)): ((St, ()), (Re, Ev))) -> Self::Output {
        let (data, event) = event.split();
        ((state, data), (rest, event))
    }
}

impl<St, Da, Re, Ev> Processor<((St, Da), (Re, Ev))> for WithUpgradedEvent
where
    Ev: UpgradeEvent<St, Da>,
{
    type Output = ((Ev::State, ()), (Re, Ev::Event));
    fn exec(&self, ((state, data), (rest, event)): ((St, Da), (Re, Ev))) -> Self::Output {
        let (state, event) = event.upgrade(data);
        ((state, ()), (rest, event))
    }
}
*/
