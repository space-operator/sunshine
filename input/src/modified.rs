use std::sync::Arc;

use crate::{
    Axis, AxisKind, ButtonKind, EventWithModifiers, Modifiers, MouseScrollDelta, RawInput,
};

pub type ModifiedEvent<T> = EventWithModifiers<ModifiedInput<T>>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ModifiedInput<T> {
    Press(ButtonKind),
    Release(ButtonKind),
    Repeat(ButtonKind),
    Change(Axis),
    Trigger(TriggerKind<T>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TriggerKind<T> {
    MouseWheelUp,
    MouseWheelDown,
    MouseScroll(MouseScrollDelta),
    Char(String),
    CharRepeat(String),
    MouseMove,
    TouchMove,
    Custom(T),
}

#[derive(Clone, Debug)]
pub struct ModifiedState<T: ModifiedContext> {
    modifiers: Arc<Modifiers>,
    context: T,
}

struct ModifiedStateUpdater<T: ModifiedContext> {
    modifiers: Modifiers,
    context: T,
    kinds: Vec<ModifiedInput<T::CustomEvent>>,
}

pub trait ModifiedContext: Sized {
    type CustomEvent;
    fn emit_event(self, ev: ModifiedEvent<Self::CustomEvent>) -> Self;
}

impl<T> ModifiedEvent<T> {
    pub fn clone_without_custom(&self) -> ModifiedEvent<()> {
        ModifiedEvent {
            input: self.input.clone_without_custom(),
            modifiers: self.modifiers.clone(),
        }
    }
}

impl<T> ModifiedInput<T> {
    pub fn clone_without_custom(&self) -> ModifiedInput<()> {
        match self {
            Self::Press(kind) => ModifiedInput::Press(kind.clone()),
            Self::Release(kind) => ModifiedInput::Release(kind.clone()),
            Self::Repeat(kind) => ModifiedInput::Repeat(kind.clone()),
            Self::Change(axis) => ModifiedInput::Change(axis.clone()),
            Self::Trigger(kind) => ModifiedInput::Trigger(kind.clone_without_custom()),
        }
    }
}

impl<T> TriggerKind<T> {
    pub fn clone_without_custom(&self) -> TriggerKind<()> {
        match self {
            Self::MouseWheelUp => TriggerKind::MouseWheelUp,
            Self::MouseWheelDown => TriggerKind::MouseWheelDown,
            Self::MouseScroll(scroll) => TriggerKind::MouseScroll(*scroll),
            Self::Char(ch) => TriggerKind::Char(ch.clone()),
            Self::CharRepeat(ch) => TriggerKind::CharRepeat(ch.clone()),
            Self::MouseMove => TriggerKind::MouseMove,
            Self::TouchMove => TriggerKind::TouchMove,
            Self::Custom(_) => TriggerKind::Custom(()),
        }
    }
}

impl<T: ModifiedContext> ModifiedState<T> {
    pub fn new(context: T) -> Self {
        Self {
            modifiers: Arc::default(),
            context,
        }
    }

    pub fn from_parts(modifiers: Arc<Modifiers>, context: T) -> Self {
        Self { modifiers, context }
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

    pub fn split(self) -> (Arc<Modifiers>, T) {
        (self.modifiers, self.context)
    }

    fn make_event(self) -> ModifiedStateUpdater<T> {
        ModifiedStateUpdater::new(self)
    }

    pub fn with_event(self, ev: RawInput<T::CustomEvent>) -> Self {
        use RawInput::{
            Char, Custom, KeyDown, KeyUp, MouseDown, MouseMove, MouseScroll, MouseUp,
            MouseWheelDown, MouseWheelUp, TouchEnd, TouchMove, TouchStart,
        };

        let event = self.make_event();
        let updater = match ev {
            KeyDown(key) => event.with_button_pressed(ButtonKind::KeyboardKey(key)),
            KeyUp(key) => event.with_button_released(ButtonKind::KeyboardKey(key)),
            MouseDown(button) => event.with_button_pressed(ButtonKind::MouseButton(button)),
            MouseUp(button) => event.with_button_released(ButtonKind::MouseButton(button)),
            MouseMove(coords) => event
                .with_axis_changed(Axis::new(AxisKind::MouseX, Some(coords.0)))
                .with_axis_changed(Axis::new(AxisKind::MouseY, Some(coords.1)))
                .with_trigger(TriggerKind::MouseMove),
            MouseWheelDown => event.with_trigger(TriggerKind::MouseWheelDown),
            MouseWheelUp => event.with_trigger(TriggerKind::MouseWheelUp),
            MouseScroll(delta) => event.with_trigger(TriggerKind::MouseScroll(delta)),
            TouchStart { touch_id, coords } => event
                .with_axis_changed(Axis::new(AxisKind::TouchX(touch_id), Some(coords.0)))
                .with_axis_changed(Axis::new(AxisKind::TouchY(touch_id), Some(coords.1)))
                .with_button_pressed(ButtonKind::Touch(touch_id)),
            TouchMove { touch_id, coords } => event
                .with_axis_changed(Axis::new(AxisKind::TouchX(touch_id), Some(coords.0)))
                .with_axis_changed(Axis::new(AxisKind::TouchY(touch_id), Some(coords.1)))
                .with_trigger(TriggerKind::TouchMove),
            TouchEnd { touch_id } => event
                .with_axis_changed(Axis::new(AxisKind::TouchX(touch_id), None))
                .with_axis_changed(Axis::new(AxisKind::TouchY(touch_id), None))
                .with_button_released(ButtonKind::Touch(touch_id)),
            Char(ch) => event.with_trigger(TriggerKind::Char(ch)),
            Custom(ev) => event.with_trigger(TriggerKind::Custom(ev)),
        };
        updater.apply()
    }
}

impl<T: ModifiedContext> ModifiedStateUpdater<T> {
    pub fn new(state: ModifiedState<T>) -> Self {
        Self {
            modifiers: state.modifiers.as_ref().clone(),
            context: state.context,
            kinds: Vec::new(),
        }
    }

    fn with_trigger(mut self, trigger: TriggerKind<T::CustomEvent>) -> Self {
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
        let is_removed = self.modifiers.buttons.remove(&button);
        assert!(is_removed);
        self.kinds.push(ModifiedInput::Release(button));
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
            });
        }
        ModifiedState { modifiers, context }
    }
}
