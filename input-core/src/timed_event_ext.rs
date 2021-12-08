use core::hash::Hash;

use thiserror::Error;

use crate::*;

pub type TimedDelayedEvent = TimedEvent<TimedDelayedEventKind>;
pub type TimedCombinedEvent = TimedEvent<TimedCombinedEventKind>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimedDelayedEventKind {
    LongPress,
    ClickExact,
    LongClickExact,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimedCombinedEventKind {
    Click,
    LongClick,
    LongPress,
    ClickExact,
    LongClickExact,
}

impl From<TimedLongPressEventKind> for TimedDelayedEventKind {
    fn from(kind: TimedLongPressEventKind) -> Self {
        match kind {
            TimedLongPressEventKind::LongPress => Self::LongPress,
        }
    }
}

impl From<TimedMultiClickEventKind> for TimedDelayedEventKind {
    fn from(kind: TimedMultiClickEventKind) -> Self {
        match kind {
            TimedMultiClickEventKind::ClickExact => Self::ClickExact,
            TimedMultiClickEventKind::LongClickExact => Self::LongClickExact,
        }
    }
}

impl From<TimedReleaseEventKind> for TimedCombinedEventKind {
    fn from(kind: TimedReleaseEventKind) -> Self {
        match kind {
            TimedReleaseEventKind::Click => Self::Click,
            TimedReleaseEventKind::LongClick => Self::LongClick,
        }
    }
}

impl From<TimedLongPressEventKind> for TimedCombinedEventKind {
    fn from(kind: TimedLongPressEventKind) -> Self {
        match kind {
            TimedLongPressEventKind::LongPress => Self::LongPress,
        }
    }
}

impl From<TimedMultiClickEventKind> for TimedCombinedEventKind {
    fn from(kind: TimedMultiClickEventKind) -> Self {
        match kind {
            TimedMultiClickEventKind::ClickExact => Self::ClickExact,
            TimedMultiClickEventKind::LongClickExact => Self::LongClickExact,
        }
    }
}

impl From<TimedDelayedEventKind> for TimedCombinedEventKind {
    fn from(kind: TimedDelayedEventKind) -> Self {
        match kind {
            TimedDelayedEventKind::LongPress => Self::LongPress,
            TimedDelayedEventKind::ClickExact => Self::ClickExact,
            TimedDelayedEventKind::LongClickExact => Self::LongClickExact,
        }
    }
}

pub trait AllowFrom<T> {}

impl AllowFrom<TimedReleaseEventKind> for TimedCombinedEventKind {}
impl AllowFrom<TimedLongPressEventKind> for TimedCombinedEventKind {}
impl AllowFrom<TimedMultiClickEventKind> for TimedCombinedEventKind {}
impl AllowFrom<TimedDelayedEventKind> for TimedCombinedEventKind {}

impl AllowFrom<TimedLongPressEventKind> for TimedDelayedEventKind {}
impl AllowFrom<TimedMultiClickEventKind> for TimedDelayedEventKind {}

impl<Ki1> From<TimedEvent<Ki1>> for TimedCombinedEvent
where
    TimedCombinedEventKind: From<Ki1> + AllowFrom<Ki1>,
{
    fn from(event: TimedEvent<Ki1>) -> Self {
        Self {
            kind: event.kind.into(),
            num_possible_clicks: event.num_possible_clicks,
        }
    }
}

impl<Ki1> From<TimedEvent<Ki1>> for TimedDelayedEvent
where
    TimedDelayedEventKind: From<Ki1> + AllowFrom<Ki1>,
{
    fn from(event: TimedEvent<Ki1>) -> Self {
        Self {
            kind: event.kind.into(),
            num_possible_clicks: event.num_possible_clicks,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TimedHandleRequest {
    LongPress(LongPressHandleRequest),
    MultiClick(MultiClickHandleRequest),
}

impl From<LongPressHandleRequest> for TimedHandleRequest {
    fn from(request: LongPressHandleRequest) -> Self {
        Self::LongPress(request)
    }
}

impl From<MultiClickHandleRequest> for TimedHandleRequest {
    fn from(request: MultiClickHandleRequest) -> Self {
        Self::MultiClick(request)
    }
}

pub trait TimedStateExt<Sw>: Sized {
    fn with_delayed_event(
        self,
        switch: Sw,
        request: TimedHandleRequest,
    ) -> (Self, Option<Result<TimedDelayedEvent, TimedDelayedError>>)
    where
        Sw: Eq + Hash;
}

impl<Sw> TimedStateExt<Sw> for TimedState<Sw> {
    fn with_delayed_event(
        self,
        switch: Sw,
        request: TimedHandleRequest,
    ) -> (Self, Option<Result<TimedDelayedEvent, TimedDelayedError>>)
    where
        Sw: Eq + Hash,
    {
        match request {
            TimedHandleRequest::LongPress(request) => {
                IntoDelayed::into_delayed(self.with_long_press_event(switch, request))
            }
            TimedHandleRequest::MultiClick(request) => {
                IntoDelayed::into_delayed(self.with_multi_click_event(switch, request))
            }
        }
    }
}

trait IntoDelayed<Sw> {
    fn into_delayed(
        self,
    ) -> (
        TimedState<Sw>,
        Option<Result<TimedDelayedEvent, TimedDelayedError>>,
    );
}

impl<Sw, T, E> IntoDelayed<Sw> for (TimedState<Sw>, Option<Result<T, E>>)
where
    Sw: Eq + Hash,
    TimedDelayedEvent: From<T>,
    TimedDelayedError: From<E>,
{
    fn into_delayed(
        self,
    ) -> (
        TimedState<Sw>,
        Option<Result<TimedDelayedEvent, TimedDelayedError>>,
    ) {
        (
            self.0,
            self.1
                .map(|result| result.map_or_else(|err| Err(err.into()), |ok| Ok(ok.into()))),
        )
    }
}

#[derive(Clone, Copy, Debug, Error)]
pub enum TimedDelayedError {
    #[error(transparent)]
    LongClick(#[from] TimedLongClickError),
    #[error(transparent)]
    MultiClick(#[from] TimedMultiClickError),
}

/*use crate::*;

pub type AggregateTimedEvent<Sw> = TimedEvent<Sw, AggregateTimedEventKind>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AggregateTimedEventKind {
    Click,
    LongClick,
    LongPress,
    ClickExact,
    LongClickExact,
}

impl From<ImmediateTimedEventKind> for AggregateTimedEventKind {
    fn from(kind: ImmediateTimedEventKind) -> Self {
        match kind {
            ImmediateTimedEventKind::Click => Self::Click,
            ImmediateTimedEventKind::LongClick => Self::LongClick,
        }
    }
}

impl From<DelayedTimedEventKind> for AggregateTimedEventKind {
    fn from(kind: DelayedTimedEventKind) -> Self {
        match kind {
            DelayedTimedEventKind::LongPress => Self::LongPress,
            DelayedTimedEventKind::ClickExact => Self::ClickExact,
            DelayedTimedEventKind::LongClickExact => Self::LongClickExact,
        }
    }
}

pub trait IntoAggregate<T> {
    fn into_aggregate(self) -> T;
}

impl<Sw, Ki> IntoAggregate<AggregateTimedEvent<Sw>> for TimedEvent<Sw, Ki>
where
    AggregateTimedEventKind: From<Ki>,
{
    fn into_aggregate(self) -> AggregateTimedEvent<Sw> {
        AggregateTimedEvent {
            switch: self.switch,
            kind: self.kind.into(),
            num_switches: self.num_switches,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AggregateTimedTransition<Ev, Sw> {
    pub event: Option<Ev>,
    pub timed_event: Option<AggregateTimedEvent<Sw>>,
    pub scheduled: Option<ScheduledTimeout<Sw>>,
    pub state: TimedState<Sw>,
}

impl<Ev, Sw> IntoAggregate<AggregateTimedTransition<Ev, Sw>> for ImmediateTimedTransition<Ev, Sw> {
    fn into_aggregate(self) -> AggregateTimedTransition<Ev, Sw> {
        AggregateTimedTransition {
            event: Some(self.event),
            timed_event: self.timed_event.map(TimedEvent::into_aggregate),
            scheduled: self.scheduled,
            state: self.state,
        }
    }
}

impl<Ev, Sw> IntoAggregate<AggregateTimedTransition<Ev, Sw>> for DelayedTimedTransition<Sw> {
    fn into_aggregate(self) -> AggregateTimedTransition<Ev, Sw> {
        AggregateTimedTransition {
            event: None,
            timed_event: self.timed_event.map(TimedEvent::into_aggregate),
            scheduled: self.scheduled,
            state: self.state,
        }
    }
}*/

/*
impl<Sw1, Sw2, Ki1, Ki2> From<TimedEvent<Sw2, Ki2>> for TimedEvent<Sw1, Ki1>
where
    Sw1: From<Sw2>,
    Ki1: From<Ki2>,
{
    fn from(event: TimedEvent<Sw2, Ki2>) -> Self {
        Self {
            switch: event.switch.into(),
            kind: event.kind.into(),
            num_switches: event.num_switches,
        }
    }
}
*/
