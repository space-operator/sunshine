use core::borrow::Borrow;
use core::hash::Hash;

use input_core::{
    LongPressHandleRequest, LongPressSchedulerState, MultiClickHandleRequest,
    MultiClickSchedulerState,
};

use crate::Processor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LongPressSchedulerProcessor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MultiClickSchedulerProcessor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScheduledLongPressProcessor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScheduledMultiClickProcessor;

type Lpr = LongPressHandleRequest;
type Mcr = MultiClickHandleRequest;

impl<Ti, Args> Processor<(LongPressSchedulerState<Ti>, (Ti, Lpr), Args)>
    for LongPressSchedulerProcessor
where
    Ti: Ord,
{
    type Output = (LongPressSchedulerState<Ti>, (), Args);
    fn exec(
        &self,
        (state, (time, request), args): (LongPressSchedulerState<Ti>, (Ti, Lpr), Args),
    ) -> Self::Output {
        let state = state.schedule(time, request);
        (state, (), args)
    }
}

impl<Ti, Args> Processor<(MultiClickSchedulerState<Ti>, (Ti, Mcr), Args)>
    for MultiClickSchedulerProcessor
where
    Ti: Ord,
{
    type Output = (MultiClickSchedulerState<Ti>, (), Args);
    fn exec(
        &self,
        (state, (time, request), args): (MultiClickSchedulerState<Ti>, (Ti, Mcr), Args),
    ) -> Self::Output {
        let state = state.schedule(time, request);
        (state, (), args)
    }
}

impl<TiRef, Ti, Args> Processor<(LongPressSchedulerState<Ti>, TiRef, Args)>
    for ScheduledLongPressProcessor
where
    Ti: Ord,
    TiRef: Borrow<Ti>,
{
    type Output = (LongPressSchedulerState<Ti>, Vec<Lpr>, Args);
    fn exec(
        &self,
        (state, time, args): (LongPressSchedulerState<Ti>, TiRef, Args),
    ) -> Self::Output {
        let (state, requests) = state.scheduled(time.borrow());
        (state, requests, args)
    }
}

impl<TiRef, Ti, Args> Processor<(MultiClickSchedulerState<Ti>, TiRef, Args)>
    for ScheduledLongPressProcessor
where
    Ti: Ord,
    TiRef: Borrow<Ti>,
{
    type Output = (MultiClickSchedulerState<Ti>, Vec<Mcr>, Args);
    fn exec(
        &self,
        (state, time, args): (MultiClickSchedulerState<Ti>, TiRef, Args),
    ) -> Self::Output {
        let (state, requests) = state.scheduled(time.borrow());
        (state, requests, args)
    }
}
