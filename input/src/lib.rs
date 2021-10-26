#![feature(map_first_last)]

mod axis;
mod button;
mod combined;
mod event;
mod keyboard;
mod mapped;
mod modified;
mod modifiers;
mod mouse;
mod processor;
mod raw;
mod timed;
mod touch;

pub use axis::*;
pub use button::*;
pub use combined::*;
pub use event::*;
pub use keyboard::*;
pub use mapped::*;
pub use modified::*;
pub use modifiers::*;
pub use mouse::*;
pub use processor::*;
pub use raw::*;
pub use timed::*;
pub use touch::*;

/*

    TimedState: button => ButtonTimedState
    ButtonTimedState: timeout, num_clicks


    ModifiedEvent | TimedState -> TimedEvent



    RawEvent
        KeyUp, MouseMove, TouchStart, etc., KeyRepeat,

    ModifiedEvent
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

    TimedEvent
        LongPress (modifiers on first press)
        Click (modifiers on first press)
        LongClick (modifiers on first press)


    Event
        kind
        timestampt EventWithModifiers
        kind
        timestamp
        modifiers

    RawEvent | ModifiedState + ModifiedContext -> ModifiedEvent
    ModifiedEvent | TimedState + TimedContext -> TimedEvent
    ModifiedEvent + TimedEvent -> CombinedEvent
    CombinedEvent | MappedContext -> AppEvent
    AppEvent | AppState -> ...


    TODOs
        + ButtonTimedStateWithContext
        + Decide tuple with data and err or Error with data
        + Fix unwrap in processor

    Later
        KeyboardKey::Other deserialization
        Better test for processor
        Add more keys
        ProcessorModifiedContext::result?


    event override
        override event with short modifiers (Ctrl+Lmb)
        by an event with longer modifiers (Ctrl+Shift+Lmb)
    event override rejection
        do no override shorter one by longer one
        if longer one is disabled by AppState
        therefore can not be handled.


*/
