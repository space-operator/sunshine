use core::borrow::Borrow;
use core::hash::Hash;

use input_core::SchedulerState;

use crate::Processor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SchedulerProcessor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScheduledProcessor;

impl<Ti, Re, Args> Processor<((SchedulerState<Ti, Re>, (Ti, Re)), Args)> for SchedulerProcessor
where
    Ti: Ord,
{
    type Output = ((SchedulerState<Ti, Re>, ()), Args);
    fn exec(
        &self,
        ((state, (time, request)), args): ((SchedulerState<Ti, Re>, (Ti, Re)), Args),
    ) -> Self::Output {
        ((state.schedule(time, request), ()), args)
    }
}

impl<Ti, TiRef, Re, Args> Processor<((SchedulerState<Ti, Re>, TiRef), Args)> for ScheduledProcessor
where
    Ti: Ord,
    TiRef: Borrow<Ti>,
{
    type Output = ((SchedulerState<Ti, Re>, (TiRef, Vec<Re>)), Args);
    fn exec(&self, ((state, time), args): ((SchedulerState<Ti, Re>, TiRef), Args)) -> Self::Output {
        ((state.scheduled(time)), args)
    }
}
