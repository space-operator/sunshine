use core::hash::Hash;
use std::collections::HashMap;
use std::sync::Arc;

use thiserror::Error;

use crate::{Action, EventWithAction};

pub type NumClicks = u32;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimedEvent<Ki> {
    pub kind: Ki,
    pub num_clicks: NumClicks,
}

pub type ReleaseTimedEvent = TimedEvent<ReleaseTimedEventKind>;
pub type DelayedTimedEvent = TimedEvent<DelayedTimedEventKind>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReleaseTimedEventKind {
    Click,
    LongClick,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DelayedTimedEventKind {
    LongPress,
    ClickExact,
    LongClickExact,
}

#[derive(Clone, Debug)]
pub struct TimedState<Sw> {
    switches: HashMap<Sw, SwitchState>,
}

#[derive(Clone, Debug)]
struct SwitchState {
    kind: SwitchStateKind,
    num_clicks: NumClicks,
}

#[derive(Clone, Debug)]
enum SwitchStateKind {
    Pressed(Arc<ScheduledTag>),
    LongPressed,
    Released(Arc<ScheduledTag>),
    LongReleased(Arc<ScheduledTag>),
}

#[derive(Clone, Debug)]
pub struct ScheduledTimeout {
    pub tag: Arc<ScheduledTag>,
    pub duration: Duration,
}

#[derive(Clone, Debug)]
pub struct ScheduledTag;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Duration {
    LongClick,
    MultiClick,
}

#[derive(Clone, Debug)]
pub struct TimedTransition<Sw, Sh> {
    pub state: TimedState<Sw>,
    pub scheduled: Sh,
}

pub type PressTimedTransition<Sw> = TimedTransition<Sw, ScheduledTimeout>;
pub type ReleaseTimedTransition<Sw> = TimedTransition<Sw, ScheduledTimeout>;
pub type DelayedTimedTransition<Sw> = TimedTransition<Sw, ()>;

#[derive(Clone, Debug)]
struct SwitchTransition<Sh> {
    pub state: SwitchState,
    pub scheduled: Sh,
}

pub type FirstPressSwitchTransition = SwitchTransition<ScheduledTimeout>;
pub type PressSwitchTransition = SwitchTransition<ScheduledTimeout>;

#[derive(Clone, Debug)]
pub struct TimedPressError<Sw> {
    state: TimedState<Sw>,
    kind: TimedPressErrorKind,
}

#[derive(Clone, Debug)]
struct SwitchPressError {
    state: SwitchState,
    kind: SwitchPressErrorKind,
}

#[derive(Clone, Debug)]
pub enum TimedPressErrorKind {
    AlreadyPressed,
    AlreadyLongPressed,
}

#[derive(Clone, Debug)]
enum SwitchPressErrorKind {
    AlreadyPressed,
    AlreadyLongPressed,
}

impl From<SwitchPressErrorKind> for TimedPressErrorKind {
    fn from(kind: SwitchPressErrorKind) -> Self {
        match kind {
            SwitchPressErrorKind::AlreadyPressed => Self::AlreadyPressed,
            SwitchPressErrorKind::AlreadyLongPressed => Self::AlreadyLongPressed,
        }
    }
}

/*
impl<Ki> TimedEvent<Ki> {
    #[must_use]
    pub fn new(event: kind: Ki, num_clicks: NumClicks) -> Self {
        Self {
            kind,
            num_clicks,
        }
    }
}
*/

impl ScheduledTimeout {
    fn new(tag: Arc<ScheduledTag>, duration: Duration) -> Self {
        Self { tag, duration }
    }
}

impl<Sw> From<HashMap<Sw, SwitchState>> for TimedState<Sw> {
    fn from(switches: HashMap<Sw, SwitchState>) -> Self {
        Self { switches }
    }
}

impl<Sw> TimedState<Sw> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_press(self, switch: Sw) -> Result<PressTimedTransition<Sw>, TimedPressError<Sw>>
    where
        Sw: Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        let mut switches = self.switches;
        let entry = switches.entry(switch);
        match entry {
            Entry::Occupied(entry) => {
                let (switch, state) = entry.remove_entry();
                match state.with_press_event() {
                    Ok(transition) => {
                        switches.insert(switch, transition.state);
                        Ok(PressTimedTransition {
                            state: Self::from(switches),
                            scheduled: transition.scheduled,
                        })
                    }
                    Err(error) => {
                        switches.insert(switch, error.state);
                        Err(TimedPressError {
                            state: Self::from(switches),
                            kind: error.kind.into(),
                        })
                    }
                }
            }
            Entry::Vacant(entry) => {
                let transition = SwitchState::from_pressed();
                let _: &mut _ = entry.insert(transition.state);
                Ok(PressTimedTransition {
                    state: Self::from(switches),
                    scheduled: transition.scheduled,
                })
            }
        }
    }

    pub fn on_release<Ev>(self, switch: Sw) {
        todo!()
    }

    /*
    pub fn with_event<Ev>(
        self,
        event: Ev,
    ) -> Result<ImmediateTimedTransition<Sw>, ImmediateTimedError<Sw>>
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
                        //let transition = state.with_press_event(switch.clone());
                        //Self::from_switches_transition(switches, event, switch, transition)
                        todo!()
                    }
                    Entry::Vacant(entry) => {
                        let transition = SwitchState::from_pressed(switch);
                        let _: &mut _ = entry.insert(transition.state);
                        Ok(ImmediateTimedTransition {
                            event: transition.event,
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
                        //let transition = state.with_release_event(switch.clone());
                        //Self::from_switches_transition(switches, event, switch, transition)
                        todo!();
                    }
                    Entry::Vacant(_) => Ok(ImmediateTimedTransition {
                        event: TimedEvent {
                            event,
                            data: None,
                        }
                        timed_event: None,
                        scheduled: None,
                        state: Self { switches },
                    }),
                }
            }
            None => Ok(ImmediateTimedTransition {
                event,
                timed_event: None,
                scheduled: None,
                state: Self { switches },
            }),
        }
    }

    pub fn with_timeout_event<Ev>(
        self,
        event: Ev,
        transition: Arc<ScheduledTag>,
    ) -> Result<DelayedTimedTransition<Sw>, TimedTransitionError<Sw, DelayedTimedEventError<Sw>>>
    where
        Sw: Clone + Eq + Hash,
    {
        let switch = transition.switch.clone(); // TODO: no clone
        let mut switches = self.switches;
        match switches.remove(&switch) {
            Some(state) => {
                let transition = state.with_timeout_event(switch.clone());

                match transition {
                    Ok(transition) => {
                        if let Some(state) = transition.state {
                            let prev = switches.insert(switch, state);
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
    }*/

    /*
    fn from_switches_transition<E1, E2: From<E1>>(
        mut switches: HashMap<Sw, SwitchState>,
        event: Ev,
        switch: Sw,
        transition: Result<SwitchTransition<Sw>, TimedSwitchTransitionError<E1>>,
    ) -> Result<ImmediateTimedTransition<Sw>, TimedTransitionError<Sw, E2>>
    where
        Sw: Eq + Hash,
        Ev: EventWithAction<Switch = Sw>,
    {
        match transition {
            Ok(transition) => {
                let prev = switches.insert(switch, transition.state);
                assert!(prev.is_none());
                Ok(ImmediateTimedTransition {
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
    }*/
}

impl<Sw> Default for TimedState<Sw> {
    fn default() -> Self {
        Self {
            switches: HashMap::default(),
        }
    }
}

impl SwitchState {
    fn from_pressed() -> FirstPressSwitchTransition {
        let tag = Arc::new(ScheduledTag);
        let scheduled = ScheduledTimeout::new(Arc::clone(&tag), Duration::LongClick);
        let state = Self {
            kind: SwitchStateKind::Pressed(tag),
            num_clicks: 0,
        };
        FirstPressSwitchTransition { state, scheduled }
    }

    fn with_press_event(self) -> Result<PressSwitchTransition, SwitchPressError> {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => Err(SwitchPressError {
                state: self,
                kind: SwitchPressErrorKind::AlreadyPressed,
            }),
            LongPressed => Err(SwitchPressError {
                state: self,
                kind: SwitchPressErrorKind::AlreadyLongPressed,
            }),
            Released(_) | LongReleased(_) => {
                let tag = Arc::new(ScheduledTag);
                let scheduled = ScheduledTimeout::new(Arc::clone(&tag), Duration::LongClick);
                let state = Self {
                    kind: Pressed(tag),
                    num_clicks: self.num_clicks,
                };
                Ok(SwitchTransition { state, scheduled })
            }
        }
    }
}
/*
impl SwitchState {
    fn from_pressed(switch: Sw) -> SwitchTransition<Sw> {
        let transition = Arc::new(ScheduledTag);
        let scheduled = ScheduledTimeout::new(Arc::clone(&transition), Duration::LongClick);
        SwitchTransition {
            state: Self {
                kind: SwitchStateKind::Pressed(transition),
                num_clicks: 0,
            },
            event: None,
            scheduled: Some(scheduled),
        }
    }

    fn with_press_event(
        self,
        switch: Sw,
    ) -> Result<SwitchTransition<Sw>, TimedSwitchTransitionError<TimedEventEnableError<Sw>>>
    {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => Err(TimedSwitchTransitionError {
                state: self,
                error: TimedEventEnableError::AlreadyPressed { switch },
            }),
            LongPressed => Err(TimedSwitchTransitionError {
                state: self,
                error: TimedEventEnableError::AlreadyLongPressed { switch },
            }),
            Released(_) | LongReleased(_) => {
                let transition = Arc::new(ScheduledTag);
                let scheduled = ScheduledTimeout::new(Arc::clone(&transition), Duration::LongClick);
                Ok(SwitchTransition {
                    state: Self {
                        kind: Pressed(transition),
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
        switch: Sw,
    ) -> Result<SwitchTransition<Sw>, TimedSwitchTransitionError<TimedEventDisableError<Sw>>>
    where
        Sw: Clone,
    {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => {
                let transition = Arc::new(ScheduledTag {
                    switch: switch.clone(),
                });
                let scheduled =
                    ScheduledTimeout::new(Arc::clone(&transition), Duration::MultiClick);
                Ok(SwitchTransition {
                    state: Self {
                        kind: Released(transition),
                        num_clicks: self.num_clicks + 1,
                    },
                    event: Some(ReleaseTimedEvent::new(
                        switch,
                        ReleaseTimedEventKind::Click,
                        self.num_clicks + 1,
                    )),
                    scheduled: Some(scheduled),
                })
            }

            LongPressed => {
                let transition = Arc::new(ScheduledTag {
                    switch: switch.clone(),
                });
                let scheduled =
                    ScheduledTimeout::new(Arc::clone(&transition), Duration::MultiClick);
                Ok(SwitchTransition {
                    state: Self {
                        kind: LongReleased(transition),
                        num_clicks: self.num_clicks + 1,
                    },
                    event: Some(ReleaseTimedEvent::new(
                        switch,
                        ReleaseTimedEventKind::LongClick,
                        self.num_clicks + 1,
                    )),
                    scheduled: Some(scheduled),
                })
            }

            Released(_) => Err(TimedSwitchTransitionError {
                state: self,
                error: TimedEventDisableError::AlreadyReleased { switch },
            }),

            LongReleased(_) => Err(TimedSwitchTransitionError {
                state: self,
                error: TimedEventDisableError::AlreadyLongPressed { switch },
            }),
        }
    }

    fn with_timeout_event(
        self,
        switch: Sw,
    ) -> Result<
        DelayedTimedSwitchTransition<Sw>,
        TimedSwitchTransitionError<DelayedTimedEventError<Sw>>,
    >
    where
        Sw: Clone,
    {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};
        let num_clicks = self.num_clicks;

        match self.kind {
            Pressed(_) => {
                let transition = Arc::new(ScheduledTag {
                    switch: switch.clone(),
                });
                let scheduled =
                    ScheduledTimeout::new(Arc::clone(&transition), Duration::MultiClick);
                Ok(DelayedTimedSwitchTransition {
                    state: Some(Self {
                        kind: LongPressed,
                        num_clicks: num_clicks,
                    }),
                    event: Some(DelayedTimedEvent::new(
                        switch,
                        DelayedTimedEventKind::LongPress,
                        self.num_clicks,
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
                    self.num_clicks,
                )),
                scheduled: None,
            }),

            LongReleased(_) => Ok(DelayedTimedSwitchTransition {
                state: None,
                event: Some(DelayedTimedEvent::new(
                    switch,
                    DelayedTimedEventKind::LongClickExact,
                    self.num_clicks,
                )),
                scheduled: None,
            }),
        }
    }
}
*/
#[derive(Clone, Debug, Error)]
pub enum TimedEventError<Sw> {
    #[error(transparent)]
    Enable(#[from] TimedEventPressError<Sw>),
    #[error(transparent)]
    Disable(#[from] TimedEventDisableError<Sw>),
}

#[derive(Clone, Debug, Error)]
pub enum TimedEventPressError<Sw> {
    #[error("Switch TODO is pressed while in Pressed state")]
    AlreadyPressed { switch: Sw },

    #[error("Switch TODO is pressed while in LongPressed state")]
    AlreadyLongPressed { switch: Sw },
}

#[derive(Clone, Debug, Error)]
pub enum TimedEventDisableError<Sw> {
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

/*
#[derive(Clone, Debug)]
struct SwitchTransition<Sw> {
    state: SwitchState,
    event: Option<ReleaseTimedEvent<Ev>>,
    scheduled: Option<ScheduledTimeout>,
}

#[derive(Clone, Debug)]
struct DelayedTimedSwitchTransition<Sw> {
    state: Option<SwitchState>,
    event: Option<DelayedTimedEvent<Ev>>,
    scheduled: Option<ScheduledTimeout>,
}*/
