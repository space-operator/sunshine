use core::borrow::Borrow;
use core::hash::Hash;

use input_core::{Modifiers, ModifiersPressError, ModifiersReleaseError};

use crate::Processor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModifiersPressProcessor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModifiersReleaseProcessor;

impl<Sw, Args> Processor<((Modifiers<Sw>, Sw), Args)> for ModifiersPressProcessor
where
    Sw: Clone + Eq + Hash + Ord,
{
    type Output = ((Modifiers<Sw>, Result<(), ModifiersPressError>), Args);
    fn exec(&self, ((modifiers, switch), args): ((Modifiers<Sw>, Sw), Args)) -> Self::Output {
        (modifiers.with_press_event(switch), args)
    }
}

impl<Sw, SwRef, Args> Processor<((Modifiers<Sw>, SwRef), Args)> for ModifiersReleaseProcessor
where
    Sw: Clone + Eq + Hash + Ord,
    SwRef: Borrow<Sw>,
{
    type Output = (
        (Modifiers<Sw>, (SwRef, Result<(), ModifiersReleaseError>)),
        Args,
    );
    fn exec(&self, ((modifiers, switch), args): ((Modifiers<Sw>, SwRef), Args)) -> Self::Output {
        (modifiers.with_release_event(switch), args)
    }
}
