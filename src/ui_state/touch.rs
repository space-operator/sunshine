use derive_more::From;

#[derive(From, Clone, Debug, Eq, PartialEq)]
pub enum UiTouchState {
    /*Default(UiDefaultTouchState),
Start(UiTouchStartState),
Moving(UiTouchMoveState),
MoveEnd(UiTouchMoveEndState),
Click(UiTouchClickState),
ClickExact(UiTouchClickExactState),*/}

/*

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


#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UiMousePressedState {
    coords: UiEventCoords,
    num_clicks: NumMouseClicks,
}


impl UiMouseStateMachine for UiDefaultMouseState {
    fn with_event<'a, T: Context>(self, data: UiMouseStateWithEventData<'a, T>) -> UiMouseState {
        match data.ev {
            LowLevelUiMouseEvent::Down(ev) => {
                UiMousePressedState::new(data.ctx, ev.coords, 0, data.timestamp).into()
            }
            LowLevelUiMouseEvent::Up(_) => {
                panic!();
            }
            LowLevelUiMouseEvent::Move(ev) => {
                data.ctx.emit_event(UiEvent {
                    timestamp: data.timestamp,
                    modifiers: Arc::clone(data.modifiers),
                    kind: UiEventKind::MouseMove(ev),
                });
                self.into()
            }
        }
    }
}
*/
