pub trait Split<T1, T2, T3> {
    fn split(self) -> (T1, T2);
}

pub trait AllowSplitFromItself<T> {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Event<Ti, Sw, Rq, Co, Dr, Mo, Td, Po, Da> {
    time: Ti,
    switch: Sw,
    request: Rq,
    coords: Co,
    is_dragged_fn: Dr,
    modifiers: Mo,
    timed_data: Td,
    pointer: Po,
    data: Da,
}

impl<T1, T3> Split<T1, (), T3> for T1
where
    T1: AllowSplitFromItself<T3>,
{
    fn split(self) -> (T1, ()) {
        (self, ())
    }
}

impl Event<(), (), (), (), (), (), (), (), ()> {
    pub fn new() -> Self {
        Self {
            time: (),
            switch: (),
            request: (),
            coords: (),
            is_dragged_fn: (),
            modifiers: (),
            timed_data: (),
            pointer: (),
            data: (),
        }
    }
}

macro_rules! impl_with {
    ( $name:ident, ( $($before:ident),* ), ( $($after:ident),* ) ) => {
        impl<$($before ,)* $($after),*> Event<$($before ,)* () $(, $after)*> {
            pub fn $name<T>(self, value: T) -> Event<$($before ,)* T $(, $after)*> {
                todo!();
            }
        }
    }
}

/*
macro_rules! impl_with {
    ($sw_flag) => {
        impl <
            generic_or_nothing!($sw_flag, Sw)
            generic_or_nothing!($sw_flag, Sw)
            generic_or_nothing!($sw_flag, Sw)
            generic_or_nothing!($sw_flag, Sw)
            generic_or_nothing!($sw_flag, Sw)
            generic_or_nothing!($sw_flag, Sw)
            generic_or_nothing!($sw_flag, Sw)
            generic_or_nothing!($sw_flag, Sw)
            generic_or_nothing!($sw_flag, Sw)
        >
    }
}

macro_rules! generic_or_nothing {
    (1, value) => value,
    (0, value) => (),
}
*/

//impl_with!(with_time, (1 0 0 0 0 0 0 0 0));
//impl_with!(with_switch, (0 1 0 0 0 0 0 0 0));

impl_with!(with_time, (), (Sw, Rq, Co, Dr, Mo, Td, Po, Da));
impl_with!(with_switch, (Ti), (Rq, Co, Dr, Mo, Td, Po, Da));
impl_with!(with_request, (Ti, Sw), (Co, Dr, Mo, Td, Po, Da));

/*
impl<Sw, Rq, Co, Dr, Mo, Td, Po, Da> Event<(), Sw, Rq, Co, Dr, Mo, Td, Po, Da> {
    pub fn with_time<Ti>(self, time: Ti) -> Event<Ti, Sw, Rq, Co, Dr, Mo, Td, Po, Da> {
        Event {
            time,
            switch: self.switch,
            request: self.request,
            coords: self.coords,
            is_dragged_fn: self.is_dragged_fn,
            modifiers: self.modifiers,
            timed_data: self.timed_data,
            pointer: self.pointer,
            data: self.data,
        }
    }
}*/

/*
pub trait Take<T> {
    type Rest;

    fn take(self) -> (T, Self::Rest);
}

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

pub trait TakeCoords<Co> {
    type Rest;

    fn take_coords(self) -> (Co, Self::Rest);
}

pub trait TakeIsDraggedFn<Dr> {
    type Rest;

    fn take_is_dragged_fn(self) -> (Dr, Self::Rest);
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
}*/

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
