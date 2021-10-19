use std::sync::Arc;

use crate::{
    Axis, AxisKind, ButtonKind, EventWithModifiers, Modifiers, MouseScrollDelta, RawEvent,
    RawInput, TimestampMs,
};

pub type ModifiedEvent = EventWithModifiers<ModifiedInput>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ModifiedInput {
    Press(ButtonKind),
    Release(ButtonKind),
    Repeat(ButtonKind),
    Change(Axis),
    Trigger(TriggerKind),
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
pub struct ModifiedState<T: RawContext> {
    modifiers: Arc<Modifiers>,
    context: T,
}

pub struct ModifiedStateUpdater<T: RawContext> {
    modifiers: Modifiers,
    context: T,
    kinds: Vec<ModifiedInput>,
    timestamp: TimestampMs,
}

pub trait RawContext: Sized {
    fn emit_event(self, ev: ModifiedEvent) -> Self;
}

impl RawEvent {
    pub fn new(kind: RawInput, timestamp: TimestampMs) -> Self {
        Self {
            input: kind,
            timestamp,
        }
    }
}

impl<T: RawContext> ModifiedState<T> {
    pub fn new(context: T) -> Self {
        Self {
            modifiers: Arc::default(),
            context,
        }
    }

    pub fn modifiers(&self) -> &Arc<Modifiers> {
        &self.modifiers
    }

    pub fn context(&self) -> &T {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut T {
        &mut self.context
    }

    pub fn make_event(self, timestamp: TimestampMs) -> ModifiedStateUpdater<T> {
        ModifiedStateUpdater::new(self, timestamp)
    }

    pub fn with_event(self, ev: RawEvent) -> Self {
        use RawInput::*;

        let event = self.make_event(ev.timestamp);
        let updater = match ev.input {
            KeyDown(key) => event.with_button_pressed(ButtonKind::KeyboardKey(key)),
            KeyUp(key) => event.with_button_released(ButtonKind::KeyboardKey(key)),
            MouseDown(button) => event.with_button_pressed(ButtonKind::MouseButton(button)),
            MouseUp(button) => event.with_button_released(ButtonKind::MouseButton(button)),
            MouseMove(coords) => event
                .with_axis_changed(Axis::new(AxisKind::MouseX, Some(coords.0)))
                .with_axis_changed(Axis::new(AxisKind::MouseY, Some(coords.1))),
            MouseWheelDown => event.with_trigger(TriggerKind::MouseWheelDown),
            MouseWheelUp => event.with_trigger(TriggerKind::MouseWheelUp),
            MouseScroll(delta) => event.with_trigger(TriggerKind::MouseScroll(delta)),
            TouchStart { touch_id, coords } => event
                .with_button_pressed(ButtonKind::Touch(touch_id))
                .with_axis_changed(Axis::new(AxisKind::TouchX(touch_id), Some(coords.0)))
                .with_axis_changed(Axis::new(AxisKind::TouchY(touch_id), Some(coords.1))),
            TouchMove { touch_id, coords } => event
                .with_axis_changed(Axis::new(AxisKind::TouchX(touch_id), Some(coords.0)))
                .with_axis_changed(Axis::new(AxisKind::TouchY(touch_id), Some(coords.1))),
            TouchEnd { touch_id } => event.with_button_released(ButtonKind::Touch(touch_id)),
            Char(ch) => event.with_trigger(TriggerKind::Char(ch)),
        };
        updater.apply()
    }
}

impl<T: RawContext> ModifiedStateUpdater<T> {
    pub fn new(state: ModifiedState<T>, timestamp: TimestampMs) -> ModifiedStateUpdater<T> {
        Self {
            modifiers: state.modifiers.as_ref().to_owned(),
            context: state.context,
            timestamp,
            kinds: Vec::new(),
        }
    }

    fn with_trigger(mut self, trigger: TriggerKind) -> Self {
        self.kinds.push(ModifiedInput::Trigger(trigger));
        self
    }

    fn with_button_pressed(mut self, button: ButtonKind) -> Self {
        self.kinds.push(ModifiedInput::Press(button.clone()));
        let is_added = self.modifiers.buttons.insert(button);
        assert!(is_added);
        self
    }

    fn with_button_released(mut self, button: ButtonKind) -> Self {
        self.kinds.push(ModifiedInput::Release(button.clone()));
        let is_removed = self.modifiers.buttons.remove(&button);
        assert!(is_removed);
        self
    }

    fn with_axis_changed(mut self, axis: Axis) -> Self {
        self.kinds.push(ModifiedInput::Change(axis.clone()));
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

    pub fn apply(self) -> ModifiedState<T> {
        assert!(!self.kinds.is_empty());
        let modifiers = Arc::new(self.modifiers);
        let mut context = self.context;
        for kind in self.kinds {
            context = context.emit_event(ModifiedEvent {
                input: kind,
                modifiers: Arc::clone(&modifiers),
                timestamp: self.timestamp,
            });
        }
        ModifiedState { modifiers, context }
    }
}
