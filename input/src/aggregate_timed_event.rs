use crate::*;

type AggregateTimedEvent<Sw> = TimedEvent<Sw, AggregateTimedEventKind>;

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
}
