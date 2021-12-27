use core::borrow::Borrow;
use core::hash::Hash;

use input_core::{
    LongPressHandleRequest, MultiClickHandleRequest, TimedLongClickError, TimedLongPressEventData,
    TimedMultiClickError, TimedMultiClickEventData, TimedPressError, TimedReleaseError,
    TimedReleaseEventData, TimedState,
};

use crate::{Context, Split, TakeState, WithState};

impl<St, Ev> Context<St, Ev> {
    pub fn with_timed_press_event<Re, Ki1, Ev2, Sw>(
        self,
    ) -> Context<Re::Output, (Result<LongPressHandleRequest, TimedPressError>, Ev2)>
    where
        St: TakeState<TimedState<Sw>, Rest = Re>,
        Re: WithState<TimedState<Sw>>,
        Ev: Split<Sw, Ev2, Ki1>,
        Sw: Hash + Ord,
    {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.split();
        let (state, result) = state.with_press_event(switch);
        Context::new(rest.with_state(state), (result, event))
    }
}

impl<St, Ev> Context<St, Ev> {
    pub fn with_timed_release_event<Re, Ki1, Ev2, Sw>(
        self,
    ) -> Context<
        Re::Output,
        (
            Result<Option<(TimedReleaseEventData, MultiClickHandleRequest)>, TimedReleaseError>,
            Ev2,
        ),
    >
    where
        St: TakeState<TimedState<Sw>, Rest = Re>,
        Re: WithState<TimedState<Sw>>,
        Ev: Split<Sw, Ev2, Ki1>,
        Sw: Hash + Ord,
    {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.split();
        let (state, result) = state.with_release_event(switch);
        Context::new(rest.with_state(state), (result, event))
    }
}

impl<St, Ev> Context<St, Ev> {
    pub fn with_long_press_event<Re, Ki1, Ev2, Ki2, Ev3, Sw>(
        self,
    ) -> Context<
        Re::Output,
        (
            Option<Result<TimedLongPressEventData, TimedLongClickError>>,
            Ev3,
        ),
    >
    where
        St: TakeState<TimedState<Sw>, Rest = Re>,
        Re: WithState<TimedState<Sw>>,
        Ev: Split<Sw, Ev2, Ki1>,
        Ev2: Split<LongPressHandleRequest, Ev3, Ki2>,
        Sw: Hash + Ord,
    {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.split();
        let (request, event) = event.split();
        let (state, result) = state.with_long_press_event(switch, request);
        Context::new(rest.with_state(state), (result, event))
    }
}

impl<St, Ev> Context<St, Ev> {
    pub fn with_multi_click_event<Re, Ki1, Ev2, Ki2, Ev3, Sw>(
        self,
    ) -> Context<
        Re::Output,
        (
            Option<Result<TimedMultiClickEventData, TimedMultiClickError>>,
            Ev3,
        ),
    >
    where
        St: TakeState<TimedState<Sw>, Rest = Re>,
        Re: WithState<TimedState<Sw>>,
        Ev: Split<Sw, Ev2, Ki1>,
        Ev2: Split<MultiClickHandleRequest, Ev3, Ki2>,
        Sw: Hash + Ord,
    {
        let (state, rest) = self.state.take_state();
        let (switch, event) = self.event.split();
        let (request, event) = event.split();
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
