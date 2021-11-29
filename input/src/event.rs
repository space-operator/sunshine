use std::sync::Arc;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event<T1, T2, T3> {
    Keyboard(T1),
    Mouse(T2),
    Touch(T3),
}

trait ToEventKind {
    type Switch;
    type Trigger;

    pub fn to_event_kind(&self) -> EventKind<Switch, Trigger>;
}

trait IntoEventKind {
    type Switch;
    type Trigger;

    pub fn into_event_kind(self) -> EventKind<Switch, Trigger>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventKind<T1, T2> {
    Press(T1),
    Release(T1),
    Trigger(T2),
}

struct WithModifiers<T>(T, HashMap, HashSet);

trait KeyboardModifiers {
    type Switch;

    pub fn keyboard_switches(&self) -> &HashSet<Self::Switch>;
}

#[derive(Clone, Debug, Default)]
pub struct KeyboardEvent<T1, T2> {
    raw: T1,
    keyboard_switches: Arc<HashSet<T2>,
}

#[derive(Clone, Debug, Default)]
pub struct MouseEvent<T1, T2> {
    raw: T1,
    keyboard_switches: Arc<HashSet<T2>>,
    mouse_switches: Arc<HashSet<T3>>,
    mouse_values: Arc<T4>,
}

impl< T1: ToEventKind + IntoEventKind<Switch = T2>, T2> for KeyboardEvent<T1, T2> {
    // ...
}

#[derive(Clone, Debug, Default)]
pub struct ModifiersState<T11, T21, T22, T31, T32> {
    keyboard: KeyboardState<T11>,
    mouse: MouseState<T21, T22>,
    touch: TouchState<T31, T32>,
}

#[derive(Clone, Debug, Default)]
pub struct KeyboardState<T1> {
    switches: HashSet<T1>,
}

#[derive(Clone, Debug, Default)]
pub struct MouseState<T1, T2> {
    switches: HashSet<T1>,
    values: T2,
}

#[derive(Clone, Debug, Default)]
pub struct TouchState<T1, T2> {
    switches: HashSet<T1>,
    values: T2,
}

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
