use core::hash::Hash;
use std::collections::hash_map::OccupiedEntry;
use std::collections::HashMap;
use std::sync::{Arc, Weak};

use thiserror::Error;

/// A type used to store the number of clicks.
pub type NumPossibleClicks = u32;

/// A structure that stores time-related input event parameters.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimedEventData<Ki> {
    pub kind: Ki,
    pub num_possible_clicks: NumPossibleClicks,
}

/// A structure that stores input event parameters for switch release/click event.
pub type TimedReleaseEventData = TimedEventData<TimedReleaseEventKind>;

/// A structure that stores input event parameters for switch long press event.
///
/// This event occurs if the switch has been pressed for a specified duration.
pub type TimedLongPressEventData = TimedEventData<TimedLongPressEventKind>;

/// A structure that stores input event parameters for switch click exact event.
///
/// This event occurs if no press/enable switch event has occurred
/// within the specified registration time of multi-click after the press.
/// Thus, while the release/click event will be generated for each multi-click
/// (that is: single-click, double-click, triple-click)
/// this will on the contrary be generated only for the last one (that is: exact-triple-click).
pub type TimedClickExactEventData = TimedEventData<TimedClickExactEventKind>;

/// A enumeration that specifies switch release/click event kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimedReleaseEventKind {
    Click,
    LongClick,
}

/// A enumeration that specifies switch long pressed event kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimedLongPressEventKind {
    LongPress,
}

/// A enumeration that specifies switch exact click event kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimedClickExactEventKind {
    ClickExact,
    LongClickExact,
}

/// A structure that stores time-related input state.
#[derive(Clone, Debug)]
pub struct TimedState<Sw> {
    switches: HashMap<Sw, SwitchState>,
}

/// A structure that stores time-related input state for a specified switch.
#[derive(Clone, Debug)]
struct SwitchState {
    kind: SwitchStateKind,
    num_possible_clicks: NumPossibleClicks,
}

/// A enumeration that specifies time-related input state for switch.
///
/// The default state of the switch is not represented by this enumeration.
/// Accordingly, it is suggested to use absent value or None to represent it.
///
/// If state switch is in the `Pressed`, `Released` or `LongReleased` state,
/// a delayed callback is scheduled.
/// Reference-counting pointer stored in these variants denote a unique scheduled callback marker.
/// When transitioning to a new state callback marker used to invalidate scheduled request.
#[derive(Clone, Debug)]
enum SwitchStateKind {
    Pressed(Arc<()>),
    LongPressed,
    Released(Arc<()>),
    LongReleased(Arc<()>),
}

/// A long press scheduled callback request.
///
/// The internal pointer is invalidated if the state changes before the callback is called.
#[derive(Clone, Debug)]
pub struct LongPressHandleRequest(Weak<()>);

/// A click exact scheduled callback request.
///
/// The internal pointer is invalidated if the state changes before the callback is called.
#[derive(Clone, Debug)]
pub struct ClickExactHandleRequest(Weak<()>);

impl<Ki> TimedEventData<Ki> {
    /// Constructs a new, empty `TimedEventData` structure.
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
    /// Constructs a new, empty `TimedState` structure.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an iterator visiting all switches with non-default state.
    pub fn iter_switches(&self) -> impl Iterator<Item = &Sw>
    where
        Sw: Eq + Hash,
    {
        self.switches.keys()
    }

    /// The callback to be called once the switch has been pressed/activated.
    ///
    /// It is assumed that the application or other library itself calls this function
    /// when the user presses a button on the keyboard, mouse, or other device.
    ///
    /// This function returns a request for a delayed `on_long_press_event` function call.
    /// `on_long_press_event` must be called after a minimum long press recognition time interval.
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

    /// The callback to be called once the switch has been released/disabled.
    ///
    /// It is assumed that the application or other library itself calls this function
    /// when the user releases a button on the keyboard, mouse, or other device.
    ///
    /// This function optionally returns a request
    /// for a delayed `on_click_exact_event` function call.
    /// `on_click_exact_event` must be called after a maximum multi-click recognition time interval.
    ///
    /// This function also optionally returns `TimedReleaseEventData`
    /// if a click or long click event has been recognized.
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

    /// The callback to be called for a long press request
    /// after a specified minimum long press recognition time interval.
    ///
    /// It is assumed that the application or other library itself calls this function
    /// for a previously obtained `LongPressHandleRequest`.
    ///
    /// This function can return `TimedLongPressEventData`
    /// if a long press event has been recognized.
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

    /// The callback to be called for a click exact request
    /// after a specified maximum multi click recognition time interval.
    ///
    /// It is assumed that the application or other library itself calls this function
    /// for a previously obtained `ClickExactHandleRequest`.
    ///
    /// This function can return `TimedClickExactEventData`
    /// if a click exact event has been recognized.
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

    /// A method that should be called if the subsequent click
    /// should not be recognized as a continuation of the last multi click sequence.
    ///
    /// After calling this function the counting of the number of clicks
    /// for a given switch will start from the beginning.
    pub fn on_reset_click_count(&mut self, switch: &Sw) -> Result<(), TimedResetClickCountError>
    where
        Sw: Eq + Hash,
    {
        let entry = self.switches.get_mut(&switch);
        match entry {
            Some(state) => {
                state.num_possible_clicks = 0;
                Ok(())
            }
            None => Err(TimedResetClickCountError::Default),
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
    /// Constructs a `SwitchState` structure from its kind and number of clicks.
    fn new(kind: SwitchStateKind, num_clicks: NumPossibleClicks) -> Self {
        Self {
            kind,
            num_possible_clicks: num_clicks,
        }
    }

    /// Constructs a `SwitchState` from pressed switch.
    fn from_pressed() -> (Self, LongPressHandleRequest) {
        let tag = Arc::new(());
        let request = LongPressHandleRequest(Arc::downgrade(&tag));
        let state = Self::new(SwitchStateKind::Pressed(tag), 1);
        (state, request)
    }

    /// The callback to be called once the switch has been pressed/activated.
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

    /// The callback to be called once the switch has been released/disabled.
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

    /// The callback to be called when a long press event is recognized.
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

    /// The callback to be called when a click exact event is recognized.
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

/// The error type which is returned from `TimedState::on_press_event`.
#[derive(Clone, Copy, Debug, Error)]
pub enum TimedPressError {
    #[error("Button is pressed while in Pressed state")]
    AlreadyPressed,
    #[error("Button is pressed while in LongPressed state")]
    AlreadyLongPressed,
}

/// The error type which is returned from `TimedState::on_release_event`.
#[derive(Clone, Copy, Debug, Error)]
pub enum TimedReleaseError {
    #[error("Button is released while in Released state")]
    AlreadyReleased,
    #[error("Button is released while in LongPressed state")]
    AlreadyLongReleased,
}

/// The error type which is returned from `TimedState::on_long_press_event`.
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

/// The error type which is returned from `TimedState::on_click_exact_event`.
#[derive(Clone, Copy, Debug, Error)]
pub enum TimedClickExactError {
    #[error("No handler calls requested for Default state")]
    Default,
    #[error("No handler calls requested for Pressed state")]
    Pressed,
    #[error("No handler calls requested for LongPressed state")]
    LongPressed,
}

/// The error type which is returned from `TimedState::on_reset_click_count`.
#[derive(Clone, Copy, Debug, Error)]
pub enum TimedResetClickCountError {
    #[error("No handler calls requested for Default state")]
    Default,
}
