use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

use crate::{ButtonKind, EventWithModifiers, ModifiedEvent, ModifiedInput, Modifiers};

pub type NumClicks = u32;

#[derive(Clone, Debug)]
pub struct TimedState<T: TimedContext> {
    buttons: TimedStateButtons<T::Timeout>,
    context: T,
}

#[derive(Clone, Debug)]
pub struct TimedStateButtons<T>(HashMap<ButtonKind, ButtonTimedState<T>>);

#[derive(Clone, Debug)]
struct ButtonTimedState<T> {
    input: ButtonTimedInput<T>,
    modifiers: Arc<Modifiers>,
    num_clicks: NumClicks,
}

struct ButtonTimedStateWithContext<T: TimedContext> {
    context: T,
    input: ButtonTimedInput<T::Timeout>,
    modifiers: Arc<Modifiers>,
    num_clicks: NumClicks,
}

enum TimedEventResult<T: TimedContext> {
    StateWithContext(ButtonTimedStateWithContext<T>),
    Context(T),
}

#[derive(Clone, Debug)]
pub enum ButtonTimedInput<T> {
    Pressed { timeout: Arc<T> },
    LongPressed,
    Released { timeout: Arc<T> },
    LongReleased { timeout: Arc<T> },
}

#[derive(Clone, Copy, Debug)]
pub enum Duration {
    LongClick(LongClickDuration),
    MultiClick(MultiClickDuration),
}

#[derive(Clone, Copy, Debug)]
pub enum LongClickDuration {
    Key,
    Mouse,
    Touch,
}

#[derive(Clone, Copy, Debug)]
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
            buttons: TimedStateButtons::default(),
            context,
        }
    }

    pub fn from_parts(buttons: TimedStateButtons<T::Timeout>, context: T) -> Self {
        Self { buttons, context }
    }

    pub fn context(&self) -> &T {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut T {
        &mut self.context
    }

    pub fn split(self) -> (TimedStateButtons<T::Timeout>, T) {
        (self.buttons, self.context)
    }

    #[allow(clippy::missing_panics_doc)] // asserts should never be called
    pub fn with_event<U>(
        self,
        ev: ModifiedEvent<U>,
    ) -> (Self, Result<(), TimedInputWithEventError>) {
        use std::collections::hash_map::Entry;
        use ModifiedInput::{Change, Press, Release, Repeat, Trigger};

        let modifiers = ev.modifiers;
        let mut buttons = self.buttons;
        let context = self.context;
        let (context, err) = match ev.input {
            Press(button) => {
                let entry = buttons.0.entry(button.clone());
                match entry {
                    Entry::Occupied(entry) => {
                        let state = entry.remove();
                        let (result, err) =
                            state.with_context(context).with_press_event(button.clone());
                        let (state, context) = result.split();
                        let prev = buttons.0.insert(button, state);
                        assert!(prev.is_none());
                        (context, err.map_err(Into::into))
                    }
                    Entry::Vacant(entry) => {
                        let (state, context) =
                            ButtonTimedStateWithContext::from_pressed(button, &modifiers, context)
                                .split();
                        let _: &mut _ = entry.insert(state);
                        (context, Ok(()))
                    }
                }
            }
            Release(button) => {
                let entry = buttons.0.entry(button.clone());
                match entry {
                    Entry::Occupied(entry) => {
                        let state = entry.remove();
                        let (result, err) = state
                            .with_context(context)
                            .with_release_event(button.clone());
                        let (state, context) = result.split();
                        let prev = buttons.0.insert(button, state);
                        assert!(prev.is_none());
                        (context, err.map_err(Into::into))
                    }
                    Entry::Vacant(_) => (context, Ok(())),
                }
            }
            Repeat(_) | Change(_) | Trigger(_) => (context, Ok(())),
        };
        (Self { buttons, context }, err)
    }

    #[allow(clippy::missing_panics_doc)] // asserts should never be called
    pub fn with_timeout_event(
        mut self,
        button: ButtonKind,
    ) -> (Self, Result<(), TimedInputWithTimeoutEventError>) {
        match self.buttons.0.remove(&button) {
            Some(state) => {
                let mut buttons = self.buttons;
                let (result, err) = state
                    .with_context(self.context)
                    .with_timeout_event(button.clone());
                let context = match result {
                    TimedEventResult::StateWithContext(result) => {
                        let (state, context) = result.split();
                        let prev = buttons.0.insert(button, state);
                        assert!(prev.is_none());
                        context
                    }
                    TimedEventResult::Context(context) => context,
                };
                (Self { buttons, context }, err)
            }
            None => (
                self,
                Err(TimedInputWithTimeoutEventError::DefaultButtonState { button }),
            ),
        }
    }
}

impl<U> ButtonTimedState<U> {
    fn with_context<T: TimedContext<Timeout = U>>(
        self,
        context: T,
    ) -> ButtonTimedStateWithContext<T> {
        ButtonTimedStateWithContext {
            context,
            input: self.input,
            modifiers: self.modifiers,
            num_clicks: self.num_clicks,
        }
    }
}

impl<T: TimedContext> ButtonTimedStateWithContext<T> {
    fn split(self) -> (ButtonTimedState<T::Timeout>, T) {
        (
            ButtonTimedState {
                input: self.input,
                modifiers: self.modifiers,
                num_clicks: self.num_clicks,
            },
            self.context,
        )
    }

    fn from_pressed(button: ButtonKind, modifiers: &Arc<Modifiers>, context: T) -> Self {
        let delay = Duration::LongClick(button.long_click_duration());
        let (context, timeout) = context.schedule(button, delay);
        let kind = ButtonTimedInput::Pressed { timeout };
        let modifiers = Arc::clone(modifiers);
        Self {
            context,
            input: kind,
            modifiers,
            num_clicks: 0,
        }
    }

    fn with_press_event(
        self,
        button: ButtonKind,
    ) -> (Self, Result<(), TimedInputWithPressEventError>) {
        use ButtonTimedInput::{LongPressed, LongReleased, Pressed, Released};

        match self.input {
            Pressed { timeout: _ } => (
                self,
                Err(TimedInputWithPressEventError::AlreadyPressed { button }),
            ),
            LongPressed {} => (
                self,
                Err(TimedInputWithPressEventError::AlreadyLongPressed { button }),
            ),
            Released { timeout: _ } | LongReleased { timeout: _ } => {
                let delay = Duration::LongClick(button.long_click_duration());
                let (context, timeout) = self.context.schedule(button, delay);
                (
                    Self {
                        context,
                        input: Pressed { timeout },
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks,
                    },
                    Ok(()),
                )
            }
        }
    }

    fn with_release_event(
        self,
        button: ButtonKind,
    ) -> (Self, Result<(), TimedInputWithReleaseEventError>) {
        use ButtonTimedInput::{LongPressed, LongReleased, Pressed, Released};

        match self.input {
            Pressed { timeout: _ } => {
                let context = self.context.emit_event(TimedEvent::new(
                    TimedInput::Click {
                        button: button.clone(),
                        num_clicks: self.num_clicks + 1,
                    },
                    Arc::clone(&self.modifiers),
                ));
                let delay = Duration::MultiClick(button.multi_click_duration());
                let (context, timeout) = context.schedule(button, delay);
                (
                    Self {
                        context,
                        input: Released { timeout },
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks + 1,
                    },
                    Ok(()),
                )
            }
            LongPressed => {
                let context = self.context.emit_event(TimedEvent::new(
                    TimedInput::LongClick {
                        button: button.clone(),
                        num_clicks: self.num_clicks + 1,
                    },
                    Arc::clone(&self.modifiers),
                ));
                let delay = Duration::MultiClick(button.multi_click_duration());
                let (context, timeout) = context.schedule(button, delay);
                (
                    Self {
                        context,
                        input: Released { timeout },
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks + 1,
                    },
                    Ok(()),
                )
            }
            Released { timeout: _ } => (
                self,
                Err(TimedInputWithReleaseEventError::AlreadyReleased { button }),
            ),

            LongReleased { timeout: _ } => (
                self,
                Err(TimedInputWithReleaseEventError::AlreadyLongPressed { button }),
            ),
        }
    }

    fn with_timeout_event(
        self,
        button: ButtonKind,
    ) -> (
        TimedEventResult<T>,
        Result<(), TimedInputWithTimeoutEventError>,
    ) {
        use ButtonTimedInput::{LongPressed, LongReleased, Pressed, Released};

        match self.input {
            Pressed { timeout: _ } => {
                let context = self.context.emit_event(EventWithModifiers::new(
                    TimedInput::LongPress {
                        button,
                        num_clicks: self.num_clicks,
                    },
                    Arc::clone(&self.modifiers),
                ));
                (
                    TimedEventResult::StateWithContext(Self {
                        context,
                        input: LongPressed,
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks,
                    }),
                    Ok(()),
                )
            }
            LongPressed => (
                (TimedEventResult::StateWithContext(self)),
                Err(TimedInputWithTimeoutEventError::LongPressed { button }),
            ),
            Released { timeout: _ } => {
                let context = self.context.emit_event(EventWithModifiers::new(
                    TimedInput::ClickExact {
                        button,
                        num_clicks: self.num_clicks,
                    },
                    Arc::clone(&self.modifiers),
                ));
                (TimedEventResult::Context(context), Ok(()))
            }
            LongReleased { timeout: _ } => {
                let context = self.context.emit_event(EventWithModifiers::new(
                    TimedInput::LongClickExact {
                        button,
                        num_clicks: self.num_clicks,
                    },
                    Arc::clone(&self.modifiers),
                ));
                (TimedEventResult::Context(context), Ok(()))
            }
        }
    }
}

impl TimedEvent {
    #[must_use]
    pub fn new(kind: TimedInput, modifiers: Arc<Modifiers>) -> Self {
        Self {
            input: kind,
            modifiers,
        }
    }
}

impl<T> Default for TimedStateButtons<T> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

#[test]
fn raw_input_to_input_test() {
    use std::{collections::BTreeMap, sync::Arc, sync::Weak};

    use crate::ModifiedContext;
    use crate::{
        ButtonKind, CombinedEvent, CombinedInput, KeyboardKey, ModifiedEvent, ModifiedState,
        MouseButton, RawInput,
    };

    type TimestampMs = u64;

    #[derive(Clone, Debug)]
    struct ScheduledTimeout {
        button: ButtonKind,
    }

    #[derive(Clone, Debug)]
    struct RawSimpleContext {
        state: TimedState<TimedSimpleContext>,
    }

    impl ModifiedState<RawSimpleContext> {
        fn with_context_event(mut self, ev: RawInput<()>, timestamp: TimestampMs) -> Self {
            println!("{:?}", ev);

            assert!(self.context().state.context.time < timestamp);
            self.context_mut().state.context.time = timestamp;

            let (modifiers, context) = self.split();
            self = Self::from_parts(modifiers, context);
            self = self.with_event(ev);

            println!("{:?}", self);
            for event in &self.context().state.context.events {
                println!("{:?}", event);
            }
            println!();
            let (modifiers, mut context) = self.split();
            context.state.context.events.clear();
            Self::from_parts(modifiers, context)
        }
    }

    impl ModifiedContext for RawSimpleContext {
        type CustomEvent = ();

        fn emit_event(mut self, ev: ModifiedEvent<Self::CustomEvent>) -> Self {
            self.state.context.events.push(CombinedEvent {
                input: CombinedInput::Modified(ev.input.clone()),
                modifiers: ev.modifiers.clone(),
            });
            Self {
                state: self.state.with_context_event(ev),
            }
        }
    }

    #[derive(Clone, Debug)]
    struct TimedSimpleContext {
        time: TimestampMs,
        timeouts: BTreeMap<TimestampMs, Weak<ScheduledTimeout>>,
        events: Vec<CombinedEvent<()>>,
    }

    impl TimedState<TimedSimpleContext> {
        fn with_context_event(mut self, ev: ModifiedEvent<()>) -> Self {
            //assert!(self.context.time < ev.timestamp);
            //self.context.time = ev.timestamp;
            while let Some(entry) = self.context.timeouts.first_entry() {
                if *entry.key() > self.context.time {
                    break;
                }
                let (_timestamp, timeout) = entry.remove_entry(); // TODO: timestamp?
                if let Some(timeout) = timeout.upgrade() {
                    let (state, err) = self.with_timeout_event(timeout.button.clone());
                    err.unwrap();
                    self = state;
                }
            }
            let (state, err) = self.with_event(ev);
            err.unwrap();
            state
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
            dbg!(&self.timeouts, self.time + delay);
            let prev = self
                .timeouts
                .insert(self.time + delay, Arc::downgrade(&timeout));
            assert!(prev.is_none());
            (self, timeout)
        }

        fn emit_event(mut self, ev: TimedEvent) -> Self {
            self.events.push(CombinedEvent {
                input: CombinedInput::Timed(ev.input),
                modifiers: ev.modifiers,
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
    let context = RawSimpleContext { state: timed_state };
    let state = ModifiedState::new(context);

    let ctrl = || KeyboardKey("LeftCtrl".to_owned());

    let state = state.with_context_event(RawInput::KeyDown(ctrl()), 10000);
    let state = state.with_context_event(RawInput::KeyUp(ctrl()), 10500);
    let state = state.with_context_event(RawInput::KeyDown(ctrl()), 11000);
    let state = state.with_context_event(RawInput::KeyUp(ctrl()), 13000);

    let state = state.with_context_event(RawInput::KeyDown(ctrl()), 15000);
    let state = state.with_context_event(RawInput::MouseDown(MouseButton::Primary), 15100);
    let state = state.with_context_event(RawInput::MouseUp(MouseButton::Primary), 15200);
    let state = state.with_context_event(RawInput::MouseDown(MouseButton::Primary), 15300);
    let state = state.with_context_event(RawInput::MouseUp(MouseButton::Primary), 15400);

    let state = state.with_context_event(RawInput::KeyUp(ctrl()), 18000);

    let _ = state;
}

/*
pub type TimedInputWithEventResultWithData<T> = Result<T, TimedInputWithEventErrorWithData<T>>;

#[derive(Clone, Debug, Error)]
#[error("{err}")]
pub struct TimedInputWithEventErrorWithData<T> {
    data: T,
    err: TimedInputWithEventError,
}
*/

#[derive(Clone, Debug, Error)]
pub enum TimedInputWithEventError {
    #[error(transparent)]
    Pressed(#[from] TimedInputWithPressEventError),
    #[error(transparent)]
    Released(#[from] TimedInputWithReleaseEventError),
}

#[derive(Clone, Debug, Error)]
pub enum TimedInputWithPressEventError {
    #[error("Button {button:?} is pressed while in Pressed state")]
    AlreadyPressed { button: ButtonKind },

    #[error("Button {button:?} is pressed while in LongPressed state")]
    AlreadyLongPressed { button: ButtonKind },
}

#[derive(Clone, Debug, Error)]
pub enum TimedInputWithReleaseEventError {
    #[error("Button {button:?} is released while in Released state")]
    AlreadyReleased { button: ButtonKind },

    #[error("Button {button:?} is released while in LongPressed state")]
    AlreadyLongPressed { button: ButtonKind },
}

#[derive(Clone, Debug, Error)]
pub enum TimedInputWithTimeoutEventError {
    #[error("Timeout handler for button {button:?} called while in LongPressed state that do not schedule any timeouts")]
    LongPressed { button: ButtonKind },

    #[error("Timeout handler for button {button:?} called for default button state")]
    DefaultButtonState { button: ButtonKind },
}

/*
impl TimedInputWithEventError {
    fn with_data<T>(self, data: T) -> TimedInputWithEventErrorWithData<T> {
        TimedInputWithEventErrorWithData { data, err: self }
    }
}

impl<T> TimedInputWithEventErrorWithData<T> {
    fn map_data<U, F>(self, f: F) -> TimedInputWithEventErrorWithData<U>
    where
        F: FnMut(T) -> U,
    {
        TimedInputWithEventErrorWithData {
            data: f(self.data),
            err: self.err,
        }
    }
}

*/
