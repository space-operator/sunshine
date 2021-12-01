use crate::{EventWithModifiers, ModifiedInput, TimedInput};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CombinedEvent<Ev, Sw> {
    Event(Ev),
    Timed(AggregateTimedEvent<Sw>),
}

/*
pub enum EventOrTimedEvent<T1, T2> {
    Event(T1),
    TimedEvent(TimedEvent<T2>),
}
*/
