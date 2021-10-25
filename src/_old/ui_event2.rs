#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum UiEventKind {
    PointerEvent(UiPointerEvent),

    MouseWheelDelta(UiEventMouseWheelDelta),
    TouchStart(UiEventTouchStart),
    TouchEnd(UiEventTouchEnd),
    TouchMove(UiEventTouchMove),
    KeyDown(UiEventKeyDown),
    KeyUp(UiEventKeyUp),
    Char(UiEventChar),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Modifiers {
    KeyboardKey(KeyboardKey),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEvent {
    kind: UiEventKind,
    modifiers: HashSet<Modifiers>,
}

pub enum UiEventKind {
    MouseMoveMaybeStart {
        coords: CoordsPair,
        button: MouseButton,
        is_long: bool,
        clicks: u32,
    },
    MouseMoveStart {
        coords: CoordsPair,
        button: MouseButton,
        is_long: bool,
        clicks: u32,
    },
    MouseMoving {
        coords: CoordsPair,
        button: MouseButton,
        is_long: bool,
        clicks: u32,
    },
    MouseMoveEnd {
        coords: CoordsPair,
        button: MouseButton,
        is_long: bool,
        clicks: u32,
    },
    MouseClick {
        coords: Coords,
        button: MouseButton,
        clicks: u32,
    },
    MouseClickExact {
        coords: Coords,
        button: MouseButton,
        is_long: bool,
        clicks: u32,
    },
    MouseWheel {
        coords: Coords,
        delta: u32,
    },
    TouchMoving {
        coords: Vec<CoordsPair>,
        is_long: bool,
        clicks: u32,
    },
    TouchMoveEnd {
        coords: Vec<CoordsPair>,
        is_long: bool,
        clicks: u32,
    },
    TouchClick {
        coords: Coords,
        is_long: bool,
        clicks: u32,
    },
    TouchClickExact {
        coords: Coords,
        is_long: bool,
        clicks: u32,
    },
    Key {
        key: KeyboardKey,
    },
    Char {
        ch: String,
    },
}
