//! The platform-neutral key event vocabulary.
//!
//! Backends convert their native event representations into these types;
//! no backend-specific types may appear here. On Linux, the evdev
//! conversion lives in `platform::linux`; other backends will bring their
//! own conversions.

use std::fmt::{self, Display, Formatter};
use std::time::SystemTime;

macro_rules! define_keys {
    ($( $variant:ident => $display:expr ),* $(,)?) => {
        /// A key or mouse button.
        ///
        /// The named variants cover a full standard keyboard layout
        /// (letters, digits, punctuation, modifiers, navigation, function
        /// keys, numpad) plus common media, power, brightness, browser and
        /// editing keys, the main international keys, and mouse buttons.
        /// Anything else falls back to [`Key::Other`], which carries the
        /// platform's raw key code so no information is lost (see the
        /// backend module docs for which code space a platform uses).
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Key {
            $(
                $variant,
            )*
            /// A key without a named variant; carries the platform's raw key code.
            Other(u16),
        }

        impl Key {
            pub fn get_all_keys() -> Vec<Key> {
                vec![
                    $(
                        Key::$variant,
                    )*
                ]
            }
        }

        impl Display for Key {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        Key::$variant => write!(f, $display),
                    )*
                    Key::Other(code) => write!(f, "Other(0x{code:02x})"),
                }
            }
        }
    };
}

define_keys! {
    // Main keys (US layout)
    Escape => "Escape",
    Digit1 => "1",
    Digit2 => "2",
    Digit3 => "3",
    Digit4 => "4",
    Digit5 => "5",
    Digit6 => "6",
    Digit7 => "7",
    Digit8 => "8",
    Digit9 => "9",
    Digit0 => "0",
    Minus => "-",
    Equal => "=",
    Backspace => "Backspace",
    Tab => "Tab",
    LeftBracket => "[",
    RightBracket => "]",
    Enter => "Enter",
    A => "A",
    B => "B",
    C => "C",
    D => "D",
    E => "E",
    F => "F",
    G => "G",
    H => "H",
    I => "I",
    J => "J",
    K => "K",
    L => "L",
    M => "M",
    N => "N",
    O => "O",
    P => "P",
    Q => "Q",
    R => "R",
    S => "S",
    T => "T",
    U => "U",
    V => "V",
    W => "W",
    X => "X",
    Y => "Y",
    Z => "Z",
    Semicolon => ";",
    Apostrophe => "'",
    Grave => "`",
    Backslash => "\\",
    Comma => ",",
    Dot => ".",
    Slash => "/",
    IntlBackslash => "IntlBackslash",
    Space => "Space",
    // Modifiers and locks
    LeftCtrl => "LeftCtrl",
    RightCtrl => "RightCtrl",
    LeftShift => "LeftShift",
    RightShift => "RightShift",
    LeftAlt => "LeftAlt",
    RightAlt => "RightAlt",
    LeftMeta => "LeftMeta",
    RightMeta => "RightMeta",
    Compose => "Compose",
    Fn => "Fn",
    Menu => "Menu",
    CapsLock => "CapsLock",
    NumLock => "NumLock",
    ScrollLock => "ScrollLock",
    // Navigation
    Insert => "Insert",
    Delete => "Delete",
    Home => "Home",
    End => "End",
    PageUp => "PageUp",
    PageDown => "PageDown",
    Up => "Up",
    Down => "Down",
    Left => "Left",
    Right => "Right",
    PrintScreen => "PrintScreen",
    Pause => "Pause",
    // Function keys
    F1 => "F1",
    F2 => "F2",
    F3 => "F3",
    F4 => "F4",
    F5 => "F5",
    F6 => "F6",
    F7 => "F7",
    F8 => "F8",
    F9 => "F9",
    F10 => "F10",
    F11 => "F11",
    F12 => "F12",
    F13 => "F13",
    F14 => "F14",
    F15 => "F15",
    F16 => "F16",
    F17 => "F17",
    F18 => "F18",
    F19 => "F19",
    F20 => "F20",
    F21 => "F21",
    F22 => "F22",
    F23 => "F23",
    F24 => "F24",
    // Numpad
    Numpad0 => "Numpad0",
    Numpad1 => "Numpad1",
    Numpad2 => "Numpad2",
    Numpad3 => "Numpad3",
    Numpad4 => "Numpad4",
    Numpad5 => "Numpad5",
    Numpad6 => "Numpad6",
    Numpad7 => "Numpad7",
    Numpad8 => "Numpad8",
    Numpad9 => "Numpad9",
    NumpadEnter => "NumpadEnter",
    NumpadPlus => "+",
    NumpadMinus => "-",
    NumpadAsterisk => "*",
    NumpadSlash => "/",
    NumpadDot => "NumpadDot",
    NumpadEqual => "=",
    NumpadComma => ",",
    KpJpComma => "KpJpComma",
    // Media keys
    VolumeUp => "VolumeUp",
    VolumeDown => "VolumeDown",
    Mute => "Mute",
    MicMute => "MicMute",
    PlayPause => "PlayPause",
    Play => "Play",
    Stop => "Stop",
    NextTrack => "NextTrack",
    PreviousTrack => "PreviousTrack",
    Rewind => "Rewind",
    FastForward => "FastForward",
    Record => "Record",
    Eject => "Eject",
    MediaSelect => "MediaSelect",
    // Power, brightness and illumination
    Power => "Power",
    Sleep => "Sleep",
    WakeUp => "WakeUp",
    BrightnessUp => "BrightnessUp",
    BrightnessDown => "BrightnessDown",
    KbdIllumUp => "KbdIllumUp",
    KbdIllumDown => "KbdIllumDown",
    KbdIllumToggle => "KbdIllumToggle",
    // Browser and application keys
    Email => "Email",
    Calculator => "Calculator",
    Computer => "Computer",
    Search => "Search",
    BrowserHome => "BrowserHome",
    BrowserBack => "BrowserBack",
    BrowserForward => "BrowserForward",
    Refresh => "Refresh",
    Bookmarks => "Bookmarks",
    Bluetooth => "Bluetooth",
    Wlan => "Wlan",
    TouchpadToggle => "TouchpadToggle",
    TouchpadOn => "TouchpadOn",
    TouchpadOff => "TouchpadOff",
    // Editing keys
    Copy => "Copy",
    Cut => "Cut",
    Paste => "Paste",
    Undo => "Undo",
    Redo => "Redo",
    Again => "Again",
    New => "New",
    Open => "Open",
    Find => "Find",
    Help => "Help",
    // International keys
    Ro => "Ro",
    Yen => "Yen",
    Hangeul => "Hangeul",
    Hanja => "Hanja",
    Hiragana => "Hiragana",
    Katakana => "Katakana",
    KatakanaHiragana => "KatakanaHiragana",
    Henkan => "Henkan",
    Muhenkan => "Muhenkan",
    ZenkakuHankaku => "ZenkakuHankaku",
    // Mouse buttons
    MouseLeft => "MouseLeft",
    MouseRight => "MouseRight",
    MouseMiddle => "MouseMiddle",
    MouseSide => "MouseSide",
    MouseExtra => "MouseExtra",
    MouseForward => "MouseForward",
    MouseBack => "MouseBack",
}

/// The physical state of a key.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyState {
    Down,
    Up,
    Hold,
}

impl KeyState {
    /// Interpret a numeric key state value.
    ///
    /// `0` = up, `1` = down, `2` = hold (auto-repeat). Returns `None` for
    /// any other value.
    pub fn from_value(value: i32) -> Option<Self> {
        match value {
            0 => Some(KeyState::Up),
            1 => Some(KeyState::Down),
            2 => Some(KeyState::Hold),
            _ => None,
        }
    }

    /// The numeric value this state represents.
    ///
    /// The inverse of [`KeyState::from_value`]: `Up` = 0, `Down` = 1,
    /// `Hold` = 2.
    pub fn to_value(self) -> i32 {
        match self {
            KeyState::Up => 0,
            KeyState::Down => 1,
            KeyState::Hold => 2,
        }
    }
}

impl Display for KeyState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            KeyState::Down => write!(f, "Down"),
            KeyState::Up => write!(f, "Up"),
            KeyState::Hold => write!(f, "Hold"),
        }
    }
}

/// Best-effort identity of an input device.
///
/// `name` is the human-readable device name and `location` a
/// platform-specific string that (usually) distinguishes physical devices
/// (on Linux, the evdev physical path, falling back to the device node).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId {
    pub name: String,
    pub location: String,
}

impl Display for DeviceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.location)
    }
}

/// A single input event: which key or mouse button, what happened to it,
/// on which device, and when.
#[derive(Debug, Clone, PartialEq)]
pub struct InputKeyEvent {
    pub key: Key,
    pub state: KeyState,
    pub device: DeviceId,
    pub timestamp: SystemTime,
}

impl Display for InputKeyEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InputKeyEvent {{ key: {}, state: {}, device: {}, at: {:?} }}",
            self.key, self.state, self.device, self.timestamp
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_state_from_value() {
        assert_eq!(KeyState::from_value(0), Some(KeyState::Up));
        assert_eq!(KeyState::from_value(1), Some(KeyState::Down));
        assert_eq!(KeyState::from_value(2), Some(KeyState::Hold));
        assert_eq!(KeyState::from_value(3), None);
    }

    #[test]
    fn key_state_to_value() {
        assert_eq!(KeyState::Up.to_value(), 0);
        assert_eq!(KeyState::Down.to_value(), 1);
        assert_eq!(KeyState::Hold.to_value(), 2);
    }

    #[test]
    fn key_state_value_roundtrip() {
        for value in 0..=2 {
            assert_eq!(KeyState::from_value(value).unwrap().to_value(), value);
        }
    }

    #[test]
    fn key_display() {
        assert_eq!(Key::Digit1.to_string(), "1");
        assert_eq!(Key::Minus.to_string(), "-");
        assert_eq!(Key::Backslash.to_string(), "\\");
        assert_eq!(Key::NumpadEnter.to_string(), "NumpadEnter");
        assert_eq!(Key::Other(0x70).to_string(), "Other(0x70)");
    }

    #[test]
    fn device_id_equality() {
        let a = DeviceId {
            name: "kbd".into(),
            location: "/dev/input/event0".into(),
        };
        let b = DeviceId {
            name: "kbd".into(),
            location: "/dev/input/event0".into(),
        };
        let c = DeviceId {
            name: "kbd".into(),
            location: "/dev/input/event1".into(),
        };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
