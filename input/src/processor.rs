use std::{collections::HashMap, sync::Arc};

use crate::{
    ButtonKind, ButtonTimedState, CombinedEvent, CombinedInput, Duration, Event, MappedContext,
    ModifiedContext, ModifiedEvent, ModifiedState, Modifiers, ModifiersFilter, RawEvent,
    TimedContext, TimedEvent, TimedState, TimestampMs,
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

    fn emit(ev: Event<Self::MappedEvent>);
}

#[derive(Clone, Debug)]
pub struct ProcessorState<C: ProcessorContext> {
    processor: C,
    modifiers: Arc<Modifiers>,
    buttons: HashMap<ButtonKind, ButtonTimedState<C::Timeout>>,
}

#[derive(Clone, Debug)]
pub struct ProcessorModifiedContext<C: ProcessorContext> {
    processor: C,
    buttons: HashMap<ButtonKind, ButtonTimedState<C::Timeout>>,
}

#[derive(Clone, Debug)]
pub struct ProcessorTimedContext<C: ProcessorContext> {
    processor: C,
}

impl<C: ProcessorContext> ProcessorState<C> {
    pub fn with_event(self, ev: RawEvent<C::CustomEvent>) -> Self {
        let context: ProcessorModifiedContext<C> = ProcessorModifiedContext {
            processor: self.processor,
            buttons: self.buttons,
        };
        let state = ModifiedState::from_parts(self.modifiers, context);
        let state = state.with_event(ev);
        let (modifiers, context) = state.split();
        Self {
            processor: context.processor,
            modifiers,
            buttons: context.buttons,
        }
    }

    pub fn with_timeout_event(self, button: ButtonKind, timestamp: TimestampMs) -> Self {
        let context: ProcessorTimedContext<C> = ProcessorTimedContext {
            processor: self.processor,
        };
        let state = TimedState::from_parts(self.buttons, context);
        let state = state.with_timeout_event(button, timestamp);
        let (buttons, context) = state.split();
        Self {
            processor: context.processor,
            modifiers: self.modifiers,
            buttons: buttons,
        }
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
        self.processor.process(event, C::emit);
        let context = ProcessorTimedContext {
            processor: self.processor,
        };
        let state = TimedState::from_parts(self.buttons, context);
        let state = state.with_event(ev_without_custom);
        let (buttons, context) = state.split();
        Self {
            processor: context.processor,
            buttons,
        }
    }
}

impl<C> TimedContext for ProcessorTimedContext<C>
where
    C: ProcessorContext,
{
    type Timeout = C::Timeout;

    fn schedule(mut self, button: ButtonKind, delay: Duration) -> (Self, Arc<Self::Timeout>) {
        let (processor, timeout) = C::schedule(self.processor, button, delay);
        self = Self { processor };
        (self, timeout)
    }

    fn emit_event(self, ev: TimedEvent) -> Self {
        let event = CombinedEvent {
            input: CombinedInput::Timed(ev.input),
            modifiers: ev.modifiers.clone(),
            timestamp: ev.timestamp,
        };
        self.processor.process(event, C::emit);
        Self {
            processor: self.processor,
        }
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

        fn emit(ev: Event<Self::MappedEvent>) {
            dbg!(ev);
        }
    }

    let state = ProcessorState {
        processor: Processor,
        modifiers: Arc::new(Modifiers::default()),
        buttons: HashMap::new(),
    };
    let state = state.with_event(RawEvent::new(RawInput::KeyDown(KeyboardKey::Space), 1000));
    let state = state.with_event(RawEvent::new(RawInput::KeyUp(KeyboardKey::Space), 1150));
    let state = state.with_event(RawEvent::new(RawInput::KeyDown(KeyboardKey::Space), 1200));
    let state = state.with_event(RawEvent::new(RawInput::KeyUp(KeyboardKey::Space), 1300));

    let _ = state;

    panic!();
}
