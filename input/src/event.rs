use std::collections::HashSet;
use std::sync::Arc;

/*
    ((raw event | time-event) + modifiers) => mapped event

    trait Event
    trait State
        fn apply(State, Event) -> (State, Event)
*/
/*
    MouseMove(x, y), MouseButtonDown(button, x, y) >

    Event

*/

pub trait EventWithAction {
    type Switch;

    fn action(&self) -> Option<Action<Self::Switch>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action<T> {
    Enable(T),
    Disable(T),
}

/*
where
    T1: EventWithAction<Switch = T2> + EventWithTimestamp<Timestamp = T3>, */
/* where T1: EventWithAction<Switch = T2>,*/
pub struct EventWithModifiers<T1, T2> {
    event: T1,
    modifiers: Arc<HashSet<T2>>,
}

#[test]
fn test() {
    enum RawEvent {
        KeyboardDown(&'static str, u64),
        KeyboardUp(&'static str, u64),
        MouseDown(&'static str, (u64, u64), u64),
        MouseUp(&'static str, (u64, u64), u64),
        MouseMove((u64, u64), u64),
    }

    enum RawEventSwitch {
        Key(&'static str),
        Button(&'static str),
    }

    impl EventWithAction for RawEvent {
        type Switch = RawEventSwitch;

        fn action(&self) -> Option<Action<Self::Switch>> {
            match self {
                RawEvent::KeyboardDown(s, t) => Some(Action::Enable(RawEventSwitch::Key(s))),
                RawEvent::KeyboardUp(s, t) => Some(Action::Disable(RawEventSwitch::Key(s))),
                RawEvent::MouseDown(s, c, t) => Some(Action::Enable(RawEventSwitch::Button(s))),
                RawEvent::MouseUp(s, c, t) => Some(Action::Disable(RawEventSwitch::Button(s))),
                RawEvent::MouseMove(c, t) => None,
            }
        }
    }
}

/*
#[test]
fn test() {
    enum RawEvent {
        CtrlDown(u64),
        CtrlUp(u64),
        MouseMove(u64, u64, u64),
        LeftMouseDown(u64, u64, u64),
        LeftMouseUp(u64, u64, u64),
    };

    enum RawKeyboardEvent {
        CtrlDown,
        CtrlUp,
    }

    enum RawKeyboardButton {
        Ctrl,
    }

    enum RawKeyboardTrigger {}

    enum RawMouseEvent {
        MouseMove(u64, u64),
        LeftMouseDown(u64, u64),
        LeftMouseUp(u64, u64),
    }

    enum RawTouchEvent {}

    impl ToEvent for RawEvent {
        type Keyboard = RawKeyboardEvent;
        type Mouse = RawMouseEvent;
        type Touch = RawTouchEvent;

        fn to_event(&self) -> Event<Self::Keyboard, Self::Mouse, Self::Touch> {
            match self {
                RawEvent::CtrlDown(_) => Event::Keyboard(RawKeyboardEvent::CtrlDown),
                RawEvent::CtrlUp(_) => Event::Keyboard(RawKeyboardEvent::CtrlUp),
                RawEvent::MouseMove(x, y, _) => Event::Mouse(RawMouseEvent::MouseMove(*x, *y)),
                RawEvent::LeftMouseDown(x, y, _) => {
                    Event::Mouse(RawMouseEvent::LeftMouseDown(*x, *y))
                }
                RawEvent::LeftMouseUp(x, y, _) => Event::Mouse(RawMouseEvent::LeftMouseUp(*x, *y)),
            }
        }
    }

    impl ToEventKind for RawKeyboardEvent {
        type Switch = RawKeyboardButton;
        type Trigger = RawKeyboardTrigger;

        fn to_event_kind(&self) -> EventKind<Self::Switch, Self::Trigger> {
            match self {
                RawKeyboardEvent::CtrlDown => EventKind::Press(RawKeyboardButton::Ctrl),
                RawKeyboardEvent::CtrlUp => EventKind::Release(RawKeyboardButton::Ctrl),
            }
        }
    }
}*/

/*
Event<EventKind<T1, T2>, EventKind<T3, T4>, EventKind<T5, T6>>
    timed
Event<EventKind<T1, TriggerOrTimedTrigger<T2, T1>>, ...
    modifiers
Event<EventKindWithModifiers<T1, T2>
    mapping
AppEvent
*/

/*
pub trait ToEvent {
    type Keyboard;
    type Mouse;
    type Touch;

    fn to_event(&self) -> Event<Self::Keyboard, Self::Mouse, Self::Touch>;
}

pub trait ToTimestamp {
    type Output;

    fn to_timestamp(&self) -> Self::Output;
}

pub trait ToEventKind {
    type Switch;
    type Trigger;

    fn to_event_kind(&self) -> EventKind<Self::Switch, Self::Trigger>;
}
*/

/*
impl< T1: ToEventKind + IntoEventKind<Switch = T2>, T2> for KeyboardEvent<T1, T2> {
    // ...
}
*/

/*
    Event<T1, T2, T3>
    >> timed-processor
    >> Event<Timed<T1>, Timed<T2>, Timed<T3>>
    >> modifiers-processor
    >> Event<Modified<Timed<T1>>, Modified<Timed<T2>>, Modified<Timed<T3>>>
    >> (handle them in program) >> mapping-processor
    >> app events
*/

/*
trait KeyboardModifiers {
    type Button;

    pub fn keyboard_buttons(&self) -> &HashSet<Self::Button>;
}

trait MouseModifiers {
    type State;
    type Button;

    pub fn mouse_state(&self) -> &Self::State;
}
*/
