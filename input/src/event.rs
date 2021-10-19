use std::sync::Arc;

use crate::Modifiers;

pub type TimestampMs = u64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event<T> {
    pub input: T,
    pub timestamp: TimestampMs,
}

#[derive(Clone, Debug)]
pub struct EventWithModifiers<T> {
    pub input: T,
    pub modifiers: Arc<Modifiers>,
    pub timestamp: TimestampMs,
}
