use core::hash::Hash;
use std::collections::hash_map::OccupiedEntry;
use std::collections::HashMap;
use std::sync::{Arc, Weak};

use thiserror::Error;

pub type NumPossibleClicks = u32;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimedEventData<Ki> {
    pub kind: Ki,
    pub num_possible_clicks: NumPossibleClicks,
}

pub type TimedReleaseEventData = TimedEventData<TimedReleaseEventKind>;
pub type TimedLongPressEventData = TimedEventData<TimedLongPressEventKind>;
pub type TimedClickExactEventData = TimedEventData<TimedClickExactEventKind>;

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
pub enum TimedClickExactEventKind {
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
    num_possible_clicks: NumPossibleClicks,
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
pub struct ClickExactHandleRequest(Weak<()>);

impl<Ki> TimedEventData<Ki> {
    #[must_use]
    pub fn new(kind: Ki, num_clicks: NumPossibleClicks) -> Self {
        Self {
            kind,
            num_possible_clicks: num_clicks,
        }
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

    pub fn iter_switches(&self) -> impl Iterator<Item = &Sw>
    where
        Sw: Eq + Hash,
    {
        self.switches.keys()
    }

    pub fn on_press_event(&mut self, switch: Sw) -> Result<LongPressHandleRequest, TimedPressError>
    where
        Sw: Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        let entry = self.switches.entry(switch);
        match entry {
            Entry::Occupied(mut entry) => entry.get_mut().on_press_event(),
            Entry::Vacant(entry) => {
                let (state, request) = SwitchState::from_pressed();
                let _: &mut _ = entry.insert(state);
                Ok(request)
            }
        }
    }

    pub fn on_release_event(
        &mut self,
        switch: Sw,
    ) -> Result<Option<(TimedReleaseEventData, ClickExactHandleRequest)>, TimedReleaseError>
    where
        Sw: Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        let entry = self.switches.entry(switch);
        match entry {
            Entry::Occupied(mut entry) => entry.get_mut().on_release_event().map(Some),
            Entry::Vacant(_) => Ok(None),
        }
    }

    fn on_timeout_event<T, E>(
        &mut self,
        switch: Sw,
        callback: impl FnOnce(OccupiedEntry<'_, Sw, SwitchState>) -> Result<T, E>,
        err: E,
    ) -> Result<T, E>
    where
        Sw: Eq + Hash,
    {
        use std::collections::hash_map::Entry;

        let entry = self.switches.entry(switch);
        match entry {
            Entry::Occupied(entry) => callback(entry),
            Entry::Vacant(_) => Err(err),
        }
    }

    pub fn on_long_press_event(
        &mut self,
        switch: Sw,
        request: LongPressHandleRequest,
    ) -> Result<Option<TimedLongPressEventData>, TimedLongClickError>
    where
        Sw: Eq + Hash,
    {
        if let Some(_) = request.0.upgrade() {
            self.on_timeout_event(
                switch,
                SwitchState::with_long_press_event,
                TimedLongClickError::Default,
            )
            .map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn on_click_exact_event(
        &mut self,
        switch: Sw,
        request: ClickExactHandleRequest,
    ) -> Result<Option<TimedClickExactEventData>, TimedClickExactError>
    where
        Sw: Eq + Hash,
    {
        if let Some(_) = request.0.upgrade() {
            self.on_timeout_event(
                switch,
                SwitchState::with_click_exact_event,
                TimedClickExactError::Default,
            )
            .map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn on_reset_click_count(&mut self, switch: &Sw) -> Result<(), WithResetClickCountError>
    where
        Sw: Eq + Hash,
    {
        let entry = self.switches.get_mut(&switch);
        match entry {
            Some(state) => {
                state.num_possible_clicks = 0;
                Ok(())
            }
            None => Err(WithResetClickCountError::Default),
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
    fn new(kind: SwitchStateKind, num_clicks: NumPossibleClicks) -> Self {
        Self {
            kind,
            num_possible_clicks: num_clicks,
        }
    }

    fn from_pressed() -> (Self, LongPressHandleRequest) {
        let tag = Arc::new(());
        let request = LongPressHandleRequest(Arc::downgrade(&tag));
        let state = Self::new(SwitchStateKind::Pressed(tag), 1);
        (state, request)
    }

    fn on_press_event(&mut self) -> Result<LongPressHandleRequest, TimedPressError> {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => Err(TimedPressError::AlreadyPressed),
            LongPressed => Err(TimedPressError::AlreadyLongPressed),
            Released(_) | LongReleased(_) => {
                let tag = Arc::new(());
                let request = LongPressHandleRequest(Arc::downgrade(&tag));
                self.kind = Pressed(tag);
                self.num_possible_clicks += 1;
                Ok(request)
            }
        }
    }

    fn on_release_event(
        &mut self,
    ) -> Result<(TimedReleaseEventData, ClickExactHandleRequest), TimedReleaseError> {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        match self.kind {
            Pressed(_) => {
                let tag = Arc::new(());
                let request = ClickExactHandleRequest(Arc::downgrade(&tag));
                self.kind = Released(tag);
                let event =
                    TimedEventData::new(TimedReleaseEventKind::Click, self.num_possible_clicks);
                Ok((event, request))
            }

            LongPressed => {
                let tag = Arc::new(());
                let request = ClickExactHandleRequest(Arc::downgrade(&tag));
                self.kind = LongReleased(tag);
                let event =
                    TimedEventData::new(TimedReleaseEventKind::LongClick, self.num_possible_clicks);
                Ok((event, request))
            }

            Released(_) => Err(TimedReleaseError::AlreadyReleased),
            LongReleased(_) => Err(TimedReleaseError::AlreadyLongReleased),
        }
    }

    fn with_long_press_event<Sw>(
        mut entry: OccupiedEntry<'_, Sw, SwitchState>,
    ) -> Result<TimedLongPressEventData, TimedLongClickError> {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        let state = entry.get_mut();
        match state.kind {
            Pressed(_) => {
                state.kind = LongPressed;
                let event = TimedLongPressEventData::new(
                    TimedLongPressEventKind::LongPress,
                    state.num_possible_clicks,
                );
                Ok(event)
            }
            LongPressed => Err(TimedLongClickError::LongPressed),
            Released(_) => Err(TimedLongClickError::Released),
            LongReleased(_) => Err(TimedLongClickError::LongReleased),
        }
    }

    fn with_click_exact_event<Sw>(
        entry: OccupiedEntry<'_, Sw, SwitchState>,
    ) -> Result<TimedClickExactEventData, TimedClickExactError> {
        use SwitchStateKind::{LongPressed, LongReleased, Pressed, Released};

        match entry.get().kind {
            Pressed(_) => Err(TimedClickExactError::Pressed),
            LongPressed => Err(TimedClickExactError::LongPressed),
            Released(_) => {
                let state = entry.remove();
                let event = TimedClickExactEventData::new(
                    TimedClickExactEventKind::ClickExact,
                    state.num_possible_clicks,
                );
                Ok(event)
            }
            LongReleased(_) => {
                let state = entry.remove();
                let event = TimedClickExactEventData::new(
                    TimedClickExactEventKind::LongClickExact,
                    state.num_possible_clicks,
                );
                Ok(event)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Error)]
pub enum TimedPressError {
    #[error("Button is pressed while in Pressed state")]
    AlreadyPressed,
    #[error("Button is pressed while in LongPressed state")]
    AlreadyLongPressed,
}

#[derive(Clone, Copy, Debug, Error)]
pub enum TimedReleaseError {
    #[error("Button is released while in Released state")]
    AlreadyReleased,
    #[error("Button is released while in LongPressed state")]
    AlreadyLongReleased,
}

#[derive(Clone, Copy, Debug, Error)]
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

#[derive(Clone, Copy, Debug, Error)]
pub enum TimedClickExactError {
    #[error("No handler calls requested for Default state")]
    Default,
    #[error("No handler calls requested for Pressed state")]
    Pressed,
    #[error("No handler calls requested for LongPressed state")]
    LongPressed,
}

#[derive(Clone, Copy, Debug, Error)]
pub enum WithResetClickCountError {
    #[error("No handler calls requested for Default state")]
    Default,
}
