use core::borrow::Borrow;
use core::hash::Hash;

use input_core::SchedulerState;

use crate::{Context, TakeRequest, TakeState, TakeSwitch, TakeTime, WithState};

impl<St, Ev> Context<St, Ev> {
    pub fn with_scheduler<Re, Ev2, Ev3, Ti, Sw, Rq>(self) -> Context<Re::Output, Ev3::Rest>
    where
        St: TakeState<SchedulerState<Ti, Sw, Rq>, Rest = Re>,
        Re: WithState<SchedulerState<Ti, Sw, Rq>>,
        Ev: TakeTime<Time = Ti, Rest = Ev2>,
        Ev2: TakeSwitch<Switch = Sw, Rest = Ev3>,
        Ev3: TakeRequest<Request = Rq>,
        Ti: Ord,
    {
        let (state, rest) = self.state.take_state();
        let (time, event) = self.event.take_time();
        let (switch, event) = event.take_switch();
        let (request, event) = event.take_request();
        let state = state.schedule(time, switch, request);
        Context::new(rest.with_state(state), event)
    }
}

impl<St, Ev> Context<St, Ev> {
    pub fn with_schedule_taken<Re, Ev2, Ev3, Ti, TiRef, Sw, Rq>(
        self,
    ) -> Context<Re::Output, ((TiRef, Vec<(Sw, Rq)>), Ev::Rest)>
    where
        St: TakeState<SchedulerState<Ti, Sw, Rq>, Rest = Re>,
        Re: WithState<SchedulerState<Ti, Sw, Rq>>,
        Ev: TakeTime<Time = TiRef>,
        Ti: Ord,
        TiRef: Borrow<Ti>,
    {
        let (state, rest) = self.state.take_state();
        let (time, event) = self.event.take_time();
        let (state, result) = state.take_scheduled(time);
        Context::new(rest.with_state(state), (result, event))
    }
}

//

use crate::Processor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SchedulerProcessor;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScheduledProcessor;

impl<Ti, Sw, Re, Args> Processor<((SchedulerState<Ti, Sw, Re>, (Ti, Sw, Re)), Args)>
    for SchedulerProcessor
where
    Ti: Ord,
{
    type Output = ((SchedulerState<Ti, Sw, Re>, ()), Args);
    fn exec(
        &self,
        ((state, (time, switch, request)), args): (
            (SchedulerState<Ti, Sw, Re>, (Ti, Sw, Re)),
            Args,
        ),
    ) -> Self::Output {
        ((state.schedule(time, switch, request), ()), args)
    }
}

impl<Ti, TiRef, Sw, Re, Args> Processor<((SchedulerState<Ti, Sw, Re>, TiRef), Args)>
    for ScheduledProcessor
where
    Ti: Ord,
    TiRef: Borrow<Ti>,
{
    type Output = ((SchedulerState<Ti, Sw, Re>, (TiRef, Vec<(Sw, Re)>)), Args);
    fn exec(
        &self,
        ((state, time), args): ((SchedulerState<Ti, Sw, Re>, TiRef), Args),
    ) -> Self::Output {
        ((state.take_scheduled(time)), args)
    }
}
