use core::hash::Hash;
use std::collections::HashMap;
use std::sync::Arc;

use thiserror::Error;

use crate::{Action, EventWithAction};

pub type NumSwitches = u32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimedEvent<Sw> {
    switch: Sw,
    kind: TimedEventKind,
    num_switches: NumSwitches,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimedEventKind {
    Click,
    LongClick,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelayedTimedEvent<Sw> {
    switch: Sw,
    kind: DelayedTimedEventKind,
    num_switches: NumSwitches,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DelayedTimedEventKind {
    LongPress,
    ClickExact,
    LongClickExact,
}

#[derive(Clone, Debug)]
pub struct TimedState<Sw> {
    switches: HashMap<Sw, TimedSwitchState<Sw>>,
}

#[derive(Clone, Debug)]
struct TimedSwitchState<Sw> {
    kind: SwitchTimedStateKind<Sw>,
    num_switches: NumSwitches,
}

#[derive(Clone, Debug)]
enum SwitchTimedStateKind<Sw> {
    Pressed(Arc<ScheduledTransition<Sw>>),
    LongPressed,
    Released(Arc<ScheduledTransition<Sw>>),
    LongReleased(Arc<ScheduledTransition<Sw>>),
}

#[derive(Clone, Debug)]
pub struct ScheduledTimeout<Sw> {
    pub transition: Arc<ScheduledTransition<Sw>>,
    pub duration: Duration,
}

#[derive(Clone, Debug)]
pub struct ScheduledTransition<Sw> {
    switch: Sw,
}

impl<Sw> ScheduledTimeout<Sw> {
    fn new(transition: Arc<ScheduledTransition<Sw>>, duration: Duration) -> Self {
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

#[derive(Clone, Debug)]
pub struct TimedTransition<Ev, Sw> {
    pub event: Ev,
    pub timed_event: Option<TimedEvent<Sw>>,
    pub scheduled: Option<ScheduledTimeout<Sw>>,
    pub state: TimedState<Sw>,
}

#[derive(Clone, Debug)]
pub struct DelayedTimedTransition<Sw> {
    pub timed_event: Option<DelayedTimedEvent<Sw>>,
    pub scheduled: Option<ScheduledTimeout<Sw>>,
    pub state: TimedState<Sw>,
}

#[derive(Clone, Debug)]
pub struct TimedTransitionError<Sw, E> {
    pub state: TimedState<Sw>,
    pub error: E,
}

impl<Sw> TimedEvent<Sw> {
    #[must_use]
    pub fn new(switch: Sw, kind: TimedEventKind, num_switches: NumSwitches) -> Self {
        Self {
            switch,
            kind,
            num_switches,
        }
    }
}

impl<Sw> DelayedTimedEvent<Sw> {
    #[must_use]
    pub fn new(switch: Sw, kind: DelayedTimedEventKind, num_switches: NumSwitches) -> Self {
        Self {
            switch,
            kind,
            num_switches,
        }
    }
}

impl<Sw> TimedState<Sw> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_event<Ev>(
        self,
        event: Ev,
    ) -> Result<TimedTransition<Ev, Sw>, TimedTransitionError<Sw, TimedInputError<Sw>>>
    where
        Sw: Clone + Eq + Hash,
        Ev: EventWithAction<Switch = Sw>,
    {
        use std::collections::hash_map::Entry;

        let mut switches = self.switches;
        match event.action() {
            Some(Action::Enable(switch)) => {
                let entry = switches.entry(switch.clone());
                match entry {
                    Entry::Occupied(entry) => {
                        let state = entry.remove();
                        let transition = state.with_press_event(switch.clone());
                        Self::from_switches_transition(switches, event, switch, transition)
                    }
                    Entry::Vacant(entry) => {
                        let transition = TimedSwitchState::from_pressed(switch);
                        let _: &mut _ = entry.insert(transition.state);
                        Ok(TimedTransition {
                            event,
                            timed_event: transition.event,
                            scheduled: transition.scheduled,
                            state: Self { switches },
                        })
                    }
                }
            }
            Some(Action::Disable(switch)) => {
                let entry = switches.entry(switch.clone());
                match entry {
                    Entry::Occupied(entry) => {
                        let state = entry.remove();
                        let transition = state.with_release_event(switch.clone());
                        Self::from_switches_transition(switches, event, switch, transition)
                    }
                    Entry::Vacant(_) => Ok(TimedTransition {
                        event,
                        timed_event: None,
                        scheduled: None,
                        state: Self { switches },
                    }),
                }
            }
            None => Ok(TimedTransition {
                event,
                timed_event: None,
                scheduled: None,
                state: Self { switches },
            }),
        }
    }

    fn from_switches_transition<Ev, E1, E2: From<E1>>(
        switches: HashMap<Sw, TimedSwitchState<Sw>>,
        event: Ev,
        switch: Sw,
        transition: Result<TimedSwitchTransition<Sw>, TimedSwitchTransitionError<Sw, E1>>,
    ) -> Result<TimedTransition<Ev, Sw>, TimedTransitionError<Sw, E2>>
    where
        Sw: Eq + Hash,
        Ev: EventWithAction<Switch = Sw>,
    {
        match transition {
            Ok(transition) => {
                let prev = switches.insert(switch, transition.state);
                assert!(prev.is_none());
                Ok(TimedTransition {
                    event,
                    timed_event: transition.event,
                    scheduled: transition.scheduled,
                    state: Self { switches },
                })
            }
            Err(error) => {
                let prev = switches.insert(switch, error.state);
                assert!(prev.is_none());
                Err(TimedTransitionError {
                    state: Self { switches },
                    error: error.error.into(),
                })
            }
        }
    }

    pub fn with_timeout_event(
        self,
        transition: Arc<ScheduledTransition<Sw>>,
    ) -> Result<DelayedTimedTransition<Sw>, TimedTransitionError<Sw, DelayedTimedEventError<Sw>>>
    where
        Sw: Clone + Eq + Hash,
    {
        let switch = transition.switch; //.clone(); // TODO: no clone
        let mut switches = self.switches;
        match switches.remove(&switch) {
            Some(state) => {
                let transition = state.with_timeout_event(switch.clone());

                match transition {
                    Ok(transition) => {
                        if let Some(switch) = transition.state {
                            let prev = switches.insert(switch, switch);
                            assert!(prev.is_none());
                        }
                        Ok(DelayedTimedTransition {
                            timed_event: transition.event,
                            scheduled: transition.scheduled,
                            state: Self { switches },
                        })
                    }
                    Err(error) => {
                        let prev = switches.insert(switch, error.state);
                        assert!(prev.is_none());
                        Err(TimedTransitionError {
                            state: Self { switches },
                            error: error.error.into(),
                        })
                    }
                }
            }
            None => Err(TimedTransitionError {
                state: Self { switches },
                error: DelayedTimedEventError::DefaultButtonState { switch },
            }),
        }
    }
}

impl<Sw> Default for TimedState<Sw> {
    fn default() -> Self {
        Self {
            switches: HashMap::default(),
        }
    }
}

/*
#[derive(Clone, Debug)]
pub struct TimedButtonTransitionOutput {
    event: Option<TimedEvent>,
    scheduled: Option<ScheduledTimeout>,
}
*/

impl<Sw> TimedSwitchState<Sw> {
    fn from_pressed(switch: Sw) -> TimedSwitchTransition<Sw> {
        let transition = Arc::new(ScheduledTransition { switch });
        let scheduled = ScheduledTimeout::new(Arc::clone(&transition), Duration::LongClick);
        TimedSwitchTransition {
            state: Self {
                kind: SwitchTimedStateKind::Pressed(transition),
                num_switches: 0,
            },
            event: None,
            scheduled: Some(scheduled),
        }
    }

    fn with_press_event(
        self,
        switch: Sw,
    ) -> Result<TimedSwitchTransition<Sw>, TimedSwitchTransitionError<Sw, TimedInputEnableError<Sw>>>
    {
        use SwitchTimedStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => Err(TimedSwitchTransitionError {
                state: self,
                error: TimedInputEnableError::AlreadyPressed { switch },
            }),
            LongPressed => Err(TimedSwitchTransitionError {
                state: self,
                error: TimedInputEnableError::AlreadyLongPressed { switch },
            }),
            Released(_) | LongReleased(_) => {
                let transition = Arc::new(ScheduledTransition { switch });
                let scheduled = ScheduledTimeout::new(Arc::clone(&transition), Duration::LongClick);
                Ok(TimedSwitchTransition {
                    state: Self {
                        kind: Pressed(transition),
                        num_switches: self.num_switches,
                    },
                    event: None,
                    scheduled: Some(scheduled),
                })
            }
        }
    }

    fn with_release_event(
        self,
        switch: Sw,
    ) -> Result<TimedSwitchTransition<Sw>, TimedSwitchTransitionError<Sw, TimedInputDisableError<Sw>>>
    where
        Sw: Clone,
    {
        use SwitchTimedStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => {
                let transition = Arc::new(ScheduledTransition {
                    switch: switch.clone(),
                });
                let scheduled =
                    ScheduledTimeout::new(Arc::clone(&transition), Duration::MultiClick);
                Ok(TimedSwitchTransition {
                    state: Self {
                        kind: Released(transition),
                        num_switches: self.num_switches + 1,
                    },
                    event: Some(TimedEvent::new(
                        switch,
                        TimedEventKind::Click,
                        self.num_switches + 1,
                    )),
                    scheduled: Some(scheduled),
                })
            }

            LongPressed => {
                let transition = Arc::new(ScheduledTransition { switch });
                let scheduled =
                    ScheduledTimeout::new(Arc::clone(&transition), Duration::MultiClick);
                Ok(TimedSwitchTransition {
                    state: Self {
                        kind: LongReleased(transition),
                        num_switches: self.num_switches + 1,
                    },
                    event: None,
                    scheduled: Some(scheduled),
                })
            }

            Released(_) => Err(TimedSwitchTransitionError {
                state: self,
                error: TimedInputDisableError::AlreadyReleased { switch },
            }),

            LongReleased(_) => Err(TimedSwitchTransitionError {
                state: self,
                error: TimedInputDisableError::AlreadyLongPressed { switch },
            }),
        }
    }

    fn with_timeout_event(
        self,
        switch: Sw,
    ) -> Result<
        DelayedTimedSwitchTransition<Sw>,
        TimedSwitchTransitionError<Sw, DelayedTimedEventError<Sw>>,
    >
    where
        Sw: Clone,
    {
        use SwitchTimedStateKind::{LongPressed, LongReleased, Pressed, Released};
        let num_clicks = self.num_switches;

        match self.kind {
            Pressed(_) => {
                let transition = Arc::new(ScheduledTransition {
                    switch: switch.clone(),
                });
                let scheduled =
                    ScheduledTimeout::new(Arc::clone(&transition), Duration::MultiClick);
                Ok(DelayedTimedSwitchTransition {
                    state: Some(Self {
                        kind: LongPressed,
                        num_switches: num_clicks,
                    }),
                    event: Some(DelayedTimedEvent::new(
                        switch,
                        DelayedTimedEventKind::LongPress,
                        self.num_switches,
                    )),
                    scheduled: Some(scheduled),
                })
            }

            LongPressed => Err(TimedSwitchTransitionError {
                state: self,
                error: DelayedTimedEventError::LongPressed { switch },
            }),

            Released(_) => Ok(DelayedTimedSwitchTransition {
                state: None,
                event: Some(DelayedTimedEvent::new(
                    switch,
                    DelayedTimedEventKind::ClickExact,
                    self.num_switches,
                )),
                scheduled: None,
            }),

            LongReleased(_) => Ok(DelayedTimedSwitchTransition {
                state: None,
                event: Some(DelayedTimedEvent::new(
                    switch,
                    DelayedTimedEventKind::LongClickExact,
                    self.num_switches,
                )),
                scheduled: None,
            }),
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum TimedInputError<Sw> {
    #[error(transparent)]
    Enable(#[from] TimedInputEnableError<Sw>),
    #[error(transparent)]
    Disable(#[from] TimedInputDisableError<Sw>),
}

#[derive(Clone, Debug, Error)]
pub enum TimedInputEnableError<Sw> {
    #[error("Switch TODO is pressed while in Pressed state")]
    AlreadyPressed { switch: Sw },

    #[error("Switch TODO is pressed while in LongPressed state")]
    AlreadyLongPressed { switch: Sw },
}

#[derive(Clone, Debug, Error)]
pub enum TimedInputDisableError<Sw> {
    #[error("Switch TODO is released while in Released state")]
    AlreadyReleased { switch: Sw },

    #[error("Switch TODO is released while in LongPressed state")]
    AlreadyLongPressed { switch: Sw },
}

#[derive(Clone, Debug, Error)]
pub enum DelayedTimedEventError<Sw> {
    #[error("Timeout handler for switch TODO called while in LongPressed state that do not schedule any timeouts")]
    LongPressed { switch: Sw },

    #[error("Timeout handler for switch TODO called for default switch state")]
    DefaultButtonState { switch: Sw },
}

#[derive(Clone, Debug)]
struct TimedSwitchTransition<Sw> {
    state: TimedSwitchState<Sw>,
    event: Option<TimedEvent<Sw>>,
    scheduled: Option<ScheduledTimeout<Sw>>,
}

#[derive(Clone, Debug)]
struct DelayedTimedSwitchTransition<Sw> {
    state: Option<TimedSwitchState<Sw>>,
    event: Option<DelayedTimedEvent<Sw>>,
    scheduled: Option<ScheduledTimeout<Sw>>,
}

#[derive(Clone, Debug)]
struct TimedSwitchTransitionError<Sw, E> {
    state: TimedSwitchState<Sw>,
    error: E,
}

/*
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
*/
