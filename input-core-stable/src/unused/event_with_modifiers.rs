use core::hash::Hash;

use crate::Modifiers;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EventWithModifiers<Ev, Sw> {
    pub event: Ev,
    pub modifiers: Modifiers<Sw>,
}

impl<Ev, Sw> EventWithModifiers<Ev, Sw> {
    pub fn new(event: Ev, modifiers: Modifiers<Sw>) -> Self {
        Self { event, modifiers }
    }
}
