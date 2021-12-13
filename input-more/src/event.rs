use core::hash::Hash;

use input_core::TimedCombinedEventData;

pub trait Event {
    type Switch;
}

pub trait ToActionOrTrigger<'a>: Event {
    type SwitchEvent: SwitchEvent<Switch = Self::Switch>;
    type TriggerEvent;

    fn to_action_or_trigger(&'a self) -> ActionOrTrigger<Self::SwitchEvent, Self::TriggerEvent>;
}

#[allow(single_use_lifetimes)] // false positive
pub trait SwitchEvent {
    type Switch: Eq + Hash + Ord;

    fn switch(&self) -> Self::Switch;
}

pub enum ActionOrTrigger<SwEv, TrEv> {
    Action(Action<SwEv>),
    Trigger(TrEv),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action<T> {
    Press(T),
    Release(T),
}

enum EventOrTimedEvent<Ev, Ti> {
    Event(Ev),
    TimedEvent(Ti),
}

struct TimedEvent<SwEv> {
    event: SwEv,
    timed: TimedCombinedEventData,
}

/*
(x, y) of down or up
each (x, y) <> (x, y)
if
    iter switches
        filter only related
            reset clicks
    emit movement
else
    -

(sys_id, x, y) of down or up
on down or up
    match our_id with sys_id
on move
    get our_id from sys_id map
too much movement
    iter switches
        filter only related
            reset

*/

// ====

/*
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Switch {
    Keyboard(&'static str),
    Mouse(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum SwitchEvent {
    Keyboard(&'static str, TimestampMs),
    Mouse(&'static str, Coords, TimestampMs),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum TriggerEvent {
    MouseMove(Coords, TimestampMs),
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RawEvent {
    KeyboardDown(&'static str, TimestampMs),
    KeyboardUp(&'static str, TimestampMs),
    MouseDown(&'static str, Coords, TimestampMs),
    MouseUp(&'static str, Coords, TimestampMs),
    MouseMove(Coords, TimestampMs),
}




#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RawEvent<KeSw, KeDa, MoSw, MoSwDa, MoTr, MoTrDa, Ti> {
    KeyboardSwitch(Action<KeSw>, KeDa, Ti),
    //
    MouseSwitch(Action<MoSw>, MoSwDa, Ti),
    MouseTrigger(MoTr, MoTrDa, Ti),
}


raw -> timed -> raw | timed

MouseDown -> MouseTrigger
MouseUp -> MouseTrigger
MouseScroll -> None

enum
    Raw(MouseDown, MouseUp)
    Timed(Mouse, data)

pub trait Preprocess {
    fn preprocess()
}

pub trait MouseEvent {
    fn coords()
}

pub trait TouchEvent {
    fn touch_id_and_coords()
}

impl Preprocess for MouseEvent {

}


pub trait EventKind<T> {
    fn to_event_kind(&self) -> Option<T>;
}

pub trait AnotherEvent {
    type Data;

    fn data(&self) -> Self::Data;
}*/

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

/*
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
}*/

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
