use crate::{EventWithModifiers, ModifiedInput, TimedInput};

pub type CombinedEvent<T> = EventWithModifiers<CombinedInput<T>>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CombinedInput<T> {
    Modified(ModifiedInput<T>),
    Timed(TimedInput),
}

/*
pub enum EventOrTimedEvent<T1, T2> {
    Event(T1),
    TimedEvent(TimedEvent<T2>),
}
*/
