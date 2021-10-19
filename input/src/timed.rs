use std::{collections::HashMap, sync::Arc};

use crate::{ButtonKind, EventWithModifiers, ModifiedEvent, ModifiedInput, Modifiers, TimestampMs};

pub type NumClicks = u32;

#[derive(Clone, Debug)]
pub struct TimedState<T: TimedContext> {
    buttons: HashMap<ButtonKind, ButtonTimedState<T::Timeout>>,
    context: T,
}

#[derive(Clone, Debug)]
pub struct ButtonTimedState<T> {
    input: ButtonTimedInput<T>,
    modifiers: Arc<Modifiers>,
    num_clicks: NumClicks,
}

#[derive(Clone, Debug)]
pub enum ButtonTimedInput<T> {
    Pressed { timeout: Arc<T> },
    LongPressed,
    Released { timeout: Arc<T> },
    LongReleased { timeout: Arc<T> },
}

#[derive(Clone, Debug)]
pub enum Duration {
    LongClick(LongClickDuration),
    MultiClick(MultiClickDuration),
}

#[derive(Clone, Debug)]
pub enum LongClickDuration {
    Key,
    Mouse,
    Touch,
}

#[derive(Clone, Debug)]
pub enum MultiClickDuration {
    Key,
    Mouse,
    Touch,
}

pub trait TimedContext: Sized {
    type Timeout;
    fn schedule(self, button: ButtonKind, delay: Duration) -> (Self, Arc<Self::Timeout>);
    fn emit_event(self, ev: TimedEvent) -> Self;
}

pub type TimedEvent = EventWithModifiers<TimedInput>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TimedInput {
    LongPress {
        button: ButtonKind,
        num_clicks: NumClicks,
    },
    Click {
        button: ButtonKind,
        num_clicks: NumClicks,
    },
    LongClick {
        button: ButtonKind,
        num_clicks: NumClicks,
    },
    ClickExact {
        button: ButtonKind,
        num_clicks: NumClicks,
    },
    LongClickExact {
        button: ButtonKind,
        num_clicks: NumClicks,
    },
}

impl<T: TimedContext> TimedState<T> {
    pub fn new(context: T) -> Self {
        Self {
            buttons: HashMap::default(),
            context,
        }
    }

    pub fn with_event(self, ev: ModifiedEvent) -> Self {
        use std::collections::hash_map::Entry;
        use ModifiedInput::*;

        let modifiers = ev.modifiers;
        let timestamp = ev.timestamp;
        let mut buttons = self.buttons;
        let context = self.context;
        let context = match ev.input {
            Press(button) => {
                let entry = buttons.entry(button.clone());
                match entry {
                    Entry::Occupied(entry) => {
                        let state = entry.remove();
                        let (state, context) = state.with_press_event(button.clone(), context);
                        buttons.insert(button.clone(), state);
                        context
                    }
                    Entry::Vacant(entry) => {
                        let (state, context) =
                            ButtonTimedState::from_pressed(button, &modifiers, context);
                        entry.insert(state);
                        context
                    }
                }
            }
            Release(button) => {
                let entry = buttons.entry(button.clone());
                match entry {
                    Entry::Occupied(entry) => {
                        let state = entry.remove();
                        let (state, context) =
                            state.with_release_event(button.clone(), timestamp, context);
                        buttons.insert(button, state);
                        context
                    }
                    Entry::Vacant(_) => context,
                }
            }
            Repeat(_) => context,
            Change(_) => context,
            Trigger(_) => context,
        };
        Self { buttons, context }
    }

    pub fn with_timeout_event(mut self, button: ButtonKind, timestamp: TimestampMs) -> Self {
        let state = self.buttons.remove(&button).unwrap();
        let mut buttons = self.buttons;
        let (state, context) = state.with_timeout_event(button.clone(), timestamp, self.context);
        if let Some(state) = state {
            buttons.insert(button, state);
        }
        Self { buttons, context }
    }
}

impl<U> ButtonTimedState<U> {
    fn from_pressed<T: TimedContext<Timeout = U>>(
        button: ButtonKind,
        modifiers: &Arc<Modifiers>,
        context: T,
    ) -> (Self, T) {
        let delay = Duration::LongClick(button.long_click_duration());
        let (context, timeout) = context.schedule(button, delay);
        let kind = ButtonTimedInput::Pressed { timeout };
        let modifiers = Arc::clone(modifiers);
        (
            ButtonTimedState {
                input: kind,
                modifiers,
                num_clicks: 0,
            },
            context,
        )
    }

    fn with_press_event<T: TimedContext<Timeout = U>>(
        self,
        button: ButtonKind,
        context: T,
    ) -> (Self, T) {
        use ButtonTimedInput::*;

        match self.input {
            Pressed { timeout: _ } => {
                panic!(); // TODO: warn
            }
            LongPressed {} => {
                panic!(); // TODO: warn
            }
            Released { timeout: _ } => {
                let delay = Duration::LongClick(button.long_click_duration());
                let (context, timeout) = context.schedule(button, delay);
                (
                    ButtonTimedState {
                        input: ButtonTimedInput::Pressed { timeout },
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks,
                    },
                    context,
                )
            }
            LongReleased { timeout: _ } => {
                let delay = Duration::LongClick(button.long_click_duration());
                let (context, timeout) = context.schedule(button, delay);
                (
                    ButtonTimedState {
                        input: ButtonTimedInput::Pressed { timeout },
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks,
                    },
                    context,
                )
            }
        }
    }

    fn with_release_event<T: TimedContext<Timeout = U>>(
        self,
        button: ButtonKind,
        timestamp: TimestampMs,
        context: T,
    ) -> (Self, T) {
        use ButtonTimedInput::*;

        match self.input {
            Pressed { timeout: _ } => {
                let context = context.emit_event(TimedEvent::new(
                    TimedInput::Click {
                        button: button.clone(),
                        num_clicks: self.num_clicks + 1,
                    },
                    Arc::clone(&self.modifiers),
                    timestamp,
                ));
                let delay = Duration::MultiClick(button.multi_click_duration());
                let (context, timeout) = context.schedule(button, delay);
                (
                    ButtonTimedState {
                        input: ButtonTimedInput::Released { timeout },
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks + 1,
                    },
                    context,
                )
            }
            LongPressed => {
                let context = context.emit_event(TimedEvent::new(
                    TimedInput::LongClick {
                        button: button.clone(),
                        num_clicks: self.num_clicks + 1,
                    },
                    Arc::clone(&self.modifiers),
                    timestamp,
                ));
                let delay = Duration::MultiClick(button.multi_click_duration());
                let (context, timeout) = context.schedule(button, delay);
                (
                    ButtonTimedState {
                        input: ButtonTimedInput::Released { timeout },
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks + 1,
                    },
                    context,
                )
            }
            Released { timeout: _ } => {
                panic!(); // TODO: warn
            }
            LongReleased { timeout: _ } => {
                panic!(); // TODO: warn
            }
        }
    }

    pub fn with_timeout_event<T: TimedContext<Timeout = U>>(
        self,
        button: ButtonKind,
        timestamp: TimestampMs,
        context: T,
    ) -> (Option<Self>, T) {
        use ButtonTimedInput::*;

        match self.input {
            Pressed { timeout: _ } => {
                let context = context.emit_event(EventWithModifiers::new(
                    TimedInput::LongPress {
                        button,
                        num_clicks: self.num_clicks,
                    },
                    Arc::clone(&self.modifiers),
                    timestamp,
                ));
                (
                    Some(ButtonTimedState {
                        input: ButtonTimedInput::LongPressed,
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks,
                    }),
                    context,
                )
            }
            LongPressed => {
                panic!("timeout event has been received but timeout is not stored in button state");
            }
            Released { timeout: _ } => {
                let context = context.emit_event(EventWithModifiers::new(
                    TimedInput::ClickExact {
                        button,
                        num_clicks: self.num_clicks,
                    },
                    Arc::clone(&self.modifiers),
                    timestamp,
                ));
                (None, context)
            }
            LongReleased { timeout: _ } => {
                let context = context.emit_event(EventWithModifiers::new(
                    TimedInput::LongClickExact {
                        button,
                        num_clicks: self.num_clicks,
                    },
                    Arc::clone(&self.modifiers),
                    timestamp,
                ));
                (None, context)
            }
        }
    }
}

impl TimedEvent {
    pub fn new(kind: TimedInput, modifiers: Arc<Modifiers>, timestamp: TimestampMs) -> Self {
        Self {
            input: kind,
            modifiers,
            timestamp,
        }
    }
}

#[test]
fn raw_input_to_input_test() {
    use std::{collections::BTreeMap, sync::Arc, sync::Weak};

    use crate::RawContext;
    use crate::{
        ButtonKind, CombinedEvent, CombinedInput, KeyboardKey, ModifiedEvent, ModifiedState,
        MouseButton, RawEvent, RawInput, TimestampMs,
    };

    #[derive(Clone, Debug)]
    struct ScheduledTimeout {
        button: ButtonKind,
    }

    #[derive(Clone, Debug)]
    struct RawSimpleContext {
        time: TimestampMs,
        state: TimedState<TimedSimpleContext>,
    }

    impl ModifiedState<RawSimpleContext> {
        fn with_context_event(mut self, ev: RawEvent) -> Self {
            // TODO: Remove
            println!("{:?}", ev);

            assert!(self.context().time < ev.timestamp);
            self.context_mut().time = ev.timestamp;
            self = self.with_event(ev);

            // TODO: Remove
            println!("{:?}", self);
            for event in &self.context().state.context.events {
                println!("{:?}", event);
            }
            println!();
            self.context_mut().state.context.events.clear();
            self
        }
    }

    impl RawContext for RawSimpleContext {
        fn emit_event(mut self, ev: ModifiedEvent) -> Self {
            self.state.context.events.push(CombinedEvent {
                input: CombinedInput::Modified(ev.input.clone()),
                modifiers: ev.modifiers.clone(),
                timestamp: ev.timestamp,
            });
            Self {
                time: self.time,
                state: self.state.with_context_event(ev),
            }
        }
    }

    #[derive(Clone, Debug)]
    struct TimedSimpleContext {
        time: TimestampMs,
        timeouts: BTreeMap<TimestampMs, Weak<ScheduledTimeout>>,
        events: Vec<CombinedEvent>,
    }

    impl TimedState<TimedSimpleContext> {
        fn with_context_event(mut self, ev: ModifiedEvent) -> Self {
            assert!(self.context.time < ev.timestamp);
            self.context.time = ev.timestamp;
            while let Some(entry) = self.context.timeouts.first_entry() {
                if *entry.key() > ev.timestamp {
                    break;
                }
                let (timestamp, timeout) = entry.remove_entry();
                if let Some(timeout) = timeout.upgrade() {
                    self = self.with_timeout_event(timeout.button.clone(), timestamp)
                }
            }
            self = self.with_event(ev);
            self
        }
    }

    impl TimedContext for TimedSimpleContext {
        type Timeout = ScheduledTimeout;

        fn schedule(
            mut self,
            button: ButtonKind,
            delay: Duration,
        ) -> (Self, Arc<ScheduledTimeout>) {
            let timeout = Arc::new(ScheduledTimeout { button });
            let delay = match delay {
                Duration::LongClick(_) => 1000,
                Duration::MultiClick(_) => 300,
            };
            self.timeouts
                .insert(self.time + delay, Arc::downgrade(&timeout));
            (self, timeout)
        }

        fn emit_event(mut self, ev: TimedEvent) -> Self {
            self.events.push(CombinedEvent {
                input: CombinedInput::Timed(ev.input),
                modifiers: ev.modifiers,
                timestamp: ev.timestamp,
            });
            self
        }
    }

    let timed_context = TimedSimpleContext {
        time: 0,
        timeouts: BTreeMap::new(),
        events: vec![],
    };
    let timed_state = TimedState::new(timed_context);
    let context = RawSimpleContext {
        time: 0,
        state: timed_state,
    };
    let state = ModifiedState::new(context);

    let state = state.with_context_event(RawEvent::new(
        RawInput::KeyDown(KeyboardKey::LeftCtrl),
        10000,
    ));
    let state =
        state.with_context_event(RawEvent::new(RawInput::KeyUp(KeyboardKey::LeftCtrl), 10500));
    let state = state.with_context_event(RawEvent::new(
        RawInput::KeyDown(KeyboardKey::LeftCtrl),
        11000,
    ));
    let state =
        state.with_context_event(RawEvent::new(RawInput::KeyUp(KeyboardKey::LeftCtrl), 13000));

    let state = state.with_context_event(RawEvent::new(
        RawInput::KeyDown(KeyboardKey::LeftCtrl),
        15000,
    ));
    let state = state.with_context_event(RawEvent::new(
        RawInput::MouseDown(MouseButton::Primary),
        15100,
    ));
    let state = state.with_context_event(RawEvent::new(
        RawInput::MouseUp(MouseButton::Primary),
        15200,
    ));
    let state = state.with_context_event(RawEvent::new(
        RawInput::MouseDown(MouseButton::Primary),
        15300,
    ));
    let state = state.with_context_event(RawEvent::new(
        RawInput::MouseUp(MouseButton::Primary),
        15400,
    ));

    let state =
        state.with_context_event(RawEvent::new(RawInput::KeyUp(KeyboardKey::LeftCtrl), 18000));

    let _ = state;

    // TODO: check states
}
