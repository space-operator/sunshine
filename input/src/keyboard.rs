use serde::{Deserialize, Serialize};

// TODO: Add more keys
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum KeyboardKey {
    // first row
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    // second row
    Backquote,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Digit0,
    Minus,
    Equal,
    Backspace,
    // third row
    Tab,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    Bracket,
    Backslash,
    // fourth row
    CapsLock,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    Semicolon,
    Quote,
    #[serde(alias = "Result")]
    Enter,
    // fifth row
    #[serde(alias = "ShiftLeft")]
    LeftShift,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Period,
    Slash,
    #[serde(alias = "ShiftRight")]
    RightShift,
    // sixth row
    #[serde(alias = "ControlLeft")]
    LeftCtrl,
    #[serde(alias = "OSLeft")]
    LeftOs,
    #[serde(alias = "AltLeft")]
    LeftAlt,
    Space,
    #[serde(alias = "AltRight")]
    RightAlt,
    #[serde(alias = "OSRight")]
    RightOs,
    #[serde(alias = "ControlRight")]
    ContextMenu,
    RightCtrl,
    // middle section
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    ArrowUp,
    ArrowLeft,
    ArrowDown,
    ArrowRight,
    // numpad
    NumLock,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubstract,
    Numpad9,
    Numpad8,
    Numpad7,
    NumpadAdd,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad3,
    Numpad2,
    Numpad1,
    NumpadEnter,
    Numpad0,
    NumpadDecimal,
    // Other
    #[serde(other)]
    Other, // TODO: Other(String)
}

#[test]
fn keyboardkey_deser_test() {
    let test = |value: &str, expected| {
        let value: KeyboardKey = serde_json::from_str(&format!("\"{}\"", value)).unwrap();
        assert_eq!(value, expected);
    };

    test("Space", KeyboardKey::Space);
    test("ControlLeft", KeyboardKey::LeftCtrl);
    test("VolumeUp", KeyboardKey::Other);
}
