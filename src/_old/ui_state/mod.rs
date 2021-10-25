//mod context;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    marker::PhantomData,
    sync::{Arc, Weak},
};

pub type UiEventTimeStampMs = u64;
pub type UiEventCoords = (i32, i32);
pub type MouseButton = u32;
pub type KeyboardKey = String;
pub type AxisValue = i32;
pub type MouseScrollDelta = i32;
pub type TouchId = u32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiRawInputEvent {
    pub kind: UiRawInputEventKind,
    pub timestamp: UiEventTimeStampMs,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiRawInputEventKind {
    KeyDown {
        key: KeyboardKey,
    },
    KeyUp {
        key: KeyboardKey,
    },
    MouseDown {
        button: MouseButton,
    },
    MouseUp {
        button: MouseButton,
    },
    MouseMove {
        coords: UiEventCoords,
    },
    MouseWheelDown,
    MouseWheelUp,
    MouseScroll {
        delta: MouseScrollDelta,
    },
    TouchStart {
        touch_id: TouchId,
        coords: UiEventCoords,
    },
    TouchEnd {
        touch_id: TouchId,
    },
    TouchMove {
        touch_id: TouchId,
        coords: UiEventCoords,
    },
    Char {
        ch: String,
    },
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ButtonKind {
    KeyboardKey(KeyboardKey),
    MouseButton(MouseButton),
    Touch(TouchId),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AxisKind {
    MouseX,
    MouseY,
    TouchX(TouchId),
    TouchY(TouchId),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Axis {
    kind: AxisKind,
    value: Option<AxisValue>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TriggerKind {
    MouseWheelUp,
    MouseWheelDown,
    MouseScroll(MouseScrollDelta),
    Char(String),
    CharRepeat(String),
    MouseMove,
    TouchMove,
}

#[derive(Clone, Debug)]
pub struct UiInputEventWithModifiers<T> {
    pub kind: T,
    pub modifiers: Arc<Modifiers>,
    pub timestamp: UiEventTimeStampMs,
}

pub type UiModifiedInputEvent = UiInputEventWithModifiers<UiModifiedInputEventKind>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UiModifiedInputEventKind {
    Press(ButtonKind),
    Release(ButtonKind),
    Repeat(ButtonKind),
    Change(Axis),
    Trigger(TriggerKind),
}

#[derive(Clone, Debug, Default)]
pub struct Modifiers {
    buttons: HashSet<ButtonKind>,
    axes: HashMap<AxisKind, AxisValue>,
}

#[derive(Clone, Debug)]
pub struct UiRawInputState<T: UiRawInputContext> {
    modifiers: Arc<Modifiers>,
    context: T,
}

pub struct UiRawInputStateUpdater<T: UiRawInputContext> {
    modifiers: Modifiers,
    context: T,
    kinds: Vec<UiModifiedInputEventKind>,
    timestamp: UiEventTimeStampMs,
}

pub trait UiRawInputContext: Sized {
    fn emit_event(self, ev: UiModifiedInputEvent) -> Self;
}

impl UiRawInputEvent {
    pub fn new(kind: UiRawInputEventKind, timestamp: UiEventTimeStampMs) -> Self {
        Self { kind, timestamp }
    }
}

impl<T: UiRawInputContext> UiRawInputStateUpdater<T> {
    pub fn new(
        state: UiRawInputState<T>,
        timestamp: UiEventTimeStampMs,
    ) -> UiRawInputStateUpdater<T> {
        Self {
            modifiers: state.modifiers.as_ref().to_owned(),
            context: state.context,
            timestamp,
            kinds: Vec::new(),
        }
    }

    fn with_trigger(mut self, trigger: TriggerKind) -> Self {
        self.kinds.push(UiModifiedInputEventKind::Trigger(trigger));
        self
    }

    fn with_button_pressed(mut self, button: ButtonKind) -> Self {
        self.kinds
            .push(UiModifiedInputEventKind::Press(button.clone()));
        let is_added = self.modifiers.buttons.insert(button);
        assert!(is_added);
        self
    }

    fn with_button_released(mut self, button: ButtonKind) -> Self {
        self.kinds
            .push(UiModifiedInputEventKind::Release(button.clone()));
        let is_removed = self.modifiers.buttons.remove(&button);
        assert!(is_removed);
        self
    }

    fn with_axis_changed(mut self, axis: Axis) -> Self {
        self.kinds
            .push(UiModifiedInputEventKind::Change(axis.clone()));
        match axis.value {
            Some(value) => {
                let _ = self.modifiers.axes.insert(axis.kind, value);
            }
            None => {
                let _ = self.modifiers.axes.remove(&axis.kind);
            }
        }

        self
    }

    pub fn apply(self) -> UiRawInputState<T> {
        assert!(!self.kinds.is_empty());
        let modifiers = Arc::new(self.modifiers);
        let mut context = self.context;
        for kind in self.kinds {
            context = context.emit_event(UiModifiedInputEvent {
                kind,
                modifiers: Arc::clone(&modifiers),
                timestamp: self.timestamp,
            });
        }
        UiRawInputState { modifiers, context }
    }
}

impl<T: UiRawInputContext> UiRawInputState<T> {
    pub fn new(context: T) -> Self {
        Self {
            modifiers: Arc::default(),
            context,
        }
    }

    pub fn make_event(self, timestamp: UiEventTimeStampMs) -> UiRawInputStateUpdater<T> {
        UiRawInputStateUpdater::new(self, timestamp)
    }

    pub fn with_event(self, ev: UiRawInputEvent) -> Self {
        use UiRawInputEventKind::*;

        let event = self.make_event(ev.timestamp);
        let updater = match ev.kind {
            KeyDown { key } => event.with_button_pressed(ButtonKind::KeyboardKey(key)),
            KeyUp { key } => event.with_button_released(ButtonKind::KeyboardKey(key)),
            MouseDown { button } => event.with_button_pressed(ButtonKind::MouseButton(button)),
            MouseUp { button } => event.with_button_released(ButtonKind::MouseButton(button)),
            MouseMove { coords } => event
                .with_axis_changed(Axis::new(AxisKind::MouseX, Some(coords.0)))
                .with_axis_changed(Axis::new(AxisKind::MouseY, Some(coords.1))),
            MouseWheelDown => event.with_trigger(TriggerKind::MouseWheelDown),
            MouseWheelUp => event.with_trigger(TriggerKind::MouseWheelUp),
            MouseScroll { delta } => event.with_trigger(TriggerKind::MouseScroll(delta)),
            TouchStart { touch_id, coords } => event
                .with_button_pressed(ButtonKind::Touch(touch_id))
                .with_axis_changed(Axis::new(AxisKind::TouchX(touch_id), Some(coords.0)))
                .with_axis_changed(Axis::new(AxisKind::TouchY(touch_id), Some(coords.1))),
            TouchMove { touch_id, coords } => event
                .with_axis_changed(Axis::new(AxisKind::TouchX(touch_id), Some(coords.0)))
                .with_axis_changed(Axis::new(AxisKind::TouchY(touch_id), Some(coords.1))),
            TouchEnd { touch_id } => event.with_button_released(ButtonKind::Touch(touch_id)),
            Char { ch } => event.with_trigger(TriggerKind::Char(ch)),
        };
        updater.apply()
    }
}

impl Axis {
    fn new(kind: AxisKind, value: Option<AxisValue>) -> Self {
        Self { kind, value }
    }
}

impl ButtonKind {
    fn long_click_duration(&self) -> UiTimedInputLongClickDuration {
        match self {
            ButtonKind::KeyboardKey(_) => UiTimedInputLongClickDuration::Key,
            ButtonKind::MouseButton(_) => UiTimedInputLongClickDuration::Mouse,
            ButtonKind::Touch(_) => UiTimedInputLongClickDuration::Touch,
        }
    }

    fn multi_click_duration(&self) -> UiTimedInputMultiClickDuration {
        match self {
            ButtonKind::KeyboardKey(_) => UiTimedInputMultiClickDuration::Key,
            ButtonKind::MouseButton(_) => UiTimedInputMultiClickDuration::Mouse,
            ButtonKind::Touch(_) => UiTimedInputMultiClickDuration::Touch,
        }
    }
}

// ====

pub type NumClicks = u32;

#[derive(Clone, Debug)]
pub struct ScheduledTimeout {
    button: ButtonKind,
}

#[derive(Clone, Debug)]
pub struct UiTimedInputState<T: UiTimedInputContext> {
    buttons: HashMap<ButtonKind, ButtonTimedState>,
    context: T,
}

#[derive(Clone, Debug)]
pub struct ButtonTimedState {
    kind: ButtonTimedStateKind,
    modifiers: Arc<Modifiers>,
    num_clicks: NumClicks,
}

#[derive(Clone, Debug)]
pub enum ButtonTimedStateKind {
    Pressed { timeout: Arc<ScheduledTimeout> },
    LongPressed,
    Released { timeout: Arc<ScheduledTimeout> },
    LongReleased { timeout: Arc<ScheduledTimeout> },
}

#[derive(Clone, Debug)]
pub enum UiTimedInputDuration {
    LongClick(UiTimedInputLongClickDuration),
    MultiClick(UiTimedInputMultiClickDuration),
}

#[derive(Clone, Debug)]
pub enum UiTimedInputLongClickDuration {
    Key,
    Mouse,
    Touch,
}

#[derive(Clone, Debug)]
pub enum UiTimedInputMultiClickDuration {
    Key,
    Mouse,
    Touch,
}

pub trait UiTimedInputContext: Sized {
    fn schedule(
        self,
        button: ButtonKind,
        delay: UiTimedInputDuration,
    ) -> (Self, Arc<ScheduledTimeout>);
    fn emit_event(self, ev: UiTimedInputEvent) -> Self;
}

pub type UiTimedInputEvent = UiInputEventWithModifiers<UiTimedInputEventKind>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UiTimedInputEventKind {
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

impl<T: UiTimedInputContext> UiTimedInputState<T> {
    pub fn new(context: T) -> Self {
        Self {
            buttons: HashMap::default(),
            context,
        }
    }

    pub fn with_event(self, ev: UiModifiedInputEvent) -> Self {
        use std::collections::hash_map::Entry;
        use UiModifiedInputEventKind::*;

        let modifiers = ev.modifiers;
        let timestamp = ev.timestamp;
        let mut buttons = self.buttons;
        let context = self.context;
        let context = match ev.kind {
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

    pub fn with_timeout_event(mut self, button: ButtonKind, timestamp: UiEventTimeStampMs) -> Self {
        let state = self.buttons.remove(&button).unwrap();
        let mut buttons = self.buttons;
        let (state, context) = state.with_timeout_event(button.clone(), timestamp, self.context);
        if let Some(state) = state {
            buttons.insert(button, state);
        }
        Self { buttons, context }
    }
}

impl ButtonTimedState {
    fn from_pressed<T: UiTimedInputContext>(
        button: ButtonKind,
        modifiers: &Arc<Modifiers>,
        mut context: T,
    ) -> (Self, T) {
        let delay = UiTimedInputDuration::LongClick(button.long_click_duration());
        let (context, timeout) = context.schedule(button, delay);
        let kind = ButtonTimedStateKind::Pressed { timeout };
        let modifiers = Arc::clone(modifiers);
        (
            ButtonTimedState {
                kind,
                modifiers,
                num_clicks: 0,
            },
            context,
        )
    }

    fn with_press_event<T: UiTimedInputContext>(self, button: ButtonKind, context: T) -> (Self, T) {
        use ButtonTimedStateKind::*;

        match self.kind {
            Pressed { timeout: _ } => {
                panic!(); // TODO: warn
            }
            LongPressed {} => {
                panic!(); // TODO: warn
            }
            Released { timeout: _ } => {
                let delay = UiTimedInputDuration::LongClick(button.long_click_duration());
                let (context, timeout) = context.schedule(button, delay);
                (
                    ButtonTimedState {
                        kind: ButtonTimedStateKind::Pressed { timeout },
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks,
                    },
                    context,
                )
            }
            LongReleased { timeout: _ } => {
                let delay = UiTimedInputDuration::LongClick(button.long_click_duration());
                let (context, timeout) = context.schedule(button, delay);
                (
                    ButtonTimedState {
                        kind: ButtonTimedStateKind::Pressed { timeout },
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks,
                    },
                    context,
                )
            }
        }
    }

    fn with_release_event<T: UiTimedInputContext>(
        self,
        button: ButtonKind,
        timestamp: UiEventTimeStampMs,
        context: T,
    ) -> (Self, T) {
        use ButtonTimedStateKind::*;

        match self.kind {
            Pressed { timeout: _ } => {
                let context = context.emit_event(UiTimedInputEvent::new(
                    UiTimedInputEventKind::Click {
                        button: button.clone(),
                        num_clicks: self.num_clicks + 1,
                    },
                    Arc::clone(&self.modifiers),
                    timestamp,
                ));
                let delay = UiTimedInputDuration::MultiClick(button.multi_click_duration());
                let (context, timeout) = context.schedule(button, delay);
                (
                    ButtonTimedState {
                        kind: ButtonTimedStateKind::Released { timeout },
                        modifiers: self.modifiers,
                        num_clicks: self.num_clicks + 1,
                    },
                    context,
                )
            }
            LongPressed => {
                let context = context.emit_event(UiTimedInputEvent::new(
                    UiTimedInputEventKind::LongClick {
                        button: button.clone(),
                        num_clicks: self.num_clicks + 1,
                    },
                    Arc::clone(&self.modifiers),
                    timestamp,
                ));
                let delay = UiTimedInputDuration::MultiClick(button.multi_click_duration());
                let (context, timeout) = context.schedule(button, delay);
                (
                    ButtonTimedState {
                        kind: ButtonTimedStateKind::Released { timeout },
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

    pub fn with_timeout_event<T: UiTimedInputContext>(
        self,
        button: ButtonKind,
        timestamp: UiEventTimeStampMs,
        mut context: T,
    ) -> (Option<Self>, T) {
        use ButtonTimedStateKind::*;

        match self.kind {
            Pressed { timeout: _ } => {
                let context = context.emit_event(UiInputEventWithModifiers::new(
                    UiTimedInputEventKind::LongPress {
                        button,
                        num_clicks: self.num_clicks,
                    },
                    Arc::clone(&self.modifiers),
                    timestamp,
                ));
                (
                    Some(ButtonTimedState {
                        kind: ButtonTimedStateKind::LongPressed,
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
                let context = context.emit_event(UiInputEventWithModifiers::new(
                    UiTimedInputEventKind::ClickExact {
                        button,
                        num_clicks: self.num_clicks,
                    },
                    Arc::clone(&self.modifiers),
                    timestamp,
                ));
                (None, context)
            }
            LongReleased { timeout: _ } => {
                let context = context.emit_event(UiInputEventWithModifiers::new(
                    UiTimedInputEventKind::LongClickExact {
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

impl UiInputEventWithModifiers<UiTimedInputEventKind> {
    pub fn new(
        kind: UiTimedInputEventKind,
        modifiers: Arc<Modifiers>,
        timestamp: UiEventTimeStampMs,
    ) -> Self {
        Self {
            kind,
            modifiers,
            timestamp,
        }
    }
}

// ====

pub type UiInputEvent = UiInputEventWithModifiers<UiInputEventKind>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UiInputEventKind {
    Modified(UiModifiedInputEventKind),
    Timed(UiTimedInputEventKind),
}

// ====

#[derive(Clone, Debug)]
pub struct UiAppEvent {
    kind: UiAppEventKind,
    timestamp: UiEventTimeStampMs,
}

#[derive(Clone, Debug)]
pub enum UiAppEventKind {
    AppEvent1,
    AppEvent2,
    AppEvent3,
    AppEvent4,
}

#[derive(Clone, Debug)]
pub struct UiInputBindings(HashMap<UiInputEventKind, Vec<(Modifiers, UiAppEventKind)>>);

impl UiInputBindings {
    fn process<F: FnMut(UiAppEvent)>(&self, ev: UiInputEvent, mut emit_fn: F) {
        let empty_vec = Vec::new();
        let bindings = self.0.get(&ev.kind).unwrap_or(&empty_vec);
        let mut bindings: Vec<_> = bindings
            .into_iter()
            .filter(|(modifiers, _)| {
                ev.modifiers.buttons.is_superset(&modifiers.buttons)
                /* TODO: for axes */
            })
            .map(Option::Some)
            .collect();
        for j1 in 0..bindings.len() {
            for j2 in 0..bindings.len() {
                if j1 != j2 {
                    match (bindings[j1], bindings[j2]) {
                        (Some(binding1), Some(binding2)) => {
                            if binding1.0.buttons.is_superset(&binding2.0.buttons) {
                                bindings[j2] = None;
                            }
                        }
                        _ => {}
                    }
                }
                /* TODO: for axes */
            }
        }

        let bindings_buttons: HashSet<_> = bindings
            .iter()
            .filter_map(|binding| *binding)
            .map(|binding| -> Vec<_> { binding.0.buttons.iter().collect() })
            .collect();
        if bindings_buttons.len() == 1 {
            for binding in bindings.into_iter().filter_map(|binding| binding) {
                emit_fn(UiAppEvent {
                    kind: binding.1.clone(),
                    timestamp: ev.timestamp,
                })
            }
        }
    }
}

#[test]
fn ui_raw_input_to_input_test() {
    #[derive(Clone, Debug)]
    struct UiRawInputSimpleContext {
        time: UiEventTimeStampMs,
        state: UiTimedInputState<UiTimedInputSimpleContext>,
    }

    impl UiRawInputState<UiRawInputSimpleContext> {
        fn with_context_event(mut self, ev: UiRawInputEvent) -> Self {
            // TODO: Remove
            println!("{:?}", ev);

            assert!(self.context.time < ev.timestamp);
            self.context.time = ev.timestamp;
            self = self.with_event(ev);

            // TODO: Remove
            println!("{:?}", self);
            for event in &self.context.state.context.events {
                println!("{:?}", event);
            }
            println!();
            self.context.state.context.events.clear();
            self
        }
    }

    impl UiRawInputContext for UiRawInputSimpleContext {
        fn emit_event(mut self, ev: UiInputEventWithModifiers<UiModifiedInputEventKind>) -> Self {
            self.state.context.events.push(UiInputEvent {
                kind: UiInputEventKind::Modified(ev.kind.clone()),
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
    struct UiTimedInputSimpleContext {
        time: UiEventTimeStampMs,
        timeouts: BTreeMap<UiEventTimeStampMs, Weak<ScheduledTimeout>>,
        events: Vec<UiInputEvent>,
    }

    impl UiTimedInputState<UiTimedInputSimpleContext> {
        fn with_context_event(
            mut self,
            ev: UiInputEventWithModifiers<UiModifiedInputEventKind>,
        ) -> Self {
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

    impl UiTimedInputContext for UiTimedInputSimpleContext {
        fn schedule(
            mut self,
            button: ButtonKind,
            delay: UiTimedInputDuration,
        ) -> (Self, Arc<ScheduledTimeout>) {
            let timeout = Arc::new(ScheduledTimeout { button });
            let delay = match delay {
                UiTimedInputDuration::LongClick(_) => 1000,
                UiTimedInputDuration::MultiClick(_) => 300,
            };
            self.timeouts
                .insert(self.time + delay, Arc::downgrade(&timeout));
            (self, timeout)
        }

        fn emit_event(mut self, ev: UiInputEventWithModifiers<UiTimedInputEventKind>) -> Self {
            self.events.push(UiInputEvent {
                kind: UiInputEventKind::Timed(ev.kind),
                modifiers: ev.modifiers,
                timestamp: ev.timestamp,
            });
            self
        }
    }

    use UiRawInputEvent as Ev;
    use UiRawInputEventKind::*;

    let ctrl = || "ctrl".to_owned();

    let timed_context = UiTimedInputSimpleContext {
        time: 0,
        timeouts: BTreeMap::new(),
        events: vec![],
    };
    let timed_state = UiTimedInputState::new(timed_context);
    let context = UiRawInputSimpleContext {
        time: 0,
        state: timed_state,
    };
    let state = UiRawInputState::new(context);

    let state = state.with_context_event(Ev::new(KeyDown { key: ctrl() }, 10000));
    let state = state.with_context_event(Ev::new(KeyUp { key: ctrl() }, 10500));
    let state = state.with_context_event(Ev::new(KeyDown { key: ctrl() }, 11000));
    let state = state.with_context_event(Ev::new(KeyUp { key: ctrl() }, 13000));

    let state = state.with_context_event(Ev::new(KeyDown { key: ctrl() }, 15000));
    let state = state.with_context_event(Ev::new(MouseDown { button: 0 }, 15100));
    let state = state.with_context_event(Ev::new(MouseUp { button: 0 }, 15200));
    let state = state.with_context_event(Ev::new(MouseDown { button: 0 }, 15300));
    let state = state.with_context_event(Ev::new(MouseUp { button: 0 }, 15400));

    let state = state.with_context_event(Ev::new(KeyUp { key: ctrl() }, 18000));

    // TODO: check states
}

#[test]
fn ui_input_binding_test() {
    let lmb = UiInputEventKind::Timed(UiTimedInputEventKind::Click {
        button: ButtonKind::MouseButton(0),
        num_clicks: 1,
    });
    let rmb = UiInputEventKind::Timed(UiTimedInputEventKind::Click {
        button: ButtonKind::MouseButton(1),
        num_clicks: 1,
    });
    let bindings = UiInputBindings(
        [
            (
                lmb,
                vec![
                    (
                        Modifiers {
                            buttons: [ButtonKind::KeyboardKey("Ctrl".to_owned())]
                                .into_iter()
                                .collect(),
                            axes: HashMap::new(),
                        },
                        UiAppEventKind::AppEvent1,
                    ),
                    (
                        Modifiers {
                            buttons: [ButtonKind::KeyboardKey("Shift".to_owned())]
                                .into_iter()
                                .collect(),
                            axes: HashMap::new(),
                        },
                        UiAppEventKind::AppEvent2,
                    ),
                    (
                        Modifiers {
                            buttons: [
                                ButtonKind::KeyboardKey("Ctrl".to_owned()),
                                ButtonKind::KeyboardKey("Alt".to_owned()),
                            ]
                            .into_iter()
                            .collect(),
                            axes: HashMap::new(),
                        },
                        UiAppEventKind::AppEvent3,
                    ),
                ],
            ),
            (
                rmb,
                vec![(
                    Modifiers {
                        buttons: [ButtonKind::KeyboardKey("Ctrl".to_owned())]
                            .into_iter()
                            .collect(),
                        axes: HashMap::new(),
                    },
                    UiAppEventKind::AppEvent4,
                )],
            ),
        ]
        .into_iter()
        .collect(),
    );

    let mut events = vec![];

    bindings.process(
        UiInputEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: HashSet::new(),
                axes: HashMap::new(),
            }),
            kind: UiInputEventKind::Timed(UiTimedInputEventKind::Click {
                button: ButtonKind::MouseButton(0),
                num_clicks: 1,
            }),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    bindings.process(
        UiInputEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: HashSet::new(),
                axes: HashMap::new(),
            }),
            kind: UiInputEventKind::Timed(UiTimedInputEventKind::Click {
                button: ButtonKind::MouseButton(1),
                num_clicks: 1,
            }),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    bindings.process(
        UiInputEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: [ButtonKind::KeyboardKey("Ctrl".to_owned())]
                    .into_iter()
                    .collect(),
                axes: HashMap::new(),
            }),
            kind: UiInputEventKind::Timed(UiTimedInputEventKind::Click {
                button: ButtonKind::MouseButton(0),
                num_clicks: 1,
            }),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    bindings.process(
        UiInputEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: [
                    ButtonKind::KeyboardKey("Ctrl".to_owned()),
                    ButtonKind::KeyboardKey("Alt".to_owned()),
                ]
                .into_iter()
                .collect(),
                axes: HashMap::new(),
            }),
            kind: UiInputEventKind::Timed(UiTimedInputEventKind::Click {
                button: ButtonKind::MouseButton(0),
                num_clicks: 1,
            }),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    bindings.process(
        UiInputEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: [
                    ButtonKind::KeyboardKey("Ctrl".to_owned()),
                    ButtonKind::KeyboardKey("Shift".to_owned()),
                ]
                .into_iter()
                .collect(),
                axes: HashMap::new(),
            }),
            kind: UiInputEventKind::Timed(UiTimedInputEventKind::Click {
                button: ButtonKind::MouseButton(0),
                num_clicks: 1,
            }),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    bindings.process(
        UiInputEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: [ButtonKind::KeyboardKey("Shift".to_owned())]
                    .into_iter()
                    .collect(),
                axes: HashMap::new(),
            }),
            kind: UiInputEventKind::Timed(UiTimedInputEventKind::Click {
                button: ButtonKind::MouseButton(0),
                num_clicks: 1,
            }),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    panic!();
}

/*

    UiTimedInputState: button => ButtonTimedState
    ButtonTimedState: timeout, num_clicks


    UiInputEventWithModifiers<UiModifiedInputEventKind> | UiTimedInputState -> UiInputEventWithModifiers<UiTimedInputEventKind>



    UiRawInputEvent
        KeyUp, MouseMove, TouchStart, etc., KeyRepeat,

    UiInputEventWithModifiers<UiModifiedInputEventKind>
        Press (modifiers on press)
        Repeat (modifiers on repeat)
        Release (modifiers on release)
        Change (modifiers on change)
            mouse x, y
            touch id, x, y
            axes id, x
        Event/Trigger
            MouseWheel (modifiers)
            Char (modifiers)
            CharRepeat (modifiers)

    UiInputEventWithModifiers<UiTimedInputEventKind>
        LongPress (modifiers on first press)
        Click (modifiers on first press)
        LongClick (modifiers on first press)

    UiRawInputEvent | UiRawInputState -> UiInputEventWithModifiers<UiModifiedInputEventKind>
    UiInputEventWithModifiers<UiModifiedInputEventKind> | UiTimedInputState -> UiInputEventWithModifiers<UiTimedInputEventKind>
    UiInputEventWithModifiers<UiModifiedInputEventKind> + UiInputEventWithModifiers<UiTimedInputEventKind> -> UiInputEvent
    UiInputEvent | BindingProcessor -> UiAppEvent  //todo axes
                    ^ disabled events
    UiAppEvent | UiAppState -> ...

    event override
        override event with short modifiers (Ctrl+Lmb)
        by an event with longer modifiers (Ctrl+Shift+Lmb)
    event override rejection
        do no override shorter one by longer one
        if longer one is disabled by AppState
        therefore can not be handled.
*/
