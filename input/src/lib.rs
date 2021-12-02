#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    non_ascii_idents,
    pointer_structural_match,
    rust_2018_idioms,
    rust_2021_compatibility,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![allow(clippy::module_name_repetitions)]

//mod axis;
//mod button;
//mod combined;
mod event;
//mod keyboard;
//mod mapping;
mod event_with_modifiers;
//mod mouse;
//mod processor;
mod aggregate_timed_event;
//mod raw;
mod timed_event;
//mod touch;

//pub use axis::*;
//pub use button::*;
//pub use combined::*;
pub use event::*;
//pub use keyboard::*;
//pub use mapping::*;
pub use event_with_modifiers::*;
//pub use mouse::*;
//pub use processor::*;
pub use aggregate_timed_event::*;
//pub use raw::*;
pub use timed_event::*;
//pub use touch::*;

/*
dbl-click
    create node at position
    enter to text mode



==================================
=              C --------D                  =
=             |
=           Schools----------B
=              |
                A

0010
    spans: Schools
    children:
        0021 (A)
        0022 (B)
        0023 (C)


node: schools
    edge: child -> A
    edge: child -> B
    edge: child -> C

node: C
    edge: link -> D



[dbl-click]Schools[enter]A[enter]B[enter][enter]
Schools
    A
    B
Playground

[dbl-click]Schools[enter][enter]


Root
    Schools
        A B []

███████████████████████████████
█ |                           █
███████████████████████████████

███████████████████████████████
█ Schools                     █
███████████████████████████████



███████████████████████████████
█ Schools                %$#  █ graph
█   A                         █
█   B                         █ ----A
█   C                         █
███████████████████████████████


█████████████████████
█                   █
█  A            V   █ --> Q
█      B            █
█          [C]>     █ ~~~~~~~~ C typting <
█              D -- █---> E
█                   █
█████████████████████



██████████████████████████
█     Schools      %$#  █
        [this] [E] is ....
██████████████████████████
  |      |      |      |
█████  █████  █████  █████
█ A █  █ this █  █ C █  █ E █
█████  █████  █████  █████

Root
    PlaygroundA
        [C]
    C


Block
                                Block
                                    A

            Block

███████████████████████████████
█      Schools                █
█                             █
█  ███████                    █
█  █  A  █                    █
█  ███████                    █
█                             █
█                             █
█                             █
█                             █
█                             █
███████████████████████████████


███████████████████████████████       ██████████████████████████████████████████████████████████████
█ -    PlaygroundA      %$#   █       █      PlaygroundB     %$#    ██      PlaygroundC      %$#   █
█        [A] [{!Map}]         █       █        A                    ██        A                    █
█        [C]                  █       █        C                    ██        C                    █
█          C1                 █       █        C                    ██        C                    █
█        D              view  █       █        D                    ██        D                    █
█                             █       █                             ██                             █
███████████████████████████████       ██████████████████████████████████████████████████████████████
  |         |                            |      |      |      |        |      |      |      |
█████    ███████                      █████  █████  █████  █████     █████  █████  █████  █████
█ A █    █ Map █                        █ E █  █ F █  █ G █  █ H █     █ E █  █ F █  █ G █  █ H █
█████    ███████                      █████  █████  █████  █████     █████  █████  █████  █████



███████████████████████████████
█      Playground             █
███████████████████████████████
  |      |      |      |
█████  █████  █████  █████
█ A █  █ B █  █ C █  █ D █
█████  █████  █████  █████


Text View
███████████████████████████████
█      Schools                █
█                             █
██ ████████████████  ███████  █            █████     █████
█  █  A  █  █  Map█  █ [С] █  █ ---------> █[C]█     █ D █
██ ███████  ███████  ███████  █            █████     █████
█                             █
█                             █
█                             █
█                             █
█                             █
███████████████████████████████



███████████████████████████████
█      Playground             █
█                             █
█                             █
█                             █
███████████████████████████████

Draft:
    Sublocks by default is indented text
    Each block can be toggled to floating mode
    Floated block no longer part of parent block text
    Block moved outside replaced to link to this block and
        block parent changed to block we move this block into
    We somehow can move inner block outside only visually with using other kind of edge


TODO:
    Input contexts

mapping draft:
    Undo
    Redo

    CreateNode(coords, parent_node)
    CreateNodeAndEditNoCoords(parent_node)
    CreateNodeAndEdit(coords, parent_node)
    RemoveNodes(nodes)

    SelectNodes(nodes)
    AddNodesToSelector(nodes)
    RemoveNodesToSelector(nodes)

    StartEdge(source_node)
    ContinueEdge(source_node, current_coords, ?target_node)
    ConnectEdge(source_node, target_node)
    StopEdge(source_node)
    RemoveEdges(edges)

    // use touch coords mean
    StartNodeMove(nodes, start_coords)
    ContinueNodeMove(nodes, start_coors, current_coords);
    StopNodeMove(nodes, start_coors, end_coords);

    // use touch coords mean
    StartScreenPan(start_coords)
    ContinueScreenPan(start_coors, current_coords)
    StopScreenPan(start_coors, end_coords)

    // use touch deltas mult or scroll
    StartScreenZoom(start_mult)
    ContinueScreenZoom(start_mult, current_mult)
    StopScreenZoom(start_mult, current_end)

    StartNodeEdit

    NodeEdit
        Stop
        Text(text)
        Backspace
        Delete
        NewChild
        Tab -> "\t"
        Indent -> goto child    ? how to override other action (Tab) with contexts or without
        Untab                   ? but what if we do not use tab for it
        StartTextSelect()

    context override

    HaveNode
        Ctrl+Click  NodeSelect
    NoNode
        Click       UnselectNodes


    raw | ?? => modifier | ?? => timed | ?? => mapping


    Mouse+coords
    Touch+touch


    MoveMove(100, 200), Ctrl+Lmb(modifiers) =>
        say to Input we have Ctrl+Click for NodeSelect and Click for UnselectNodes

    => NodeSelect



NodeEdit + NonEmptyChildText
NodeEdit

Schools
    A\t
    \t






*/

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


    Refactor
        Replace: ev >> Context::process(impl in input) >> Context::emit(impl in app)
            with: ev >> Context::process >> returns array with dynamic size

    TODOs
        Generic KeyboardKey (not only the string but <T>)
        Mapping sequential events like Adown Aup Bdown Bup
        MouseScroll X and Y
        ProcessorModifiedContext::result?
        Better test for processor

    Done
        Make TimestampMs genetic, just removed
        ButtonTimedStateWithContext
        Decide tuple with data and err or Error with data
        Fix unwrap in processor


    event override
        override event with short modifiers (Ctrl+Lmb)
        by an event with longer modifiers (Ctrl+Shift+Lmb)
    event override rejection
        do no override shorter one by longer one
        if longer one is disabled by AppState
        therefore can not be handled.


*/
