pub trait TakeSwitch {
    type Switch;
    type Rest;

    fn take_switch(self) -> (Self::Switch, Self::Rest);
}

// of <Ev>
pub trait TakeTime {
    type Time;
    type Rest;

    fn take_time(self) -> (Self::Time, Self::Rest);
}

pub trait TakeRequest {
    type Request;
    type Rest;

    fn take_request(self) -> (Self::Request, Self::Rest);
}
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
