use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

use crate::{ButtonKind, EventWithModifiers, ModifiedEvent, ModifiedInput, Modifiers};

pub type NumClicks = u32;
pub type TimedEvent = EventWithModifiers<TimedInput>;

#[derive(Clone, Debug, Default)]
pub struct TimedState {
    buttons: TimedStateButtons,
}

#[derive(Clone, Debug)]
pub struct TimedStateButtons(HashMap<ButtonKind, ButtonTimedState>);

#[derive(Clone, Debug)]
struct ButtonTimedState {
    input: ButtonTimedInput,
    modifiers: Arc<Modifiers>,
    num_clicks: NumClicks,
}

#[derive(Clone, Debug)]
enum ButtonTimedInput {
    Pressed(Arc<ScheduledTransition>),
    LongPressed,
    Released(Arc<ScheduledTransition>),
    LongReleased(Arc<ScheduledTransition>),
}

#[derive(Clone, Debug)]
pub struct ScheduledTimeout {
    pub transition: Arc<ScheduledTransition>,
    pub duration: Duration,
}

#[derive(Clone, Debug)]
pub struct ScheduledTransition {
    button: ButtonKind,
}

impl ScheduledTimeout {
    fn new(transition: Arc<ScheduledTransition>, duration: Duration) -> Self {
        Self {
            transition,
            duration,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Duration {
    LongClick,
    MultiClick,
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

#[derive(Clone, Debug)]
pub struct TimedTransition {
    pub event: Option<TimedEvent>,
    pub scheduled: Option<ScheduledTimeout>,
    pub state: TimedState,
}

#[derive(Clone, Debug)]
pub struct TimedTransitionError<E> {
    pub state: TimedState,
    pub error: E,
}

impl TimedState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_event<T>(
        self,
        ev: ModifiedEvent<T>,
    ) -> Result<TimedTransition, TimedTransitionError<TimedInputWithEventError>> {
        use std::collections::hash_map::Entry;
        use ModifiedInput::{Change, Press, Release, Repeat, Trigger};

        let modifiers = ev.modifiers;
        let mut buttons = self.buttons;
        match ev.input {
            Press(button) => {
                let entry = buttons.0.entry(button.clone());
                match entry {
                    Entry::Occupied(entry) => {
                        let state = entry.remove();
                        let transition = state.with_press_event(button.clone());
                        Self::from_buttons_transition(buttons, button, transition)
                    }
                    Entry::Vacant(entry) => {
                        let transition = ButtonTimedState::from_pressed(button, &modifiers);
                        let _: &mut _ = entry.insert(transition.state);
                        Ok(TimedTransition {
                            event: transition.event,
                            scheduled: transition.scheduled,
                            state: Self { buttons },
                        })
                    }
                }
            }
            Release(button) => {
                let entry = buttons.0.entry(button.clone());
                match entry {
                    Entry::Occupied(entry) => {
                        let state = entry.remove();
                        let transition = state.with_release_event(button.clone());
                        Self::from_buttons_transition(buttons, button, transition)
                    }
                    Entry::Vacant(_) => Ok(TimedTransition {
                        event: None,
                        scheduled: None,
                        state: Self { buttons },
                    }),
                }
            }
            Repeat(_) | Change(_) | Trigger(_) => Ok(TimedTransition {
                event: None,
                scheduled: None,
                state: Self { buttons },
            }),
        }
    }

    fn from_buttons_transition<E1, E2: From<E1>>(
        mut buttons: TimedStateButtons,
        button: ButtonKind,
        transition: Result<TimedButtonTransition, TimedButtonTransitionError<E1>>,
    ) -> Result<TimedTransition, TimedTransitionError<E2>> {
        match transition {
            Ok(transition) => {
                let prev = buttons.0.insert(button, transition.state);
                assert!(prev.is_none());
                Ok(TimedTransition {
                    event: transition.event,
                    scheduled: transition.scheduled,
                    state: Self { buttons },
                })
            }
            Err(error) => {
                let prev = buttons.0.insert(button, error.state);
                assert!(prev.is_none());
                Err(TimedTransitionError {
                    state: Self { buttons },
                    error: error.error.into(),
                })
            }
        }
    }

    pub fn with_timeout_event(
        self,
        transition: Arc<ScheduledTransition>,
    ) -> Result<TimedTransition, TimedTransitionError<TimedInputWithTimeoutEventError>> {
        let button = transition.button.clone(); // TODO: no clone
        let mut buttons = self.buttons;
        match buttons.0.remove(&button) {
            Some(state) => {
                let transition = state.with_timeout_event(button.clone());

                match transition {
                    Ok(transition) => {
                        if let Some(state) = transition.state {
                            let prev = buttons.0.insert(button, state);
                            assert!(prev.is_none());
                        }
                        Ok(TimedTransition {
                            event: transition.event,
                            scheduled: transition.scheduled,
                            state: Self { buttons },
                        })
                    }
                    Err(error) => {
                        let prev = buttons.0.insert(button, error.state);
                        assert!(prev.is_none());
                        Err(TimedTransitionError {
                            state: Self { buttons },
                            error: error.error.into(),
                        })
                    }
                }
            }
            None => Err(TimedTransitionError {
                state: Self { buttons },
                error: TimedInputWithTimeoutEventError::DefaultButtonState { button },
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimedButtonTransition {
    state: ButtonTimedState,
    event: Option<TimedEvent>,
    scheduled: Option<ScheduledTimeout>,
}

#[derive(Clone, Debug)]
pub struct TimedButtonOptionTransition {
    state: Option<ButtonTimedState>,
    event: Option<TimedEvent>,
    scheduled: Option<ScheduledTimeout>,
}

/*
#[derive(Clone, Debug)]
pub struct TimedButtonTransitionOutput {
    event: Option<TimedEvent>,
    scheduled: Option<ScheduledTimeout>,
}
*/

#[derive(Clone, Debug)]
pub struct TimedButtonTransitionError<E> {
    state: ButtonTimedState,
    error: E,
}

impl ButtonTimedState {
    fn from_pressed(button: ButtonKind, modifiers: &Arc<Modifiers>) -> TimedButtonTransition {
        let transition = Arc::new(ScheduledTransition { button });
        let scheduled = ScheduledTimeout::new(Arc::clone(&transition), Duration::LongClick);
        TimedButtonTransition {
            state: Self {
                input: ButtonTimedInput::Pressed(transition),
                modifiers: Arc::clone(modifiers),
                num_clicks: 0,
            },
            event: None,
            scheduled: Some(scheduled),
        }
    }

    fn with_press_event(
        self,
        button: ButtonKind,
    ) -> Result<TimedButtonTransition, TimedButtonTransitionError<TimedInputWithPressEventError>>
    {
        use ButtonTimedInput::{LongPressed, LongReleased, Pressed, Released};

        match self.input {
            Pressed(_) => Err(TimedButtonTransitionError {
                state: self,
                error: TimedInputWithPressEventError::AlreadyPressed { button },
            }),
            LongPressed => Err(TimedButtonTransitionError {
                state: self,
                error: TimedInputWithPressEventError::AlreadyLongPressed { button },
            }),
            Released(_) | LongReleased(_) => {
                let transition = Arc::new(ScheduledTransition { button });
                let scheduled = ScheduledTimeout::new(Arc::clone(&transition), Duration::LongClick);
                Ok(TimedButtonTransition {
                    state: Self {
                        input: Pressed(transition),
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks,
                    },
                    event: None,
                    scheduled: Some(scheduled),
                })
            }
        }
    }

    fn with_release_event(
        self,
        button: ButtonKind,
    ) -> Result<TimedButtonTransition, TimedButtonTransitionError<TimedInputWithReleaseEventError>>
    {
        use ButtonTimedInput::{LongPressed, LongReleased, Pressed, Released};

        match self.input {
            Pressed(_) => {
                let transition = Arc::new(ScheduledTransition {
                    button: button.clone(),
                });
                let scheduled =
                    ScheduledTimeout::new(Arc::clone(&transition), Duration::MultiClick);
                Ok(TimedButtonTransition {
                    state: Self {
                        input: Released(transition),
                        modifiers: Arc::clone(&self.modifiers),
                        num_clicks: self.num_clicks + 1,
                    },
                    event: Some(TimedEvent::new(
                        TimedInput::Click {
                            button: button,
                            num_clicks: self.num_clicks + 1,
                        },
                        self.modifiers,
                    )),
                    scheduled: Some(scheduled),
                })
            }

            LongPressed => {
                let transition = Arc::new(ScheduledTransition { button });
                let scheduled =
                    ScheduledTimeout::new(Arc::clone(&transition), Duration::MultiClick);
                Ok(TimedButtonTransition {
                    state: Self {
                        input: LongReleased(transition),
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks + 1,
                    },
                    event: None,
                    scheduled: Some(scheduled),
                })
            }

            Released(_) => Err(TimedButtonTransitionError {
                state: self,
                error: TimedInputWithReleaseEventError::AlreadyReleased { button },
            }),

            LongReleased(_) => Err(TimedButtonTransitionError {
                state: self,
                error: TimedInputWithReleaseEventError::AlreadyLongPressed { button },
            }),
        }
    }

    fn with_timeout_event(
        self,
        button: ButtonKind,
    ) -> Result<
        TimedButtonOptionTransition,
        TimedButtonTransitionError<TimedInputWithTimeoutEventError>,
    > {
        use ButtonTimedInput::{LongPressed, LongReleased, Pressed, Released};
        let num_clicks = self.num_clicks;

        match self.input {
            Pressed(_) => {
                let transition = Arc::new(ScheduledTransition {
                    button: button.clone(),
                });
                let scheduled =
                    ScheduledTimeout::new(Arc::clone(&transition), Duration::MultiClick);
                Ok(TimedButtonOptionTransition {
                    state: Some(Self {
                        input: LongPressed,
                        modifiers: Arc::clone(&self.modifiers),
                        num_clicks,
                    }),
                    event: Some(TimedEvent::new(
                        TimedInput::LongPress { button, num_clicks },
                        self.modifiers,
                    )),
                    scheduled: Some(scheduled),
                })
            }

            LongPressed => Err(TimedButtonTransitionError {
                state: self,
                error: TimedInputWithTimeoutEventError::LongPressed { button },
            }),

            Released(_) => Ok(TimedButtonOptionTransition {
                state: None,
                event: Some(TimedEvent::new(
                    TimedInput::ClickExact { button, num_clicks },
                    Arc::clone(&self.modifiers),
                )),
                scheduled: None,
            }),

            LongReleased(_) => Ok(TimedButtonOptionTransition {
                state: None,
                event: Some(TimedEvent::new(
                    TimedInput::LongClickExact { button, num_clicks },
                    Arc::clone(&self.modifiers),
                )),
                scheduled: None,
            }),
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

impl Default for TimedStateButtons {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

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

#[test]
fn raw_input_to_input_test() {
    use std::{collections::BTreeMap, sync::Arc, sync::Weak};

    use crate::{CombinedEvent, CombinedInput, KeyboardKey, ModifiedState, MouseButton, RawInput};

    type TimestampMs = u64;

    #[derive(Clone, Debug)]
    struct State {
        modified_state: ModifiedState,
        timed_state: TimedState,
        timeouts: BTreeMap<TimestampMs, Weak<ScheduledTransition>>,
        last_timestamp: TimestampMs,
    }

    impl State {
        fn with_event(
            self,
            ev: RawInput<()>,
            timestamp: TimestampMs,
        ) -> (Self, Vec<CombinedEvent<()>>) {
            fn apply_timed_transition(
                mut events: Vec<EventWithModifiers<CombinedInput<()>>>,
                mut timeouts: BTreeMap<TimestampMs, Weak<ScheduledTransition>>,
                transition: TimedTransition,
                timestamp: TimestampMs,
            ) -> (
                Vec<EventWithModifiers<CombinedInput<()>>>,
                TimedState,
                BTreeMap<TimestampMs, Weak<ScheduledTransition>>,
            ) {
                events.extend(transition.event.into_iter().map(|ev| CombinedEvent {
                    input: CombinedInput::Timed(ev.input),
                    modifiers: ev.modifiers,
                }));
                if let Some(scheduled) = transition.scheduled {
                    let delay = match scheduled.duration {
                        Duration::LongClick => 1000,
                        Duration::MultiClick => 300,
                    };
                    let _ =
                        timeouts.insert(timestamp + delay, Arc::downgrade(&scheduled.transition));
                }
                (events, transition.state, timeouts)
            }

            println!();
            println!("{:?}", ev);

            let mut timed_state = self.timed_state;
            let mut timeouts = self.timeouts;
            assert!(self.last_timestamp < timestamp);
            let last_timestamp = timestamp;

            let mut events = Vec::new();

            while let Some(entry) = timeouts.first_entry() {
                if *entry.key() > timestamp {
                    break;
                }
                let (_, timeout) = entry.remove_entry();
                if let Some(timeout) = timeout.upgrade() {
                    let transition = timed_state.with_timeout_event(timeout).unwrap();
                    let result = apply_timed_transition(events, timeouts, transition, timestamp);
                    events = result.0;
                    timed_state = result.1;
                    timeouts = result.2;
                }
            }

            let transition = self.modified_state.with_event(ev.clone());
            let modified_state = transition.to_state();

            for ev in transition {
                events.push(CombinedEvent {
                    input: CombinedInput::Modified(ev.input.clone()),
                    modifiers: ev.modifiers.clone(),
                });

                let transition = timed_state.with_event(ev.clone()).unwrap();
                let result = apply_timed_transition(events, timeouts, transition, timestamp);
                events = result.0;
                timed_state = result.1;
                timeouts = result.2;
            }

            println!("{:?}", events);

            (
                Self {
                    modified_state,
                    timed_state,
                    timeouts,
                    last_timestamp,
                },
                events,
            )
        }
    }

    let state = State {
        modified_state: ModifiedState::new(),
        timed_state: TimedState::new(),
        timeouts: BTreeMap::new(),
        last_timestamp: 0,
    };

    let ctrl = || KeyboardKey("LeftCtrl".to_owned());

    let state = state.with_event(RawInput::KeyDown(ctrl()), 10000).0;
    let state = state.with_event(RawInput::KeyUp(ctrl()), 10500).0;
    let state = state.with_event(RawInput::KeyDown(ctrl()), 11000).0;
    let state = state.with_event(RawInput::KeyUp(ctrl()), 13000).0;

    let state = state.with_event(RawInput::KeyDown(ctrl()), 15000).0;
    let state = state
        .with_event(RawInput::MouseDown(MouseButton::Primary), 15100)
        .0;
    let state = state
        .with_event(RawInput::MouseUp(MouseButton::Primary), 15200)
        .0;
    let state = state
        .with_event(RawInput::MouseDown(MouseButton::Primary), 15300)
        .0;
    let state = state
        .with_event(RawInput::MouseUp(MouseButton::Primary), 15400)
        .0;

    let state = state.with_event(RawInput::KeyUp(ctrl()), 18000).0;

    let _ = state;
}
