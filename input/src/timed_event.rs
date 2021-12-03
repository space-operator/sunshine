use core::hash::Hash;
use std::collections::HashMap;
use std::sync::{Arc, Weak};

use thiserror::Error;

pub type NumClicks = u32;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimedEvent<Ki> {
    pub kind: Ki,
    pub num_clicks: NumClicks,
}

pub type TimedReleaseEvent = TimedEvent<TimedReleaseEventKind>;
pub type TimedLongPressEvent = TimedEvent<TimedLongPressEventKind>;
pub type TimedMultiClickEvent = TimedEvent<TimedMultiClickEventKind>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimedReleaseEventKind {
    Click,
    LongClick,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimedLongPressEventKind {
    LongPress,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimedMultiClickEventKind {
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
    Pressed(Arc<()>),
    LongPressed,
    Released(Arc<()>),
    LongReleased(Arc<()>),
}

#[derive(Clone, Debug)]
pub struct LongPressHandleRequest(Weak<()>);

#[derive(Clone, Debug)]
pub struct MultiClickHandleRequest(Weak<()>);

impl<Ki> TimedEvent<Ki> {
    #[must_use]
    pub fn new(kind: Ki, num_clicks: NumClicks) -> Self {
        Self { kind, num_clicks }
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

    pub fn with_press_event(
        self,
        switch: Sw,
    ) -> (Self, Result<LongPressHandleRequest, TimedPressError>)
    where
        Sw: Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        let mut switches = self.switches;
        let entry = switches.entry(switch);
        match entry {
            Entry::Occupied(entry) => {
                let (switch, state) = entry.remove_entry();
                let (state, request) = state.with_press_event();
                switches.insert(switch, state);
                (Self::from(switches), request)
            }
            Entry::Vacant(entry) => {
                let (state, request) = SwitchState::from_pressed();
                let _: &mut _ = entry.insert(state);
                (Self::from(switches), Ok(request))
            }
        }
    }

    pub fn with_release_event(
        self,
        switch: Sw,
    ) -> (
        Self,
        Result<Option<(TimedReleaseEvent, MultiClickHandleRequest)>, TimedReleaseError>,
    )
    where
        Sw: Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        let mut switches = self.switches;
        let entry = switches.entry(switch);
        match entry {
            Entry::Occupied(entry) => {
                let (switch, state) = entry.remove_entry();
                let (state, result) = state.with_release_event();
                switches.insert(switch, state);
                (Self::from(switches), result.map(Some))
            }
            Entry::Vacant(_) => (Self::from(switches), Ok(None)),
        }
    }

    pub fn with_timeout_event<T, E>(
        self,
        switch: Sw,
        callback: impl FnOnce(SwitchState) -> (Option<SwitchState>, Result<T, E>),
        err: E,
    ) -> (Self, Option<Result<T, E>>)
    where
        Sw: Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        let mut switches = self.switches;
        let entry = switches.entry(switch);
        match entry {
            Entry::Occupied(entry) => {
                let (switch, state) = entry.remove_entry();
                let (state, result) = callback(state);
                if let Some(state) = state {
                    switches.insert(switch, state);
                }
                (Self::from(switches), Some(result))
            }
            Entry::Vacant(_) => (Self::from(switches), Some(Err(err))),
        }
    }

    pub fn with_long_press_event(
        self,
        switch: Sw,
        request: LongPressHandleRequest,
    ) -> (
        Self,
        Option<Result<TimedLongPressEvent, TimedLongClickError>>,
    )
    where
        Sw: Eq + Hash,
    {
        if let Some(_) = request.0.upgrade() {
            self.with_timeout_event(
                switch,
                SwitchState::with_long_press_event,
                TimedLongClickError::Default,
            )
        } else {
            (self, None)
        }
    }

    pub fn with_multi_click_event(
        self,
        switch: Sw,
        request: MultiClickHandleRequest,
    ) -> (
        Self,
        Option<Result<TimedMultiClickEvent, TimedMultiClickError>>,
    )
    where
        Sw: Eq + Hash,
    {
        if let Some(_) = request.0.upgrade() {
            self.with_timeout_event(
                switch,
                SwitchState::with_multi_click_event,
                TimedMultiClickError::Default,
            )
        } else {
            (self, None)
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

impl SwitchState {
    fn new(kind: SwitchStateKind, num_clicks: NumClicks) -> Self {
        Self { kind, num_clicks }
    }

    fn from_pressed() -> (Self, LongPressHandleRequest) {
        let tag = Arc::new(());
        let request = LongPressHandleRequest(Arc::downgrade(&tag));
        let state = Self::new(SwitchStateKind::Pressed(tag), 0);
        (state, request)
    }

    fn with_press_event(self) -> (Self, Result<LongPressHandleRequest, TimedPressError>) {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => (self, Err(TimedPressError::AlreadyPressed)),
            LongPressed => (self, Err(TimedPressError::AlreadyLongPressed)),
            Released(_) | LongReleased(_) => {
                let tag = Arc::new(());
                let request = LongPressHandleRequest(Arc::downgrade(&tag));
                let state = Self::new(Pressed(tag), self.num_clicks);
                (state, Ok(request))
            }
        }
    }

    fn with_release_event(
        self,
    ) -> (
        Self,
        Result<(TimedReleaseEvent, MultiClickHandleRequest), TimedReleaseError>,
    ) {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => {
                let tag = Arc::new(());
                let request = MultiClickHandleRequest(Arc::downgrade(&tag));
                let state = Self::new(Released(tag), self.num_clicks + 1);
                let event = TimedEvent::new(TimedReleaseEventKind::Click, self.num_clicks + 1);
                (state, Ok((event, request)))
            }

            LongPressed => {
                let tag = Arc::new(());
                let request = MultiClickHandleRequest(Arc::downgrade(&tag));
                let state = Self::new(LongReleased(tag), self.num_clicks + 1);
                let event = TimedEvent::new(TimedReleaseEventKind::LongClick, self.num_clicks + 1);
                (state, Ok((event, request)))
            }

            Released(_) => (self, Err(TimedReleaseError::AlreadyReleased)),
            LongReleased(_) => (self, Err(TimedReleaseError::AlreadyLongReleased)),
        }
    }

    fn with_long_press_event(
        self,
    ) -> (
        Option<Self>,
        Result<TimedLongPressEvent, TimedLongClickError>,
    ) {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => {
                let tag = Arc::new(());
                let state = Self::new(LongPressed, self.num_clicks);
                let event =
                    TimedLongPressEvent::new(TimedLongPressEventKind::LongPress, self.num_clicks);
                (Some(state), Ok(event))
            }
            LongPressed => (Some(self), Err(TimedLongClickError::LongPressed)),
            Released(_) => (Some(self), Err(TimedLongClickError::Released)),
            LongReleased(_) => (Some(self), Err(TimedLongClickError::LongReleased)),
        }
    }

    fn with_multi_click_event(
        self,
    ) -> (
        Option<Self>,
        Result<TimedMultiClickEvent, TimedMultiClickError>,
    ) {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => (Some(self), Err(TimedMultiClickError::Pressed)),
            LongPressed => (Some(self), Err(TimedMultiClickError::LongPressed)),
            Released(_) => {
                let event = TimedMultiClickEvent::new(
                    TimedMultiClickEventKind::ClickExact,
                    self.num_clicks,
                );
                (None, Ok(event))
            }
            LongReleased(_) => {
                let event = TimedMultiClickEvent::new(
                    TimedMultiClickEventKind::LongClickExact,
                    self.num_clicks,
                );
                (None, Ok(event))
            }
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum TimedPressError {
    #[error("Button is pressed while in Pressed state")]
    AlreadyPressed,
    #[error("Button is pressed while in LongPressed state")]
    AlreadyLongPressed,
}

#[derive(Clone, Debug, Error)]
pub enum TimedReleaseError {
    #[error("Button is released while in Released state")]
    AlreadyReleased,
    #[error("Button is released while in LongPressed state")]
    AlreadyLongReleased,
}

#[derive(Clone, Debug, Error)]
pub enum TimedLongClickError {
    #[error("No handler calls requested for Default state")]
    Default,
    #[error("No handler calls requested for LongPressed state")]
    LongPressed,
    #[error("No handler calls requested for Released state")]
    Released,
    #[error("No handler calls requested for LongReleased state")]
    LongReleased,
}

#[derive(Clone, Debug, Error)]
pub enum TimedMultiClickError {
    #[error("No handler calls requested for Default state")]
    Default,
    #[error("No handler calls requested for Pressed state")]
    Pressed,
    #[error("No handler calls requested for LongPressed state")]
    LongPressed,
}
