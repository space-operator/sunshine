use core::borrow::Borrow;
use core::hash::Hash;

use input_core::{
    LongPressHandleRequest, MultiClickHandleRequest, TimedLongClickError, TimedLongPressEventData,
    TimedMultiClickError, TimedMultiClickEventData, TimedPressError, TimedReleaseError,
    TimedReleaseEventData, TimedState,
};

use crate::{Context, TakeRequest, TakeState, TakeSwitch, WithState};

impl<St, Ev> Context<St, Ev> {
    pub fn with_timed_press_event<Re, Sw>(
        self,
    ) -> Context<Re::Output, (Result<LongPressHandleRequest, TimedPressError>, Ev::Rest)>
    where
        St: TakeState<TimedState<Sw>, Rest = Re>,
        Re: WithState<TimedState<Sw>>,
        Ev: TakeSwitch<Switch = Sw>,
        Sw: Hash + Ord,
    {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.take_switch();
        let (state, result) = state.with_press_event(switch);
        Context::new(rest.with_state(state), (result, event))
    }
}

impl<St, Ev> Context<St, Ev> {
    pub fn with_timed_release_event<Re, Sw>(
        self,
    ) -> Context<
        Re::Output,
        (
            Result<Option<(TimedReleaseEventData, MultiClickHandleRequest)>, TimedReleaseError>,
            Ev::Rest,
        ),
    >
    where
        St: TakeState<TimedState<Sw>, Rest = Re>,
        Re: WithState<TimedState<Sw>>,
        Ev: TakeSwitch<Switch = Sw>,
        Sw: Hash + Ord,
    {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.take_switch();
        let (state, result) = state.with_release_event(switch);
        Context::new(rest.with_state(state), (result, event))
    }
}

impl<St, Ev> Context<St, Ev> {
    pub fn with_long_press_event<Re, Ev2, Sw>(
        self,
    ) -> Context<
        Re::Output,
        (
            Option<Result<TimedLongPressEventData, TimedLongClickError>>,
            Ev2::Rest,
        ),
    >
    where
        St: TakeState<TimedState<Sw>, Rest = Re>,
        Re: WithState<TimedState<Sw>>,
        Ev: TakeSwitch<Switch = Sw, Rest = Ev2>,
        Ev2: TakeRequest<Request = LongPressHandleRequest>,
        Sw: Hash + Ord,
    {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.take_switch();
        let (request, event) = event.take_request();
        let (state, result) = state.with_long_press_event(switch, request);
        Context::new(rest.with_state(state), (result, event))
    }
}

impl<St, Ev> Context<St, Ev> {
    pub fn with_multi_click_event<Re, Ev2, Sw>(
        self,
    ) -> Context<
        Re::Output,
        (
            Option<Result<TimedMultiClickEventData, TimedMultiClickError>>,
            Ev2::Rest,
        ),
    >
    where
        St: TakeState<TimedState<Sw>, Rest = Re>,
        Re: WithState<TimedState<Sw>>,
        Ev: TakeSwitch<Switch = Sw, Rest = Ev2>,
        Ev2: TakeRequest<Request = MultiClickHandleRequest>,
        Sw: Hash + Ord,
    {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.take_switch();
        let (request, event) = event.take_request();
        let (state, result) = state.with_multi_click_event(switch, request);
        Context::new(rest.with_state(state), (result, event))
    }
}

//

use crate::Processor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimedPressProcessor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimedReleaseProcessor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimedLongPressProcessor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimedMultiClickProcessor;

impl<Sw, Args> Processor<((TimedState<Sw>, Sw), Args)> for TimedPressProcessor
where
    Sw: Eq + Hash,
{
    type Output = (
        (
            TimedState<Sw>,
            Result<LongPressHandleRequest, TimedPressError>,
        ),
        Args,
    );
    fn exec(&self, ((state, switch), args): ((TimedState<Sw>, Sw), Args)) -> Self::Output {
        let (state, result) = state.with_press_event(switch);
        ((state, result), args)
    }
}

impl<Sw, Args> Processor<((TimedState<Sw>, Sw), Args)> for TimedReleaseProcessor
where
    Sw: Eq + Hash,
{
    type Output = (
        (
            TimedState<Sw>,
            Result<Option<(TimedReleaseEventData, MultiClickHandleRequest)>, TimedReleaseError>,
        ),
        Args,
    );
    fn exec(&self, ((state, switch), args): ((TimedState<Sw>, Sw), Args)) -> Self::Output {
        let (state, result) = state.with_release_event(switch);
        ((state, result), args)
    }
}

impl<Sw, Args> Processor<((TimedState<Sw>, (Sw, LongPressHandleRequest)), Args)>
    for TimedLongPressProcessor
where
    Sw: Eq + Hash,
{
    type Output = (
        (
            TimedState<Sw>,
            Option<Result<TimedLongPressEventData, TimedLongClickError>>,
        ),
        Args,
    );
    fn exec(
        &self,
        ((state, (switch, request)), args): ((TimedState<Sw>, (Sw, LongPressHandleRequest)), Args),
    ) -> Self::Output {
        let (state, result) = state.with_long_press_event(switch, request);
        ((state, result), args)
    }
}

impl<Sw, Args> Processor<((TimedState<Sw>, (Sw, MultiClickHandleRequest)), Args)>
    for TimedMultiClickProcessor
where
    Sw: Eq + Hash,
{
    type Output = (
        (
            TimedState<Sw>,
            Option<Result<TimedMultiClickEventData, TimedMultiClickError>>,
        ),
        Args,
    );
    fn exec(
        &self,
        ((state, (switch, request)), args): ((TimedState<Sw>, (Sw, MultiClickHandleRequest)), Args),
    ) -> Self::Output {
        let (state, result) = state.with_multi_click_event(switch, request);
        ((state, result), args)
    }
}
