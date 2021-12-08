#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action<T> {
    Press(T),
    Release(T),
}

pub trait Event {
    type Switch: Eq + Hash + Ord;
    type MouseCoord;
    type TouchCoord;
    type TouchId;
    type Timestamp;

    fn switch(&self) -> Option<Action<Self::Switch>>;
    fn device(&self) -> Device<Self::Coord, Self::TouchId>;
    fn timestamp(&self) -> Timestamp;
}

pub enum Device<MouseCoord, TouchId, TouchCoord> {
    Keyboard,
    Mouse((MouseCoord, MouseCoord)),
    Touch(TouchId, (TouchCoord, TouchCoord)),
}

pub trait ToEventKind<T> {
    fn to_event_kind(&self) -> Option<T>;
}

pub trait AnotherEvent {
    type Data;

    fn data(&self) -> Self::Data;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RawEvent {
    KeyboardDown(&'static str, TimestampMs),
    KeyboardUp(&'static str, TimestampMs),
    MouseDown(&'static str, Coords, TimestampMs),
    MouseUp(&'static str, Coords, TimestampMs),
    MouseMove(Coords, TimestampMs),
}

impl ToEventKind<MouseEvent> {}

// ====

/*
    KeyboardEvent
        Press   key
        Release key
        Trigger
            char
    MouseEvent +coords
        Press   button
        Release button
        Trigger
            move
            wheelup
            wheeldown
            scroll(delta)
    TouchEvent +touch-id +coords
        Press   ?
        Release ?
        Trigger move

    multitouch-state

        Mouse((x, y))
        Mouse(x)
        Mouse(y)

        Event::MutliTouch {
            + take context in account, so we have at the same time
                touch on on-screen ctrl button
                double touch on canvas
                single touch on zoom slider
            num_touhes: 2,
            kind: CenterXy | CenterX | CenterY | Scale | Rotation,
            value: (x, y) | x
        }
            center, scale, rotation

        shift+
*/

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Event<Ke, Mo, To> {
    Keyboard(Ke),
    Mouse(Mo),
    Touch(To),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeyboardEvent<Sw, Tr> {
    Press(Sw),
    Release(Sw),
    Trigger(Tr),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MouseEvent<Sw, Tr> {
    Press(Sw),
    Release(Sw),
    Trigger(Tr),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TouchEvent<Sw, Tr> {
    Press(Sw),
    Release(Sw),
    Trigger(Tr),
}

/*
pub trait Coords {
    fn
}

enum Device {
    Keyboard()
    Mouse(Coords)
    Touch()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RawEvent {
    KeyboardDown(&'static str, TimestampMs),
    KeyboardUp(&'static str, TimestampMs),
    MouseDown(&'static str, Coords, TimestampMs),
    MouseUp(&'static str, Coords, TimestampMs),
    MouseMove(Coords, TimestampMs),
}
*/
