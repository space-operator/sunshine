//mod context;

use context::*;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

pub type UiEventTimeStampMs = u64;
pub type UiEventCoords = (i32, i32);
pub type MouseButton = u32;
pub type KeyboardKey = String;
pub type AxisValue = f32;
pub type MouseScrollDelta = i32;
pub type TouchId = u32;

#[derive(Clone, Debug, Eq, From, PartialEq)]
pub struct UiRawInputEvent {
    pub kind: UiRawInputEventKind,
    pub timestamp: UiEventTimeStampMs,
}

#[derive(Clone, Debug, Eq, From, PartialEq)]
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
    MouseWheelDown {
        coords: UiEventCoords,
    },
    MouseWheelUp {
        coords: UiEventCoords,
    },
    MouseScroll {
        coords: UiEventCoords,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ButtonKind {
    MouseButton(MouseButton),
    KeyboardKey(KeyboardKey),
    Touch(TouchId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AxisKind {
    MouseX,
    MouseY,
    TouchX(TouchId),
    TouchY(TouchId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Axis {
    kind: AxisKind,
    value: AxisValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TriggerKind {
    MouseWheelUp,
    MouseWheelDown,
    MouseScroll(MouseScrollDelta),
    Char(String),
    CharRepeat(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiModifiedInputEvent {
    pub kind: UiModifiedInputEvent,
    pub modifiers: Arc<Modifiers>,
    pub timestamp: UiEventTimeStampMs,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiModifiedInputEventKind {
    Press(ButtonKind),
    Repeat(ButtonKind),
    Release(ButtonKind),
    Change(Axis),
    Trigger(TriggerKind),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Modifiers {
    buttons: HashSet<ButtonKind>,
    axes: HashMap<AxisKind, AxisValue>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiRawInputState {
    modifiers: Arc<Modifiers>,
}

pub trait UiRawInputContext {
    fn emit_event(&mut self, ev: UiModifiedInputEvent);
}

impl UiRawInputState {
    fn with_event<T: UiRawInputContext>(self, ev: UiRawInputEventKind, ctx: T) -> Self {
        match ev.kind {
            KeyDown { key } => self.with_button_pressed(ButtonKind::KeyboardKey(key), ev.timestamp),
            KeyUp { key } => self.with_button_released(ButtonKind::KeyboardKey(key), ev.timestamp),
            MouseDown { button } => {
                self.with_button_pressed(ButtonKind::MouseButton(button), ev.timestamp)
            }
            MouseUp { button } => {
                self.with_button_released(ButtonKind::MouseButton(button), ev.timestamp)
            }
            MouseMove { coords } => {
                self.with_axis_change(Axis {
                    kind: MouseX
                    value: coords.0
                }, ev.timestamp).with_axis_change(Axis {
                    kind: MouseY
                    value: coords.1
                }, ev.timestamp)
            } // TODO: Handle mouse and touch x and y movement simultaneously
            MouseWheelDown { coords } => todo!(), // with_trigger
            MouseWheelUp { coords } => todo!(), // with_trigger
            MouseScroll { coords, delta } => todo!(), // with_trigger
            TouchStart { touch_id, coords } => {
                self.with_button_pressed(ButtonKind::Touch(touch_id), ev.timestamp).with_axis_change(Axis {
                    kind: TouchX
                    value: coords.0
                }, ev.timestamp).with_axis_change(Axis {
                    kind: TouchY
                    value: coords.1
                }, ev.timestamp)
            } // TODO: Handle mouse and touch x and y movement simultaneously
            TouchMove { touch_id, coords } => todo!(),
            TouchEnd { touch_id } => todo!(),
            Char { ch } => todo!(), // with_trigger
        }
    }

    fn with_trigger<T: UiRawInputContext>(
        self,
        trigger: TriggerKind,
        timestamp: UiEventTimeStampMs,
        ctx: T,
    ) -> Self {
        ctx.emit_event(UiModifiedInputEvent {
            kind: UiModifiedInputEventKind::TriggerKind(trigger),
            modifiers: Arc::clone(&self.modifiers),
            timestamp,
        });
        self
    }

    fn with_button_pressed<T: UiRawInputContext>(
        self,
        button: ButtonKind,
        timestamp: UiEventTimeStampMs,
        ctx: T,
    ) -> Self {
        ctx.emit_event(UiModifiedInputEvent {
            kind: UiModifiedInputEventKind::Press(button),
            modifiers: Arc::clone(&self.modifiers),
            timestamp,
        });
        let is_added = self.modifiers.make_mut().buttons.insert(button);
        assert!(is_added);
        self
    }

    fn with_button_released<T: UiRawInputContext>(
        self,
        button: ButtonKind,
        timestamp: UiEventTimeStampMs,
        ctx: T,
    ) -> Self {
        ctx.emit_event(UiModifiedInputEvent {
            kind: UiModifiedInputEventKind::Release(button),
            modifiers: Arc::clone(&self.modifiers),
            timestamp,
        });
        let is_removed = self.modifiers.make_mut().buttons.remove(button);
        assert!(is_removed);
        self
    }

    fn with_axis_changed(self, axis: Axis, timestamp: UiEventTimeStampMs, ctx: T) -> Self {
        ctx.emit_event(UiModifiedInputEvent {
            kind: UiModifiedInputEventKind::Change(axis),
            modifiers: Arc::clone(&self.modifiers),
            timestamp,
        });
        let _ = self.modifiers.make_mut().axes.insert(axis.kind, axis.value);
        self
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

    UiInputEvent
        LongPress (modifiers on first press)
        Click (modifiers on first press)
        LongClick (modifiers on first press)
        DblClick (modifiers on first press)

    UiRawInputEvent -> UiRawInputState -> UiModifiedInputEvent       UiInputState
                                                  v                      ^
                                            UiModifiedInputState -> UiInputEvent
z
    UiRawInputEvent
        UiRawInputState
    UiModifiedInputEvent
        UiModifiedInputState    +timeout
    UiInputEvent
        UiInputState
    UiAppEvent
        UiAppState
*/
