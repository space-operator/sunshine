/*use crate::EventWithModifiers;

pub trait EventWithModifiersExt<Ev1, Sw>: Sized {
    fn from<Ev2>(other: EventWithModifiers<Ev2, Sw>) -> Self
    where
        Ev1: From<Ev2>;
}

impl<Ev1, Sw> EventWithModifiersExt<Ev1, Sw> for EventWithModifiers<Ev1, Sw> {
    fn from<Ev2>(other: EventWithModifiers<Ev2, Sw>) -> Self
    where
        Ev1: From<Ev2>,
    {
        Self {
            event: other.event.into(),
            modifiers: other.modifiers,
        }
    }
}
*/
