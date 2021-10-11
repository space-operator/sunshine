//mod context;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
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
    MouseButton(MouseButton),
    KeyboardKey(KeyboardKey),
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

#[derive(Clone, Debug)]
pub struct Modifiers {
    buttons: HashSet<ButtonKind>,
    axes: HashMap<AxisKind, AxisValue>,
}

#[derive(Clone, Debug)]
pub struct UiRawInputState {
    modifiers: Arc<Modifiers>,
}

pub struct UiRawInputStateUpdater<T: UiRawInputContext> {
    ctx: T,
    kinds: Vec<UiModifiedInputEventKind>,
    modifiers: Modifiers,
    timestamp: UiEventTimeStampMs,
}

pub trait UiRawInputContext {
    fn emit_event(&mut self, ev: UiModifiedInputEvent);
}

impl<T: UiRawInputContext> UiRawInputStateUpdater<T> {
    pub fn new(
        state: UiRawInputState,
        ctx: T,
        timestamp: UiEventTimeStampMs,
    ) -> UiRawInputStateUpdater<T> {
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
    pub fn make_event<T: UiRawInputContext>(
        self,
        ctx: T,
        timestamp: UiEventTimeStampMs,
    ) -> UiRawInputStateUpdater<T> {
        UiRawInputStateUpdater::new(self, ctx, timestamp)
    }

    pub fn with_event<T: UiRawInputContext>(self, ev: UiRawInputEvent, mut ctx: T) -> Self {
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

// ====

pub type NumClicks = u32;
pub type ScheduledTimeout = Arc<UiEventTimeStampMs>;

#[derive(Clone, Debug)]
pub struct UiTimedInputState {
    buttons: HashMap<ButtonKind, ButtonTimedState>,
}

#[derive(Clone, Debug)]
pub enum ButtonTimedState {
    Pressed { timeout: ScheduledTimeout },
    LongPressed {},
    Released { timeout: ScheduledTimeout },
    LongReleased { timeout: ScheduledTimeout },
}

/*pub struct UiTimedInputStateUpdater<T: UiTimedInputContext> {
    ctx: T,
    kinds: Vec<UiModifiedInputEventKind>,
    modifiers: Modifiers,
    timestamp: UiEventTimeStampMs,
}*/

pub trait UiTimedInputContext {
    fn schedule_after_long_click(&mut self) -> ScheduledTimeout;
    fn schedule_after_multi_click(&mut self) -> ScheduledTimeout;
    fn emit_event(&mut self, ev: UiModifiedInputEvent);
}

#[derive(Clone, Debug)]
pub struct UiTimedInputEvent {
    pub kinds: UiTimedInputEventKind,
    pub modifiers: Arc<Modifiers>,
    pub timestamp: UiEventTimeStampMs,
}

#[derive(Clone, Debug, PartialEq)]
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
}

impl UiTimedInputState {
    pub fn with_event<T: UiTimedInputContext>(
        mut self,
        ev: UiModifiedInputEvent,
        mut ctx: T,
    ) -> Self {
        let modifiers = ev.modifiers;
        let timestamp = ev.timestamp;
        for kind in ev.kinds {
            self = self.with_event_kind(kind, &modifiers, timestamp, &mut ctx);
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
                let entry = self.buttons.entry(button);
                entry
                    .and_modify(|state| state.on_press_event(ctx))
                    .or_insert_with(|| {
                        let state = ButtonTimedState::from_pressed(modifiers, timestamp, ctx);
                        state
                    });
            }
            Release(button) => todo!(),
            Repeat(_) => {}
            Change(_) => {}
            Trigger(_) => {}
        }
        todo!()
    }
}

impl ButtonTimedState {
    fn from_pressed<T: UiTimedInputContext>(
        modifiers: &Arc<Modifiers>,
        timestamp: UiEventTimeStampMs,
        ctx: &mut T,
    ) -> Self {
        todo!()
    }

    fn on_press_event<T: UiTimedInputContext>(&mut self, ctx: &mut T) {
        todo!()
    }
}

/*
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
