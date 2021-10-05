use std::sync::Arc;

use derive_more::From;

use crate::ui_event::*;
use crate::ui_state::context::*;

pub struct UiMouseStateWithEventData<'a, T: Context> {
    pub ctx: &'a mut T,
    pub ev: LowLevelUiMouseEvent,
    pub timestamp: UiEventTimeStampMs,
    pub modifiers: &'a Arc<UiModifiers>,
}

pub struct UiMouseStateWithTimeoutData<'a, T: Context> {
    pub ctx: &'a mut T,
    pub timestamp: UiEventTimeStampMs,
    pub modifiers: &'a Arc<UiModifiers>,
}

pub trait UiMouseStateMachine: Sized {
    fn with_timeout<'a, T: Context>(
        self,
        data: UiMouseStateWithTimeoutData<'a, T>,
    ) -> UiMouseState {
        panic!("state should not be called by timeout");
    }

    fn with_event<'a, T: Context>(self, data: UiMouseStateWithEventData<'a, T>) -> UiMouseState;
}

#[derive(From, Clone, Debug, Eq, PartialEq)]
pub enum UiMouseState {
    Default(UiMouseDefaultState),
    Pressed(UiMousePressedState),
    Click(UiMouseClickState),
    ClickExact(UiMouseClickExactState),
    MoveMaybeStart(UiMouseMoveMaybeStartState),
    MoveStart(UiMouseMoveStartState),
    Moving(UiMouseMovingState),
    MoveEnd(UiMouseMoveEndState),
}

pub enum LowLevelUiMouseEvent {
    Down(UiMouseDownEvent),
    Up(UiMouseUpEvent),
    Move(UiMouseMoveEvent),
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UiMouseDefaultState;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UiMousePressedState {
    coords: UiEventCoords,
    num_clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UiMouseClickState {
    coords: UiEventCoords,
    num_clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UiMouseClickExactState {
    coords: UiEventCoords,
    num_clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UiMouseMoveMaybeStartState {
    coords: UiEventCoords,
    num_clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UiMouseMoveStartState {
    coords: UiEventCoords,
    num_clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UiMouseMovingState {
    coords: UiEventCoords,
    num_clicks: NumMouseClicks,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UiMouseMoveEndState {
    coords: UiEventCoords,
    num_clicks: NumMouseClicks,
}

impl UiMouseStateMachine for UiMouseDefaultState {
    fn with_event<'a, T: Context>(self, data: UiMouseStateWithEventData<'a, T>) -> UiMouseState {
        match data.ev {
            LowLevelUiMouseEvent::Down(ev) => {
                UiMousePressedState::new(data.ctx, ev.coords, 0, data.timestamp).into()
            }
            LowLevelUiMouseEvent::Up(_) => {
                panic!();
            }
            LowLevelUiMouseEvent::Move(ev) => {
                data.ctx.emit_event(UiEvent::new(
                    data.timestamp,
                    Arc::clone(data.modifiers),
                    UiEventKind::MouseMove(ev),
                ));
                self.into()
            }
        }
    }
}

impl UiMousePressedState {
    fn new<T: Context>(
        ctx: &mut T,
        coords: UiEventCoords,
        num_clicks: NumMouseClicks,
        timestamp: UiEventTimeStampMs,
    ) -> Self {
        Self { coords, num_clicks }
    }
}

impl UiMouseStateMachine for UiMousePressedState {
    fn with_event<'a, T: Context>(self, data: UiMouseStateWithEventData<'a, T>) -> UiMouseState {
        match data.ev {
            LowLevelUiMouseEvent::Down(_) => {
                panic!();
            }
            LowLevelUiMouseEvent::Up(ev) => {
                data.ctx.emit_event(UiEvent::new(
                    data.timestamp,
                    Arc::clone(data.modifiers),
                    UiEventKind::MouseClick(ev),
                ));
                UiMouseClickState::new(data.ctx, ev.coords, self.num_clicks + 1, data.timestamp)
                    .into()
            }
            LowLevelUiMouseEvent::Move(ev) => {
                todo!();
            }
        }
    }
}

impl UiMouseClickState {
    fn new<T: Context>(
        ctx: &mut T,
        coords: UiEventCoords,
        num_clicks: NumMouseClicks,
        timestamp: UiEventTimeStampMs,
    ) -> Self {
        ctx.schedule_timeout(timestamp + ctx.max_click_time());
        Self { coords, num_clicks }
    }
}

impl UiMouseStateMachine for UiMouseClickState {
    fn with_timeout<'a, T: Context>(
        self,
        data: UiMouseStateWithTimeoutData<'a, T>,
    ) -> UiMouseState {
        data.ctx.emit_event(UiEvent::new(
            data.timestamp,
            Arc::clone(data.modifiers),
            UiEventKind::MouseClickExact {
                coords: UiEventCoords,
                button: MouseButton,
                clicks: self.num_clicks,
            },
        ));
        UiMouseDefaultState.into()
    }

    fn with_event<'a, T: Context>(self, data: UiMouseStateWithEventData<'a, T>) -> UiMouseState {
        match data.ev {
            LowLevelUiMouseEvent::Down(ev) => {
                UiMousePressedState::new(data.ctx, ev.coords, self.num_clicks, data.timestamp)
                    .into()
            }
            LowLevelUiMouseEvent::Up(_) => {
                panic!();
            }
            LowLevelUiMouseEvent::Move(ev) => {
                data.ctx.emit_event(UiEvent::new(
                    data.timestamp,
                    Arc::clone(data.modifiers),
                    UiEventKind::MouseMove(ev),
                ));
                self.into()
            }
        }
    }
}

impl UiMouseClickExactState {
    fn new<T: Context>(
        ctx: &mut T,
        coords: UiEventCoords,
        num_clicks: NumMouseClicks,
        timestamp: UiEventTimeStampMs,
    ) -> Self {
        ctx.schedule_timeout(timestamp + ctx.max_click_time());
        Self { coords, num_clicks }
    }
}

impl UiMouseStateMachine for UiMouseClickExactState {
    fn with_event<'a, T: Context>(self, data: UiMouseStateWithEventData<'a, T>) -> UiMouseState {
        match data.ev {
            LowLevelUiMouseEvent::Down(ev) => {
                todo!();
            }
            LowLevelUiMouseEvent::Up(ev) => {
                todo!();
            }
            LowLevelUiMouseEvent::Move(ev) => {
                todo!();
            }
        }
    }
}

impl UiMouseMoveMaybeStartState {
    fn new<T: Context>(
        ctx: &mut T,
        coords: UiEventCoords,
        num_clicks: NumMouseClicks,
        timestamp: UiEventTimeStampMs,
    ) -> Self {
        ctx.schedule_timeout(timestamp + ctx.max_click_time());
        Self { coords, num_clicks }
    }
}

impl UiMouseStateMachine for UiMouseMoveMaybeStartState {
    fn with_event<'a, T: Context>(self, data: UiMouseStateWithEventData<'a, T>) -> UiMouseState {
        match data.ev {
            LowLevelUiMouseEvent::Down(ev) => {
                todo!();
            }
            LowLevelUiMouseEvent::Up(ev) => {
                todo!();
            }
            LowLevelUiMouseEvent::Move(ev) => {
                todo!();
            }
        }
    }
}

impl UiMouseMoveStartState {
    fn new<T: Context>(
        ctx: &mut T,
        coords: UiEventCoords,
        num_clicks: NumMouseClicks,
        timestamp: UiEventTimeStampMs,
    ) -> Self {
        ctx.schedule_timeout(timestamp + ctx.max_click_time());
        Self { coords, num_clicks }
    }
}

impl UiMouseStateMachine for UiMouseMoveStartState {
    fn with_event<'a, T: Context>(self, data: UiMouseStateWithEventData<'a, T>) -> UiMouseState {
        match data.ev {
            LowLevelUiMouseEvent::Down(ev) => {
                todo!();
            }
            LowLevelUiMouseEvent::Up(ev) => {
                todo!();
            }
            LowLevelUiMouseEvent::Move(ev) => {
                todo!();
            }
        }
    }
}

impl UiMouseMovingState {
    fn new<T: Context>(
        ctx: &mut T,
        coords: UiEventCoords,
        num_clicks: NumMouseClicks,
        timestamp: UiEventTimeStampMs,
    ) -> Self {
        ctx.schedule_timeout(timestamp + ctx.max_click_time());
        Self { coords, num_clicks }
    }
}

impl UiMouseStateMachine for UiMouseMovingState {
    fn with_event<'a, T: Context>(self, data: UiMouseStateWithEventData<'a, T>) -> UiMouseState {
        match data.ev {
            LowLevelUiMouseEvent::Down(ev) => {
                todo!();
            }
            LowLevelUiMouseEvent::Up(ev) => {
                todo!();
            }
            LowLevelUiMouseEvent::Move(ev) => {
                todo!();
            }
        }
    }
}

impl UiMouseMoveEndState {
    fn new<T: Context>(
        ctx: &mut T,
        coords: UiEventCoords,
        num_clicks: NumMouseClicks,
        timestamp: UiEventTimeStampMs,
    ) -> Self {
        ctx.schedule_timeout(timestamp + ctx.max_click_time());
        Self { coords, num_clicks }
    }
}

impl UiMouseStateMachine for UiMouseMoveEndState {
    fn with_event<'a, T: Context>(self, data: UiMouseStateWithEventData<'a, T>) -> UiMouseState {
        match data.ev {
            LowLevelUiMouseEvent::Down(ev) => {
                todo!();
            }
            LowLevelUiMouseEvent::Up(ev) => {
                todo!();
            }
            LowLevelUiMouseEvent::Move(ev) => {
                todo!();
            }
        }
    }
}
