use std::sync::Arc;
use thiserror::Error;

use crate::{
    ButtonKind, CombinedEvent, CombinedInput, Duration, Event, MappedContext, ModifiedContext,
    ModifiedEvent, ModifiedState, Modifiers, ModifiersFilter, RawEvent, TimedContext, TimedEvent,
    TimedInputWithEventError, TimedInputWithTimeoutEventError, TimedState, TimedStateButtons,
    TimestampMs,
};

pub trait ProcessorContext: Sized {
    type Timeout;
    type CustomEvent;
    type MappedEvent: Clone;

    fn schedule(self, button: ButtonKind, delay: Duration) -> (Self, Arc<Self::Timeout>);

    fn events(
        &self,
        input: &CombinedInput<Self::CustomEvent>,
    ) -> Vec<(Self::MappedEvent, ModifiersFilter)>;

    fn emit(self, ev: Event<Self::MappedEvent>) -> Self;
}

#[derive(Clone, Debug)]
pub struct ProcessorState<C: ProcessorContext> {
    context: C,
    modifiers: Arc<Modifiers>,
    buttons: TimedStateButtons<C::Timeout>,
}

#[derive(Clone, Debug)]
pub struct ProcessorStateData<T> {
    modifiers: Arc<Modifiers>,
    buttons: TimedStateButtons<T>,
}

#[derive(Clone, Debug)]
pub struct ProcessorModifiedContext<C: ProcessorContext> {
    context: C,
    buttons: TimedStateButtons<C::Timeout>,
    result: Result<(), ProcessorWithEventError>,
}

#[derive(Clone, Debug)]
pub struct ProcessorTimedContext<C: ProcessorContext> {
    context: C,
}

impl<C: ProcessorContext> ProcessorState<C> {
    pub fn new(context: C) -> Self {
        Self {
            context,
            modifiers: Arc::default(),
            buttons: TimedStateButtons::default(),
        }
    }

    pub fn from_parts(data: ProcessorStateData<C::Timeout>, context: C) -> Self {
        Self {
            context,
            modifiers: data.modifiers,
            buttons: data.buttons,
        }
    }

    pub fn split(self) -> (ProcessorStateData<C::Timeout>, C) {
        (
            ProcessorStateData {
                modifiers: self.modifiers,
                buttons: self.buttons,
            },
            self.context,
        )
    }

    pub fn with_event(
        self,
        ev: RawEvent<C::CustomEvent>,
    ) -> (Self, Result<(), ProcessorWithEventError>) {
        let context: ProcessorModifiedContext<C> = ProcessorModifiedContext {
            context: self.context,
            buttons: self.buttons,
            result: Ok(()),
        };
        let state = ModifiedState::from_parts(self.modifiers, context);
        let state = state.with_event(ev);

        let (modifiers, context) = state.split();
        (
            Self {
                context: context.context,
                modifiers,
                buttons: context.buttons,
            },
            context.result.map_err(Into::into),
        )
    }

    pub fn with_timeout_event(
        self,
        button: ButtonKind,
        timestamp: TimestampMs,
    ) -> (Self, Result<(), ProcessorWithTimeoutEventError>) {
        let context: ProcessorTimedContext<C> = ProcessorTimedContext {
            context: self.context,
        };
        let state = TimedState::from_parts(self.buttons, context);
        let (state, result) = state.with_timeout_event(button, timestamp);
        let (buttons, context) = state.split();
        (
            Self {
                context: context.context,
                modifiers: self.modifiers,
                buttons,
            },
            result.map_err(Into::into),
        )
    }
}

impl<C> ModifiedContext for ProcessorModifiedContext<C>
where
    C: ProcessorContext,
{
    type CustomEvent = C::CustomEvent;

    fn emit_event(self, ev: ModifiedEvent<Self::CustomEvent>) -> Self {
        let ev_without_custom = ev.clone_without_custom();
        let event = CombinedEvent {
            input: CombinedInput::Modified(ev.input),
            modifiers: ev.modifiers.clone(),
            timestamp: ev.timestamp,
        };
        let context = self.context.process(event);
        let context = ProcessorTimedContext { context };
        let state = TimedState::from_parts(self.buttons, context);
        let (state, result) = state.with_event(ev_without_custom);
        let (buttons, context) = state.split();
        Self {
            context: context.context,
            buttons,
            result: result.map_err(Into::into),
        }
    }
}

impl<C> TimedContext for ProcessorTimedContext<C>
where
    C: ProcessorContext,
{
    type Timeout = C::Timeout;

    fn schedule(mut self, button: ButtonKind, delay: Duration) -> (Self, Arc<Self::Timeout>) {
        let (context, timeout) = C::schedule(self.context, button, delay);
        self = Self { context };
        (self, timeout)
    }

    fn emit_event(self, ev: TimedEvent) -> Self {
        let event = CombinedEvent {
            input: CombinedInput::Timed(ev.input),
            modifiers: ev.modifiers.clone(),
            timestamp: ev.timestamp,
        };
        let context = self.context.process(event);
        Self { context }
    }
}

impl<C: ProcessorContext> MappedContext for C {
    type CustomEvent = C::CustomEvent;
    type MappedEvent = C::MappedEvent;

    fn events(
        &self,
        input: &CombinedInput<Self::CustomEvent>,
    ) -> Vec<(Self::MappedEvent, ModifiersFilter)> {
        C::events(self, input)
    }

    fn emit(self, ev: Event<Self::MappedEvent>) -> Self {
        ProcessorContext::emit(self, ev)
    }
}

impl<T> Default for ProcessorStateData<T> {
    fn default() -> Self {
        Self {
            modifiers: Arc::new(Modifiers::default()),
            buttons: TimedStateButtons::default(),
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum ProcessorWithEventError {
    #[error(transparent)]
    TimedInput(#[from] TimedInputWithEventError),
}

#[derive(Clone, Debug, Error)]
pub enum ProcessorWithTimeoutEventError {
    #[error(transparent)]
    TimedInput(#[from] TimedInputWithTimeoutEventError),
}

#[test]
fn test() {
    use crate::{KeyboardKey, RawInput};

    struct Processor;

    impl ProcessorContext for Processor {
        type Timeout = ();
        type CustomEvent = ();
        type MappedEvent = &'static str;

        fn schedule(self, button: ButtonKind, delay: Duration) -> (Self, Arc<Self::Timeout>) {
            dbg!(button, delay);
            (self, Arc::new(()))
        }

        fn events(
            &self,
            input: &CombinedInput<Self::CustomEvent>,
        ) -> Vec<(Self::MappedEvent, ModifiersFilter)> {
            dbg!(input);
            vec![("SpaceDblClick", ModifiersFilter::default())]
        }

        fn emit(self, ev: Event<Self::MappedEvent>) -> Self {
            dbg!(ev);
            self
        }
    }

    let state = ProcessorState {
        context: Processor,
        modifiers: Arc::new(Modifiers::default()),
        buttons: TimedStateButtons::default(),
    };

    let (state, err) = state.with_event(RawEvent::new(RawInput::KeyDown(KeyboardKey::Space), 1000));
    err.unwrap();
    let (state, err) = state.with_event(RawEvent::new(RawInput::KeyUp(KeyboardKey::Space), 1150));
    err.unwrap();
    let (state, err) = state.with_event(RawEvent::new(RawInput::KeyDown(KeyboardKey::Space), 1200));
    err.unwrap();
    let (state, err) = state.with_event(RawEvent::new(RawInput::KeyUp(KeyboardKey::Space), 1300));
    err.unwrap();

    let _ = state;

    let ev = RawEvent::<()>::new(RawInput::KeyDown(KeyboardKey::Space), 1000);
    println!("{:?}", serde_json::to_string(&ev));
    //panic!();
}
