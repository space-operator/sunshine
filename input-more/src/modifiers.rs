use core::hash::Hash;

use input_core::{Modifiers, ModifiersPressError, ModifiersReleaseError};

use crate::Processor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModifiersPressProcessor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModifiersReleaseProcessor;

impl<Sw, Args> Processor<(Modifiers<Sw>, Sw, Args)> for ModifiersPressProcessor
where
    Sw: Clone + Eq + Hash + Ord,
{
    type Output = (Modifiers<Sw>, Result<(), ModifiersPressError>, Args);
    fn exec(&self, (modifiers, switch, args): (Modifiers<Sw>, Sw, Args)) -> Self::Output {
        let (modifiers, result) = modifiers.with_press_event(switch);
        (modifiers, result, args)
    }
}

impl<Sw, Args> Processor<(Modifiers<Sw>, Sw, Args)> for ModifiersReleaseProcessor
where
    Sw: Clone + Eq + Hash + Ord,
{
    type Output = (Modifiers<Sw>, Result<(), ModifiersReleaseError>, Args);
    fn exec(&self, (modifiers, switch, args): (Modifiers<Sw>, Sw, Args)) -> Self::Output {
        let (modifiers, result) = modifiers.with_release_event(switch);
        (modifiers, result, args)
    }
}
