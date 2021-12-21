use core::borrow::Borrow;
use core::hash::Hash;

use input_core::{Modifiers, ModifiersPressError, ModifiersReleaseError};

use crate::{Context, TakeState, TakeSwitch, WithState};

//pub trait ModifiersProcessing {}

impl<St, Re, Ev, Sw> Context<St, Ev>
where
    St: TakeState<Modifiers<Sw>, Rest = Re>,
    Re: WithState<Modifiers<Sw>>,
    Ev: TakeSwitch<Switch = Sw>,
    Sw: Clone + Hash + Ord,
{
    pub fn with_modifiers_press_event(
        self,
    ) -> Context<Re::Output, (Result<(), ModifiersPressError>, Ev::Rest)> {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.take_switch();
        let (state, result) = state.with_press_event(switch);
        Context::new(rest.with_state(state), (result, event))
    }
}

impl<St, Re, Ev, Sw> Context<St, Ev>
where
    St: TakeState<Modifiers<Sw>, Rest = Re>,
    Re: WithState<Modifiers<Sw>>,
    Ev: TakeSwitch<Switch = Sw>, // SwRef
    Sw: Clone + Eq + Hash + Ord, // TODO
{
    pub fn with_modifiers_release_event(
        self,
    ) -> Context<Re::Output, ((Sw, Result<(), ModifiersReleaseError>), Ev::Rest)> {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.take_switch();
        let (state, result) = state.with_release_event(switch);
        Context::new(rest.with_state(state), (result, event))
    }
}

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
