use crate::{EventWithModifiers, ModifiedInput, TimedInput};

pub type CombinedEvent = EventWithModifiers<CombinedInput>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CombinedInput {
    Modified(ModifiedInput),
    Timed(TimedInput),
}
