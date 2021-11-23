use core::iter::FusedIterator;
use std::sync::Arc;

use crate::{
    Axis, AxisKind, ButtonKind, EventCoord, EventCoords, EventWithModifiers, Modifiers,
    MouseScrollDelta, RawInput, TouchId,
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
pub struct ModifiedState {
    modifiers: Arc<Modifiers>,
}

#[derive(Clone, Debug)]
struct ModifiedTransitionEventsBuilder {
    modifiers: Modifiers,
}

#[derive(Clone, Debug)]
pub struct ModifiedTransitionEvents<T> {
    state: ModifiedTransitionEventsState<T>,
    modifiers: Arc<Modifiers>,
}

#[derive(Clone, Debug)]
enum ModifiedTransitionEventsState<T> {
    Pressed(ButtonKind),
    Released(ButtonKind),
    MouseMoveXY(EventCoords),
    MouseMoveY(EventCoord),
    Trigger(TriggerKind<T>),
    TouchStartXY(TouchId, EventCoords),
    TouchStartY(TouchId, EventCoord),
    TouchMoveXY(TouchId, EventCoords),
    TouchMoveY(TouchId, EventCoord),
    TouchEndXY(TouchId),
    TouchEndY(TouchId),
    Empty,
}

impl ModifiedState {
    pub fn new() -> Self {
        Self {
            modifiers: Arc::default(),
        }
    }

    pub fn modifiers(&self) -> &Arc<Modifiers> {
        &self.modifiers
    }

    pub fn with_event<T>(self, ev: RawInput<T>) -> ModifiedTransitionEvents<T> {
        use RawInput::{
            Char, Custom, KeyDown, KeyUp, MouseDown, MouseMove, MouseScroll, MouseUp,
            MouseWheelDown, MouseWheelUp, TouchEnd, TouchMove, TouchStart,
        };

        let builder = ModifiedTransitionEventsBuilder::new(self.modifiers.as_ref().clone());
        match ev {
            KeyDown(key) => builder.with_pressed(ButtonKind::KeyboardKey(key)),
            KeyUp(key) => builder.with_released(ButtonKind::KeyboardKey(key)),
            MouseDown(button) => builder.with_pressed(ButtonKind::MouseButton(button)),
            MouseUp(button) => builder.with_released(ButtonKind::MouseButton(button)),
            MouseMove(coords) => builder.with_mouse_move(coords),
            MouseWheelDown => builder.with_trigger(TriggerKind::MouseWheelDown),
            MouseWheelUp => builder.with_trigger(TriggerKind::MouseWheelUp),
            MouseScroll(delta) => builder.with_trigger(TriggerKind::MouseScroll(delta)),
            TouchStart { touch_id, coords } => builder.with_touch_start(touch_id, coords),
            TouchMove { touch_id, coords } => builder.with_touch_move(touch_id, coords),
            TouchEnd { touch_id } => builder.with_touch_end(touch_id),
            Char(ch) => builder.with_trigger(TriggerKind::Char(ch)),
            Custom(ev) => builder.with_trigger(TriggerKind::Custom(ev)),
        }
    }
}

impl<T> ModifiedTransitionEvents<T> {
    pub fn to_state(&self) -> ModifiedState {
        ModifiedState {
            modifiers: Arc::clone(&self.modifiers),
        }
    }

    pub fn modifiers(&self) -> &Arc<Modifiers> {
        &self.modifiers
    }
}

impl ModifiedTransitionEventsBuilder {
    fn new(modifiers: Modifiers) -> Self {
        Self { modifiers }
    }

    fn with_pressed<T>(mut self, button: ButtonKind) -> ModifiedTransitionEvents<T> {
        let is_added = self.modifiers.buttons.insert(button.clone());
        assert!(is_added);
        ModifiedTransitionEvents {
            state: ModifiedTransitionEventsState::Pressed(button),
            modifiers: Arc::new(self.modifiers),
        }
    }

    fn with_released<T>(mut self, button: ButtonKind) -> ModifiedTransitionEvents<T> {
        let is_removed = self.modifiers.buttons.remove(&button);
        assert!(is_removed);
        ModifiedTransitionEvents {
            state: ModifiedTransitionEventsState::Released(button),
            modifiers: Arc::new(self.modifiers),
        }
    }

    fn with_trigger<T>(self, trigger: TriggerKind<T>) -> ModifiedTransitionEvents<T> {
        ModifiedTransitionEvents {
            state: ModifiedTransitionEventsState::Trigger(trigger),
            modifiers: Arc::new(self.modifiers),
        }
    }

    fn with_mouse_move<T>(mut self, (x, y): EventCoords) -> ModifiedTransitionEvents<T> {
        let _ = self.modifiers.axes.insert(AxisKind::MouseX, x);
        let _ = self.modifiers.axes.insert(AxisKind::MouseY, y);
        ModifiedTransitionEvents {
            state: ModifiedTransitionEventsState::MouseMoveXY((x, y)),
            modifiers: Arc::new(self.modifiers),
        }
    }

    fn with_touch_start<T>(
        mut self,
        touch_id: TouchId,
        (x, y): EventCoords,
    ) -> ModifiedTransitionEvents<T> {
        let prev = self.modifiers.axes.insert(AxisKind::TouchX(touch_id), x);
        assert!(prev.is_none());
        let prev = self.modifiers.axes.insert(AxisKind::TouchY(touch_id), y);
        assert!(prev.is_none());
        ModifiedTransitionEvents {
            state: ModifiedTransitionEventsState::TouchStartXY(touch_id, (x, y)),
            modifiers: Arc::new(self.modifiers),
        }
    }

    fn with_touch_move<T>(
        mut self,
        touch_id: TouchId,
        (x, y): EventCoords,
    ) -> ModifiedTransitionEvents<T> {
        let prev = self.modifiers.axes.insert(AxisKind::TouchX(touch_id), x);
        assert!(prev.is_some());
        let prev = self.modifiers.axes.insert(AxisKind::TouchY(touch_id), y);
        assert!(prev.is_some());
        ModifiedTransitionEvents {
            state: ModifiedTransitionEventsState::TouchMoveXY(touch_id, (x, y)),
            modifiers: Arc::new(self.modifiers),
        }
    }

    fn with_touch_end<T>(mut self, touch_id: TouchId) -> ModifiedTransitionEvents<T> {
        let prev = self.modifiers.axes.remove(&AxisKind::TouchX(touch_id));
        assert!(prev.is_some());
        let prev = self.modifiers.axes.remove(&AxisKind::TouchY(touch_id));
        assert!(prev.is_some());
        ModifiedTransitionEvents {
            state: ModifiedTransitionEventsState::TouchEndXY(touch_id),
            modifiers: Arc::new(self.modifiers),
        }
    }
}

impl<T> Iterator for ModifiedTransitionEvents<T> {
    type Item = ModifiedEvent<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next().map(|input| ModifiedEvent {
            input,
            modifiers: Arc::clone(&self.modifiers),
        })
    }
}

impl<T> Iterator for ModifiedTransitionEventsState<T> {
    type Item = ModifiedInput<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let axis = |kind, value| Some(ModifiedInput::Change(Axis { kind, value }));

        use ModifiedTransitionEventsState as State;
        let (state, input) = match core::mem::replace(self, State::Empty) {
            State::Pressed(button) => (State::Empty, Some(ModifiedInput::Press(button))),
            State::Released(button) => (State::Empty, Some(ModifiedInput::Release(button))),
            State::Trigger(trigger) => (State::Empty, Some(ModifiedInput::Trigger(trigger))),
            State::MouseMoveXY((x, y)) => (State::MouseMoveY(y), axis(AxisKind::MouseX, Some(x))),
            State::MouseMoveY(y) => (
                State::Trigger(TriggerKind::MouseMove),
                axis(AxisKind::MouseY, Some(y)),
            ),
            State::TouchStartXY(touch_id, (x, y)) => (
                State::TouchStartY(touch_id, y),
                axis(AxisKind::TouchX(touch_id), Some(x)),
            ),
            State::TouchStartY(touch_id, y) => (
                State::Pressed(ButtonKind::Touch(touch_id)),
                axis(AxisKind::TouchY(touch_id), Some(y)),
            ),
            State::TouchMoveXY(touch_id, (x, y)) => (
                State::TouchMoveY(touch_id, y),
                axis(AxisKind::TouchX(touch_id), Some(x)),
            ),
            State::TouchMoveY(touch_id, y) => (
                State::Trigger(TriggerKind::TouchMove),
                axis(AxisKind::TouchY(touch_id), Some(y)),
            ),
            State::TouchEndXY(touch_id) => (
                State::TouchEndY(touch_id),
                axis(AxisKind::TouchX(touch_id), None),
            ),
            State::TouchEndY(touch_id) => (
                State::Released(ButtonKind::Touch(touch_id)),
                axis(AxisKind::TouchY(touch_id), None),
            ),
            State::Empty => (State::Empty, None),
        };
        *self = state;
        input
    }
}

impl<T> FusedIterator for ModifiedTransitionEvents<T> {}
impl<T> FusedIterator for ModifiedTransitionEventsState<T> {}

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
