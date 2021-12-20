use core::hash::Hash;

use input_core::{
    LongPressHandleRequest, MultiClickHandleRequest, TimedLongClickError, TimedLongPressEventData,
    TimedMultiClickError, TimedMultiClickEventData, TimedPressError, TimedReleaseError,
    TimedReleaseEventData, TimedState,
};

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
