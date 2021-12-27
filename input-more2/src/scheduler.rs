use core::borrow::Borrow;
use core::hash::Hash;

use input_core::SchedulerState;

use crate::{Context, Split, TakeState, WithState};

impl<St, Ev> Context<St, Ev> {
    pub fn with_scheduler<Re, Ki1, Ev2, Ki2, Ev3, Ki3, Ev4, Ti, Sw, Rq>(
        self,
    ) -> Context<Re::Output, Ev4>
    where
        St: TakeState<SchedulerState<Ti, Sw, Rq>, Rest = Re>,
        Re: WithState<SchedulerState<Ti, Sw, Rq>>,
        Ev: Split<Ti, Ev2, Ki1>,
        Ev2: Split<Sw, Ev3, Ki2>,
        Ev3: Split<Rq, Ev4, Ki3>,
        Ti: Ord,
    {
        let (state, rest) = self.state.take_state();
        let (time, event) = self.event.split();
        let (switch, event) = event.split();
        let (request, event) = event.split();
        let state = state.schedule(time, switch, request);
        Context::new(rest.with_state(state), event)
    }
}

impl<St, Ev> Context<St, Ev> {
    pub fn with_scheduled_taken<Re, Ki1, Ev2, Ti, TiRef, Sw, Rq>(
        self,
    ) -> Context<Re::Output, ((TiRef, Vec<(Ti, Vec<(Sw, Rq)>)>), Ev2)>
    where
        St: TakeState<SchedulerState<Ti, Sw, Rq>, Rest = Re>,
        Re: WithState<SchedulerState<Ti, Sw, Rq>>,
        Ev: Split<TiRef, Ev2, Ki1>,
        Ti: Ord,
        TiRef: Borrow<Ti>,
    {
        let (state, rest) = self.state.take_state();
        let (time, event) = self.event.split();
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

impl<Ti, Sw, Rq, Args> Processor<((SchedulerState<Ti, Sw, Rq>, (Ti, Sw, Rq)), Args)>
    for SchedulerProcessor
where
    Ti: Ord,
{
    type Output = ((SchedulerState<Ti, Sw, Rq>, ()), Args);
    fn exec(
        &self,
        ((state, (time, switch, request)), args): (
            (SchedulerState<Ti, Sw, Rq>, (Ti, Sw, Rq)),
            Args,
        ),
    ) -> Self::Output {
        ((state.schedule(time, switch, request), ()), args)
    }
}

impl<Ti, TiRef, Sw, Rq, Args> Processor<((SchedulerState<Ti, Sw, Rq>, TiRef), Args)>
    for ScheduledProcessor
where
    Ti: Ord,
    TiRef: Borrow<Ti>,
{
    type Output = (
        (
            SchedulerState<Ti, Sw, Rq>,
            (TiRef, Vec<(Ti, Vec<(Sw, Rq)>)>),
        ),
        Args,
    );
    fn exec(
        &self,
        ((state, time), args): ((SchedulerState<Ti, Sw, Rq>, TiRef), Args),
    ) -> Self::Output {
        ((state.take_scheduled(time)), args)
    }
}
