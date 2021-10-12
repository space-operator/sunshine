//mod context;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::{Arc, Weak},
};

pub type UiEventTimeStampMs = u64;
pub type UiEventCoords = (i32, i32);
pub type MouseButton = u32;
pub type KeyboardKey = String;
pub type AxisValue = f64;
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

#[derive(Clone, Debug, PartialEq)]
pub struct Axis {
    kind: AxisKind,
    value: Option<AxisValue>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
pub struct UiModifiedInputEvent {
    pub kinds: Vec<UiModifiedInputEventKind>,
    pub modifiers: Arc<Modifiers>,
    pub timestamp: UiEventTimeStampMs,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, Default)]
pub struct UiRawInputState {
    modifiers: Arc<Modifiers>,
}

pub struct UiRawInputStateUpdater<'a, T: UiRawInputContext> {
    ctx: &'a mut T,
    kinds: Vec<UiModifiedInputEventKind>,
    modifiers: Modifiers,
    timestamp: UiEventTimeStampMs,
}

pub trait UiRawInputContext {
    fn emit_event(&mut self, ev: UiModifiedInputEvent);
}

impl UiRawInputEvent {
    pub fn new(kind: UiRawInputEventKind, timestamp: UiEventTimeStampMs) -> Self {
        Self { kind, timestamp }
    }
}

impl<'a, T: UiRawInputContext> UiRawInputStateUpdater<'a, T> {
    pub fn new(
        state: UiRawInputState,
        ctx: &'a mut T,
        timestamp: UiEventTimeStampMs,
    ) -> UiRawInputStateUpdater<'a, T> {
        Self {
            ctx,
            timestamp,
            kinds: Vec::new(),
            modifiers: state.modifiers.as_ref().to_owned(),
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

    pub fn apply(mut self) -> UiRawInputState {
        assert!(!self.kinds.is_empty());
        let modifiers = Arc::new(self.modifiers);
        self.ctx.emit_event(UiModifiedInputEvent {
            kinds: self.kinds,
            modifiers: Arc::clone(&modifiers),
            timestamp: self.timestamp,
        });
        UiRawInputState { modifiers }
    }
}

impl UiRawInputState {
    pub fn make_event<'a, T: UiRawInputContext>(
        self,
        ctx: &'a mut T,
        timestamp: UiEventTimeStampMs,
    ) -> UiRawInputStateUpdater<'a, T> {
        UiRawInputStateUpdater::new(self, ctx, timestamp)
    }

    pub fn with_event<T: UiRawInputContext>(self, ev: UiRawInputEvent, mut ctx: &mut T) -> Self {
        use UiRawInputEventKind::*;

        let event = self.make_event(ctx, ev.timestamp);
        let updater = match ev.kind {
            KeyDown { key } => event.with_button_pressed(ButtonKind::KeyboardKey(key)),
            KeyUp { key } => event.with_button_released(ButtonKind::KeyboardKey(key)),
            MouseDown { button } => event.with_button_pressed(ButtonKind::MouseButton(button)),
            MouseUp { button } => event.with_button_released(ButtonKind::MouseButton(button)),
            MouseMove { coords } => event
                .with_axis_changed(Axis::new(AxisKind::MouseX, Some(coords.0 as f64)))
                .with_axis_changed(Axis::new(AxisKind::MouseY, Some(coords.1 as f64))),
            MouseWheelDown => event.with_trigger(TriggerKind::MouseWheelDown),
            MouseWheelUp => event.with_trigger(TriggerKind::MouseWheelUp),
            MouseScroll { delta } => event.with_trigger(TriggerKind::MouseScroll(delta)),
            TouchStart { touch_id, coords } => event
                .with_button_pressed(ButtonKind::Touch(touch_id))
                .with_axis_changed(Axis::new(AxisKind::TouchX(touch_id), Some(coords.0 as f64)))
                .with_axis_changed(Axis::new(AxisKind::TouchY(touch_id), Some(coords.1 as f64))),
            TouchMove { touch_id, coords } => event
                .with_axis_changed(Axis::new(AxisKind::TouchX(touch_id), Some(coords.0 as f64)))
                .with_axis_changed(Axis::new(AxisKind::TouchY(touch_id), Some(coords.1 as f64))),
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

#[derive(Clone, Debug, Default)]
pub struct UiTimedInputState {
    buttons: HashMap<ButtonKind, ButtonTimedState>,
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

pub trait UiTimedInputContext {
    fn schedule(
        &mut self,
        button: ButtonKind,
        delay: UiTimedInputDuration,
    ) -> Arc<ScheduledTimeout>;
    fn emit_event(&mut self, ev: UiTimedInputEvent);
}

#[derive(Clone, Debug)]
pub struct UiTimedInputEvent {
    pub kinds: UiTimedInputEventKind,
    pub modifiers: Arc<Modifiers>,
    pub timestamp: UiEventTimeStampMs,
    pub button: ButtonKind,
    pub num_clicks: NumClicks,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UiTimedInputEventKind {
    LongPress,
    Click,
    LongClick,
    ClickExact,
    LongClickExact,
}

impl UiTimedInputState {
    pub fn with_event<T: UiTimedInputContext>(
        mut self,
        ev: UiModifiedInputEvent,
        ctx: &mut T,
    ) -> Self {
        let modifiers = ev.modifiers;
        let timestamp = ev.timestamp;
        for kind in ev.kinds {
            self = self.with_event_kind(kind, &modifiers, timestamp, ctx);
        }
        self
    }

    pub fn with_event_kind<T: UiTimedInputContext>(
        mut self,
        kind: UiModifiedInputEventKind,
        modifiers: &Arc<Modifiers>,
        timestamp: UiEventTimeStampMs,
        ctx: &mut T,
    ) -> Self {
        use std::collections::hash_map::Entry;
        use UiModifiedInputEventKind::*;

        match kind {
            Press(button) => {
                let entry = self.buttons.entry(button.clone());
                match entry {
                    Entry::Occupied(entry) => {
                        let state = entry.remove();
                        let state = state.with_press_event(button.clone(), timestamp, ctx);
                        self.buttons.insert(button.clone(), state);
                    }
                    Entry::Vacant(entry) => {
                        let state = ButtonTimedState::from_pressed(button, modifiers, ctx);
                        entry.insert(state);
                    }
                }
            }
            Release(button) => {
                let entry = self.buttons.entry(button.clone());
                match entry {
                    Entry::Occupied(entry) => {
                        let state = entry.remove();
                        let state = state.with_release_event(button.clone(), timestamp, ctx);
                        self.buttons.insert(button, state);
                    }
                    Entry::Vacant(entry) => {}
                }
            }
            Repeat(_) => {}
            Change(_) => {}
            Trigger(_) => {}
        }
        self
    }

    pub fn with_timeout_event<T: UiTimedInputContext>(
        mut self,
        button: ButtonKind,
        timestamp: UiEventTimeStampMs,
        ctx: &mut T,
    ) -> Self {
        let state = self.buttons.remove(&button).unwrap();
        let state = state.with_timeout_event(button.clone(), timestamp, ctx);
        if let Some(state) = state {
            self.buttons.insert(button, state);
        }
        self
    }
}

impl ButtonTimedState {
    fn from_pressed<T: UiTimedInputContext>(
        button: ButtonKind,
        modifiers: &Arc<Modifiers>,
        ctx: &mut T,
    ) -> Self {
        let delay = UiTimedInputDuration::LongClick(button.long_click_duration());
        let timeout = ctx.schedule(button, delay);
        let kind = ButtonTimedStateKind::Pressed { timeout };
        let modifiers = Arc::clone(modifiers);
        ButtonTimedState {
            kind,
            modifiers,
            num_clicks: 0,
        }
    }

    fn with_press_event<T: UiTimedInputContext>(
        self,
        button: ButtonKind,
        timestamp: UiEventTimeStampMs,
        ctx: &mut T,
    ) -> Self {
        use ButtonTimedStateKind::*;

        match self.kind {
            Pressed { timeout } => {
                panic!(); // TODO: warn
            }
            LongPressed {} => {
                panic!(); // TODO: warn
            }
            Released { timeout } => {
                let delay = UiTimedInputDuration::LongClick(button.long_click_duration());
                let timeout = ctx.schedule(button, delay);
                ButtonTimedState {
                    kind: ButtonTimedStateKind::Pressed { timeout },
                    modifiers: self.modifiers,
                    num_clicks: self.num_clicks,
                }
            }
            LongReleased { timeout } => {
                let delay = UiTimedInputDuration::LongClick(button.long_click_duration());
                let timeout = ctx.schedule(button, delay);
                ButtonTimedState {
                    kind: ButtonTimedStateKind::Pressed { timeout },
                    modifiers: self.modifiers,
                    num_clicks: self.num_clicks,
                }
            }
        }
    }

    fn with_release_event<T: UiTimedInputContext>(
        self,
        button: ButtonKind,
        timestamp: UiEventTimeStampMs,
        ctx: &mut T,
    ) -> Self {
        use ButtonTimedStateKind::*;

        match self.kind {
            Pressed { timeout } => {
                ctx.emit_event(UiTimedInputEvent::new(
                    UiTimedInputEventKind::Click,
                    Arc::clone(&self.modifiers),
                    timestamp,
                    button.clone(),
                    self.num_clicks + 1,
                ));
                let delay = UiTimedInputDuration::MultiClick(button.multi_click_duration());
                let timeout = ctx.schedule(button, delay);
                ButtonTimedState {
                    kind: ButtonTimedStateKind::Pressed { timeout },
                    modifiers: self.modifiers,
                    num_clicks: self.num_clicks + 1,
                }
            }
            LongPressed => {
                ctx.emit_event(UiTimedInputEvent::new(
                    UiTimedInputEventKind::LongClick,
                    Arc::clone(&self.modifiers),
                    timestamp,
                    button.clone(),
                    self.num_clicks + 1,
                ));
                let delay = UiTimedInputDuration::MultiClick(button.multi_click_duration());
                let timeout = ctx.schedule(button, delay);
                ButtonTimedState {
                    kind: ButtonTimedStateKind::Pressed { timeout },
                    modifiers: self.modifiers,
                    num_clicks: self.num_clicks + 1,
                }
            }
            Released { timeout } => {
                panic!(); // TODO: warn
            }
            LongReleased { timeout } => {
                panic!(); // TODO: warn
            }
        }
    }

    pub fn with_timeout_event<T: UiTimedInputContext>(
        self,
        button: ButtonKind,
        timestamp: UiEventTimeStampMs,
        ctx: &mut T,
    ) -> Option<Self> {
        use ButtonTimedStateKind::*;

        match self.kind {
            Pressed { timeout } => {
                ctx.emit_event(UiTimedInputEvent::new(
                    UiTimedInputEventKind::LongPress,
                    Arc::clone(&self.modifiers),
                    timestamp,
                    button,
                    self.num_clicks,
                ));
                Some(ButtonTimedState {
                    kind: ButtonTimedStateKind::LongPressed,
                    modifiers: self.modifiers,
                    num_clicks: self.num_clicks,
                })
            }
            LongPressed => {
                panic!("timeout event has been received but timeout is not stored in button state");
            }
            Released { timeout } => {
                ctx.emit_event(UiTimedInputEvent::new(
                    UiTimedInputEventKind::ClickExact,
                    Arc::clone(&self.modifiers),
                    timestamp,
                    button,
                    self.num_clicks,
                ));
                None
            }
            LongReleased { timeout } => {
                ctx.emit_event(UiTimedInputEvent::new(
                    UiTimedInputEventKind::LongClickExact,
                    Arc::clone(&self.modifiers),
                    timestamp,
                    button,
                    self.num_clicks,
                ));
                None
            }
        }
    }
}

impl UiTimedInputEvent {
    pub fn new(
        kinds: UiTimedInputEventKind,
        modifiers: Arc<Modifiers>,
        timestamp: UiEventTimeStampMs,
        button: ButtonKind,
        num_clicks: NumClicks,
    ) -> Self {
        Self {
            kinds,
            modifiers,
            timestamp,
            button,
            num_clicks,
        }
    }
}

// ====

#[derive(Clone, Debug)]
pub enum UiInputEvent {
    Modified(UiModifiedInputEvent),
    Timed(UiTimedInputEvent),
}

// ====

#[test]
fn ui_raw_input_to_input() {
    #[derive(Clone, Debug)]
    struct UiRawInputProcessor {
        state: Option<UiRawInputState>,
        context: UiRawInputSimpleContext,
    }

    #[derive(Clone, Debug)]
    struct UiRawInputSimpleContext {
        time: UiEventTimeStampMs,
        processor: UiTimedInputProcessor,
    }

    impl UiRawInputProcessor {
        fn on_event(&mut self, ev: UiRawInputEvent) {
            // TODO: Remove
            println!("{:?}", ev);

            assert!(self.context.time < ev.timestamp);
            let timestamp = ev.timestamp;
            self.state = Some(self.state.take().unwrap().with_event(ev, &mut self.context));
            self.context.time = timestamp;

            // TODO: Remove
            println!("{:?}", self.state);
            println!("{:?}", self.context.processor.state);
            println!("{:?}", self.context.processor.context.events);
            self.context.processor.context.events.clear();
            println!();
        }
    }

    impl UiRawInputContext for UiRawInputSimpleContext {
        fn emit_event(&mut self, ev: UiModifiedInputEvent) {
            self.processor
                .context
                .events
                .push(UiInputEvent::Modified(ev.clone()));
            self.processor.on_event(ev);
        }
    }

    #[derive(Clone, Debug)]
    struct UiTimedInputProcessor {
        state: Option<UiTimedInputState>,
        context: UiTimedInputSimpleContext,
    }

    #[derive(Clone, Debug)]
    struct UiTimedInputSimpleContext {
        time: UiEventTimeStampMs,
        timeouts: BTreeMap<UiEventTimeStampMs, Weak<ScheduledTimeout>>,
        events: Vec<UiInputEvent>,
    }

    impl UiTimedInputProcessor {
        fn on_event(&mut self, ev: UiModifiedInputEvent) {
            assert!(self.context.time < ev.timestamp);
            while let Some(entry) = self.context.timeouts.first_entry() {
                if *entry.key() > ev.timestamp {
                    break;
                }
                let (timestamp, timeout) = entry.remove_entry();
                if let Some(timeout) = timeout.upgrade() {
                    self.state = Some(self.state.take().unwrap().with_timeout_event(
                        timeout.button.clone(),
                        timestamp,
                        &mut self.context,
                    ))
                }
            }
            let timestamp = ev.timestamp;
            self.state = Some(self.state.take().unwrap().with_event(ev, &mut self.context));
            self.context.time = timestamp;
        }
    }

    impl UiTimedInputContext for UiTimedInputSimpleContext {
        fn schedule(
            &mut self,
            button: ButtonKind,
            delay: UiTimedInputDuration,
        ) -> Arc<ScheduledTimeout> {
            let timeout = Arc::new(ScheduledTimeout { button });
            let delay = match delay {
                UiTimedInputDuration::LongClick(_) => 1000,
                UiTimedInputDuration::MultiClick(_) => 300,
            };
            self.timeouts
                .insert(self.time + delay, Arc::downgrade(&timeout));
            timeout
        }

        fn emit_event(&mut self, ev: UiTimedInputEvent) {
            self.events.push(UiInputEvent::Timed(ev));
        }
    }

    use UiRawInputEvent as Ev;
    use UiRawInputEventKind::*;

    let ctrl = || "ctrl".to_owned();

    let mut processor = UiRawInputProcessor {
        state: Some(UiRawInputState::default()),
        context: UiRawInputSimpleContext {
            time: 0,
            processor: UiTimedInputProcessor {
                state: Some(UiTimedInputState::default()),
                context: UiTimedInputSimpleContext {
                    time: 0,
                    timeouts: BTreeMap::new(),
                    events: vec![],
                },
            },
        },
    };
    processor.on_event(Ev::new(KeyDown { key: ctrl() }, 10000));
    processor.on_event(Ev::new(KeyUp { key: ctrl() }, 10500));
    processor.on_event(Ev::new(KeyDown { key: ctrl() }, 11000));
    processor.on_event(Ev::new(KeyUp { key: ctrl() }, 13000));
    processor.on_event(Ev::new(KeyDown { key: ctrl() }, 11000));
    processor.on_event(Ev::new(MouseDown { button: 0 }, 11100));
    processor.on_event(Ev::new(MouseUp { button: 0 }, 11200));
    processor.on_event(Ev::new(MouseDown { button: 0 }, 11300));
    processor.on_event(Ev::new(MouseUp { button: 0 }, 11400));
    processor.on_event(Ev::new(KeyUp { key: ctrl() }, 13000));

    panic!();
}

/*
    UiTimedInputState: button => ButtonTimedState
    ButtonTimedState: timeout, num_clicks


    UiModifiedInputEvent | UiTimedInputState -> UiTimedInputEvent



    UiRawInputEvent
        KeyUp, MouseMove, TouchStart, etc., KeyRepeat,

    UiModifiedInputEvent
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

    UiTimedInputEvent
        LongPress (modifiers on first press)
        Click (modifiers on first press)
        LongClick (modifiers on first press)

    UiRawInputEvent | UiRawInputState -> UiModifiedInputEvent
    UiModifiedInputEvent | UiTimedInputState -> UiTimedInputEvent
    UiModifiedInputEvent + UiTimedInputEvent -> UiInputEvent
    UiInputEvent | UiInputState -> UiAppEvent
    UiAppEvent | UiAppState -> ...
*/
