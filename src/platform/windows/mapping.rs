//! Conversion between Windows virtual-key codes and the platform-neutral
//! [`crate::keys`] vocabulary, plus the `SendInput` targets used to inject
//! events.
//!
//! This is where Windows-specific knowledge lives; nothing outside
//! `platform::windows` may touch `windows_sys` types.
//!
//! [`Key::Other`] codes are Windows virtual-key codes (`VK_*`,
//! `0x01`-`0xFF`).
//!
//! # Coverage
//!
//! Windows virtual-key codes cover most of the same physical keys as
//! evdev. Keys with no virtual-key equivalent (Compose, Fn, Power,
//! brightness/illumination, bluetooth/WLAN, most application/editing
//! keys, Ro/Yen, mouse buttons 6/7, ...) cannot be sent and arrive as
//! [`Key::Other`] with their code preserved when received. A few IME
//! codes (`VK_PACKET`, `VK_PROCESSKEY`) are synthetic and are filtered
//! out when listening.

use windows_sys::Win32::UI::Input::KeyboardAndMouse as kbd;

use crate::Result;
use crate::keys::Key;

/// Map a virtual-key code (and whether the physical key is in the
/// extended section of the keyboard) to a [`Key`].
///
/// Returns `None` for synthetic codes that never represent a physical
/// key; every other code maps to a named variant or falls back to
/// [`Key::Other`] with the raw code preserved.
pub(super) fn map_vk(vk: u16, extended: bool) -> Option<Key> {
    Some(match vk {
        // Synthetic IME events, not physical keys.
        kbd::VK_PACKET | kbd::VK_PROCESSKEY => return None,

        // Main keys (US layout)
        kbd::VK_ESCAPE => Key::Escape,
        kbd::VK_1 => Key::Digit1,
        kbd::VK_2 => Key::Digit2,
        kbd::VK_3 => Key::Digit3,
        kbd::VK_4 => Key::Digit4,
        kbd::VK_5 => Key::Digit5,
        kbd::VK_6 => Key::Digit6,
        kbd::VK_7 => Key::Digit7,
        kbd::VK_8 => Key::Digit8,
        kbd::VK_9 => Key::Digit9,
        kbd::VK_0 => Key::Digit0,
        kbd::VK_OEM_MINUS => Key::Minus,
        kbd::VK_OEM_PLUS => Key::Equal,
        kbd::VK_BACK => Key::Backspace,
        kbd::VK_TAB => Key::Tab,
        kbd::VK_Q => Key::Q,
        kbd::VK_W => Key::W,
        kbd::VK_E => Key::E,
        kbd::VK_R => Key::R,
        kbd::VK_T => Key::T,
        kbd::VK_Y => Key::Y,
        kbd::VK_U => Key::U,
        kbd::VK_I => Key::I,
        kbd::VK_O => Key::O,
        kbd::VK_P => Key::P,
        kbd::VK_OEM_4 => Key::LeftBracket,
        kbd::VK_OEM_6 => Key::RightBracket,
        kbd::VK_RETURN => Key::Enter,
        kbd::VK_A => Key::A,
        kbd::VK_B => Key::B,
        kbd::VK_C => Key::C,
        kbd::VK_D => Key::D,
        kbd::VK_F => Key::F,
        kbd::VK_G => Key::G,
        kbd::VK_H => Key::H,
        kbd::VK_J => Key::J,
        kbd::VK_K => Key::K,
        kbd::VK_L => Key::L,
        kbd::VK_OEM_1 => Key::Semicolon,
        kbd::VK_OEM_7 => Key::Apostrophe,
        kbd::VK_OEM_3 => Key::Grave,
        kbd::VK_OEM_5 => Key::Backslash,
        kbd::VK_Z => Key::Z,
        kbd::VK_X => Key::X,
        kbd::VK_V => Key::V,
        kbd::VK_N => Key::N,
        kbd::VK_M => Key::M,
        kbd::VK_OEM_COMMA => Key::Comma,
        kbd::VK_OEM_PERIOD => Key::Dot,
        kbd::VK_OEM_2 => Key::Slash,
        kbd::VK_OEM_102 => Key::IntlBackslash,
        kbd::VK_SPACE => Key::Space,
        // Modifiers and locks. The extended flag distinguishes left from
        // right for the generic SHIFT/CONTROL/MENU codes; many keyboards
        // report the specific VK_L* / VK_R* codes directly.
        kbd::VK_LSHIFT => Key::LeftShift,
        kbd::VK_RSHIFT => Key::RightShift,
        kbd::VK_SHIFT if extended => Key::RightShift,
        kbd::VK_SHIFT => Key::LeftShift,
        kbd::VK_LCONTROL => Key::LeftCtrl,
        kbd::VK_RCONTROL => Key::RightCtrl,
        kbd::VK_CONTROL if extended => Key::RightCtrl,
        kbd::VK_CONTROL => Key::LeftCtrl,
        kbd::VK_LMENU => Key::LeftAlt,
        kbd::VK_RMENU => Key::RightAlt,
        kbd::VK_MENU if extended => Key::RightAlt,
        kbd::VK_MENU => Key::LeftAlt,
        kbd::VK_LWIN => Key::LeftMeta,
        kbd::VK_RWIN => Key::RightMeta,
        kbd::VK_APPS => Key::Menu,
        kbd::VK_CAPITAL => Key::CapsLock,
        kbd::VK_NUMLOCK => Key::NumLock,
        kbd::VK_SCROLL => Key::ScrollLock,
        // Navigation
        kbd::VK_INSERT => Key::Insert,
        kbd::VK_DELETE => Key::Delete,
        kbd::VK_HOME => Key::Home,
        kbd::VK_END => Key::End,
        kbd::VK_PRIOR => Key::PageUp,
        kbd::VK_NEXT => Key::PageDown,
        kbd::VK_UP => Key::Up,
        kbd::VK_DOWN => Key::Down,
        kbd::VK_LEFT => Key::Left,
        kbd::VK_RIGHT => Key::Right,
        kbd::VK_SNAPSHOT => Key::PrintScreen,
        kbd::VK_PAUSE => Key::Pause,
        // Function keys
        kbd::VK_F1 => Key::F1,
        kbd::VK_F2 => Key::F2,
        kbd::VK_F3 => Key::F3,
        kbd::VK_F4 => Key::F4,
        kbd::VK_F5 => Key::F5,
        kbd::VK_F6 => Key::F6,
        kbd::VK_F7 => Key::F7,
        kbd::VK_F8 => Key::F8,
        kbd::VK_F9 => Key::F9,
        kbd::VK_F10 => Key::F10,
        kbd::VK_F11 => Key::F11,
        kbd::VK_F12 => Key::F12,
        kbd::VK_F13 => Key::F13,
        kbd::VK_F14 => Key::F14,
        kbd::VK_F15 => Key::F15,
        kbd::VK_F16 => Key::F16,
        kbd::VK_F17 => Key::F17,
        kbd::VK_F18 => Key::F18,
        kbd::VK_F19 => Key::F19,
        kbd::VK_F20 => Key::F20,
        kbd::VK_F21 => Key::F21,
        kbd::VK_F22 => Key::F22,
        kbd::VK_F23 => Key::F23,
        kbd::VK_F24 => Key::F24,
        // Numpad
        kbd::VK_NUMPAD0 => Key::Numpad0,
        kbd::VK_NUMPAD1 => Key::Numpad1,
        kbd::VK_NUMPAD2 => Key::Numpad2,
        kbd::VK_NUMPAD3 => Key::Numpad3,
        kbd::VK_NUMPAD4 => Key::Numpad4,
        kbd::VK_NUMPAD5 => Key::Numpad5,
        kbd::VK_NUMPAD6 => Key::Numpad6,
        kbd::VK_NUMPAD7 => Key::Numpad7,
        kbd::VK_NUMPAD8 => Key::Numpad8,
        kbd::VK_NUMPAD9 => Key::Numpad9,
        kbd::VK_MULTIPLY => Key::NumpadAsterisk,
        kbd::VK_ADD => Key::NumpadPlus,
        kbd::VK_SEPARATOR => Key::NumpadComma,
        kbd::VK_SUBTRACT => Key::NumpadMinus,
        kbd::VK_DECIMAL => Key::NumpadDot,
        kbd::VK_DIVIDE => Key::NumpadSlash,
        // `VK_CLEAR` is the numpad 5 key with NumLock off.
        kbd::VK_CLEAR => Key::Numpad5,
        // Media keys
        kbd::VK_VOLUME_UP => Key::VolumeUp,
        kbd::VK_VOLUME_DOWN => Key::VolumeDown,
        kbd::VK_VOLUME_MUTE => Key::Mute,
        kbd::VK_MEDIA_PLAY_PAUSE => Key::PlayPause,
        kbd::VK_PLAY => Key::Play,
        kbd::VK_MEDIA_STOP => Key::Stop,
        kbd::VK_MEDIA_NEXT_TRACK => Key::NextTrack,
        kbd::VK_MEDIA_PREV_TRACK => Key::PreviousTrack,
        kbd::VK_LAUNCH_MEDIA_SELECT => Key::MediaSelect,
        // Power and sleep
        kbd::VK_SLEEP => Key::Sleep,
        // Browser and application keys
        kbd::VK_LAUNCH_MAIL => Key::Email,
        kbd::VK_LAUNCH_APP2 => Key::Calculator,
        kbd::VK_LAUNCH_APP1 => Key::Computer,
        kbd::VK_BROWSER_SEARCH => Key::Search,
        kbd::VK_BROWSER_HOME => Key::BrowserHome,
        kbd::VK_BROWSER_BACK => Key::BrowserBack,
        kbd::VK_BROWSER_FORWARD => Key::BrowserForward,
        kbd::VK_BROWSER_REFRESH => Key::Refresh,
        kbd::VK_BROWSER_FAVORITES => Key::Bookmarks,
        // Editing keys
        kbd::VK_OEM_COPY => Key::Copy,
        kbd::VK_HELP | kbd::VK_ICO_HELP => Key::Help,
        // International keys. VK_KANA / VK_HANGUL share code 0x15 (the
        // physical key toggles the IME mode), and VK_HANJA / VK_KANJI
        // share 0x19; match on the numeric values to avoid duplicate
        // patterns.
        0x15 => Key::Hiragana,
        0x19 => Key::Hanja,
        kbd::VK_CONVERT => Key::Henkan,
        kbd::VK_NONCONVERT => Key::Muhenkan,

        other => Key::Other(other),
    })
}

/// A mouse button, injected with a `MOUSEINPUT` event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MouseButton {
    Left,
    Right,
    Middle,
    Side,
    Extra,
}

/// How a [`Key`] is injected with [`SendInput`](windows_sys::Win32::UI::Input::KeyboardAndMouse::SendInput).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum InjectTarget {
    /// A keyboard event with this virtual-key code.
    Vk(u16),
    /// A mouse-button event.
    Mouse(MouseButton),
}

/// Map a [`Key`] to its injection target.
///
/// Unlike listening, sending cannot invent a virtual-key code for keys
/// Windows has no equivalent for; those return
/// [`crate::Error::KeyNotMapped`]. [`Key::Other`] passes its code
/// straight through, since on Windows that code *is* a virtual-key code.
pub(super) fn map_key(key: Key) -> Result<InjectTarget> {
    Ok(match key {
        Key::Other(vk) => InjectTarget::Vk(vk),

        // Main keys (US layout)
        Key::Escape => InjectTarget::Vk(kbd::VK_ESCAPE),
        Key::Digit1 => InjectTarget::Vk(kbd::VK_1),
        Key::Digit2 => InjectTarget::Vk(kbd::VK_2),
        Key::Digit3 => InjectTarget::Vk(kbd::VK_3),
        Key::Digit4 => InjectTarget::Vk(kbd::VK_4),
        Key::Digit5 => InjectTarget::Vk(kbd::VK_5),
        Key::Digit6 => InjectTarget::Vk(kbd::VK_6),
        Key::Digit7 => InjectTarget::Vk(kbd::VK_7),
        Key::Digit8 => InjectTarget::Vk(kbd::VK_8),
        Key::Digit9 => InjectTarget::Vk(kbd::VK_9),
        Key::Digit0 => InjectTarget::Vk(kbd::VK_0),
        Key::Minus => InjectTarget::Vk(kbd::VK_OEM_MINUS),
        Key::Equal => InjectTarget::Vk(kbd::VK_OEM_PLUS),
        Key::Backspace => InjectTarget::Vk(kbd::VK_BACK),
        Key::Tab => InjectTarget::Vk(kbd::VK_TAB),
        Key::Q => InjectTarget::Vk(kbd::VK_Q),
        Key::W => InjectTarget::Vk(kbd::VK_W),
        Key::E => InjectTarget::Vk(kbd::VK_E),
        Key::R => InjectTarget::Vk(kbd::VK_R),
        Key::T => InjectTarget::Vk(kbd::VK_T),
        Key::Y => InjectTarget::Vk(kbd::VK_Y),
        Key::U => InjectTarget::Vk(kbd::VK_U),
        Key::I => InjectTarget::Vk(kbd::VK_I),
        Key::O => InjectTarget::Vk(kbd::VK_O),
        Key::P => InjectTarget::Vk(kbd::VK_P),
        Key::LeftBracket => InjectTarget::Vk(kbd::VK_OEM_4),
        Key::RightBracket => InjectTarget::Vk(kbd::VK_OEM_6),
        Key::Enter => InjectTarget::Vk(kbd::VK_RETURN),
        Key::A => InjectTarget::Vk(kbd::VK_A),
        Key::B => InjectTarget::Vk(kbd::VK_B),
        Key::C => InjectTarget::Vk(kbd::VK_C),
        Key::D => InjectTarget::Vk(kbd::VK_D),
        Key::F => InjectTarget::Vk(kbd::VK_F),
        Key::G => InjectTarget::Vk(kbd::VK_G),
        Key::H => InjectTarget::Vk(kbd::VK_H),
        Key::J => InjectTarget::Vk(kbd::VK_J),
        Key::K => InjectTarget::Vk(kbd::VK_K),
        Key::L => InjectTarget::Vk(kbd::VK_L),
        Key::Semicolon => InjectTarget::Vk(kbd::VK_OEM_1),
        Key::Apostrophe => InjectTarget::Vk(kbd::VK_OEM_7),
        Key::Grave => InjectTarget::Vk(kbd::VK_OEM_3),
        Key::Backslash => InjectTarget::Vk(kbd::VK_OEM_5),
        Key::Z => InjectTarget::Vk(kbd::VK_Z),
        Key::X => InjectTarget::Vk(kbd::VK_X),
        Key::V => InjectTarget::Vk(kbd::VK_V),
        Key::N => InjectTarget::Vk(kbd::VK_N),
        Key::M => InjectTarget::Vk(kbd::VK_M),
        Key::Comma => InjectTarget::Vk(kbd::VK_OEM_COMMA),
        Key::Dot => InjectTarget::Vk(kbd::VK_OEM_PERIOD),
        Key::Slash => InjectTarget::Vk(kbd::VK_OEM_2),
        Key::IntlBackslash => InjectTarget::Vk(kbd::VK_OEM_102),
        Key::Space => InjectTarget::Vk(kbd::VK_SPACE),
        // Modifiers and locks
        Key::LeftCtrl => InjectTarget::Vk(kbd::VK_LCONTROL),
        Key::RightCtrl => InjectTarget::Vk(kbd::VK_RCONTROL),
        Key::LeftShift => InjectTarget::Vk(kbd::VK_LSHIFT),
        Key::RightShift => InjectTarget::Vk(kbd::VK_RSHIFT),
        Key::LeftAlt => InjectTarget::Vk(kbd::VK_LMENU),
        Key::RightAlt => InjectTarget::Vk(kbd::VK_RMENU),
        Key::LeftMeta => InjectTarget::Vk(kbd::VK_LWIN),
        Key::RightMeta => InjectTarget::Vk(kbd::VK_RWIN),
        Key::Menu => InjectTarget::Vk(kbd::VK_APPS),
        Key::CapsLock => InjectTarget::Vk(kbd::VK_CAPITAL),
        Key::NumLock => InjectTarget::Vk(kbd::VK_NUMLOCK),
        Key::ScrollLock => InjectTarget::Vk(kbd::VK_SCROLL),
        // Navigation
        Key::Insert => InjectTarget::Vk(kbd::VK_INSERT),
        Key::Delete => InjectTarget::Vk(kbd::VK_DELETE),
        Key::Home => InjectTarget::Vk(kbd::VK_HOME),
        Key::End => InjectTarget::Vk(kbd::VK_END),
        Key::PageUp => InjectTarget::Vk(kbd::VK_PRIOR),
        Key::PageDown => InjectTarget::Vk(kbd::VK_NEXT),
        Key::Up => InjectTarget::Vk(kbd::VK_UP),
        Key::Down => InjectTarget::Vk(kbd::VK_DOWN),
        Key::Left => InjectTarget::Vk(kbd::VK_LEFT),
        Key::Right => InjectTarget::Vk(kbd::VK_RIGHT),
        Key::PrintScreen => InjectTarget::Vk(kbd::VK_SNAPSHOT),
        Key::Pause => InjectTarget::Vk(kbd::VK_PAUSE),
        // Function keys
        Key::F1 => InjectTarget::Vk(kbd::VK_F1),
        Key::F2 => InjectTarget::Vk(kbd::VK_F2),
        Key::F3 => InjectTarget::Vk(kbd::VK_F3),
        Key::F4 => InjectTarget::Vk(kbd::VK_F4),
        Key::F5 => InjectTarget::Vk(kbd::VK_F5),
        Key::F6 => InjectTarget::Vk(kbd::VK_F6),
        Key::F7 => InjectTarget::Vk(kbd::VK_F7),
        Key::F8 => InjectTarget::Vk(kbd::VK_F8),
        Key::F9 => InjectTarget::Vk(kbd::VK_F9),
        Key::F10 => InjectTarget::Vk(kbd::VK_F10),
        Key::F11 => InjectTarget::Vk(kbd::VK_F11),
        Key::F12 => InjectTarget::Vk(kbd::VK_F12),
        Key::F13 => InjectTarget::Vk(kbd::VK_F13),
        Key::F14 => InjectTarget::Vk(kbd::VK_F14),
        Key::F15 => InjectTarget::Vk(kbd::VK_F15),
        Key::F16 => InjectTarget::Vk(kbd::VK_F16),
        Key::F17 => InjectTarget::Vk(kbd::VK_F17),
        Key::F18 => InjectTarget::Vk(kbd::VK_F18),
        Key::F19 => InjectTarget::Vk(kbd::VK_F19),
        Key::F20 => InjectTarget::Vk(kbd::VK_F20),
        Key::F21 => InjectTarget::Vk(kbd::VK_F21),
        Key::F22 => InjectTarget::Vk(kbd::VK_F22),
        Key::F23 => InjectTarget::Vk(kbd::VK_F23),
        Key::F24 => InjectTarget::Vk(kbd::VK_F24),
        // Numpad
        Key::Numpad0 => InjectTarget::Vk(kbd::VK_NUMPAD0),
        Key::Numpad1 => InjectTarget::Vk(kbd::VK_NUMPAD1),
        Key::Numpad2 => InjectTarget::Vk(kbd::VK_NUMPAD2),
        Key::Numpad3 => InjectTarget::Vk(kbd::VK_NUMPAD3),
        Key::Numpad4 => InjectTarget::Vk(kbd::VK_NUMPAD4),
        Key::Numpad5 => InjectTarget::Vk(kbd::VK_NUMPAD5),
        Key::Numpad6 => InjectTarget::Vk(kbd::VK_NUMPAD6),
        Key::Numpad7 => InjectTarget::Vk(kbd::VK_NUMPAD7),
        Key::Numpad8 => InjectTarget::Vk(kbd::VK_NUMPAD8),
        Key::Numpad9 => InjectTarget::Vk(kbd::VK_NUMPAD9),
        Key::NumpadEnter => InjectTarget::Vk(kbd::VK_RETURN),
        Key::NumpadPlus => InjectTarget::Vk(kbd::VK_ADD),
        Key::NumpadMinus => InjectTarget::Vk(kbd::VK_SUBTRACT),
        Key::NumpadAsterisk => InjectTarget::Vk(kbd::VK_MULTIPLY),
        Key::NumpadSlash => InjectTarget::Vk(kbd::VK_DIVIDE),
        Key::NumpadDot => InjectTarget::Vk(kbd::VK_DECIMAL),
        Key::NumpadComma => InjectTarget::Vk(kbd::VK_SEPARATOR),
        // Media keys
        Key::VolumeUp => InjectTarget::Vk(kbd::VK_VOLUME_UP),
        Key::VolumeDown => InjectTarget::Vk(kbd::VK_VOLUME_DOWN),
        Key::Mute => InjectTarget::Vk(kbd::VK_VOLUME_MUTE),
        Key::PlayPause => InjectTarget::Vk(kbd::VK_MEDIA_PLAY_PAUSE),
        Key::Play => InjectTarget::Vk(kbd::VK_PLAY),
        Key::Stop => InjectTarget::Vk(kbd::VK_MEDIA_STOP),
        Key::NextTrack => InjectTarget::Vk(kbd::VK_MEDIA_NEXT_TRACK),
        Key::PreviousTrack => InjectTarget::Vk(kbd::VK_MEDIA_PREV_TRACK),
        Key::MediaSelect => InjectTarget::Vk(kbd::VK_LAUNCH_MEDIA_SELECT),
        // Power, brightness and illumination: no virtual-key codes exist.
        Key::Sleep => InjectTarget::Vk(kbd::VK_SLEEP),
        // Browser and application keys
        Key::Email => InjectTarget::Vk(kbd::VK_LAUNCH_MAIL),
        Key::Calculator => InjectTarget::Vk(kbd::VK_LAUNCH_APP2),
        Key::Computer => InjectTarget::Vk(kbd::VK_LAUNCH_APP1),
        Key::Search => InjectTarget::Vk(kbd::VK_BROWSER_SEARCH),
        Key::BrowserHome => InjectTarget::Vk(kbd::VK_BROWSER_HOME),
        Key::BrowserBack => InjectTarget::Vk(kbd::VK_BROWSER_BACK),
        Key::BrowserForward => InjectTarget::Vk(kbd::VK_BROWSER_FORWARD),
        Key::Refresh => InjectTarget::Vk(kbd::VK_BROWSER_REFRESH),
        Key::Bookmarks => InjectTarget::Vk(kbd::VK_BROWSER_FAVORITES),
        // Editing keys
        Key::Copy => InjectTarget::Vk(kbd::VK_OEM_COPY),
        Key::Help => InjectTarget::Vk(kbd::VK_HELP),
        // International keys. KANA/HANGUL share a code and only toggle the
        // IME mode; CONVERT/NONCONVERT are the Japanese IME keys.
        Key::Hiragana | Key::Katakana => InjectTarget::Vk(kbd::VK_KANA),
        Key::Hangeul => InjectTarget::Vk(kbd::VK_HANGUL),
        Key::Hanja => InjectTarget::Vk(kbd::VK_HANJA),
        Key::Henkan => InjectTarget::Vk(kbd::VK_CONVERT),
        Key::Muhenkan => InjectTarget::Vk(kbd::VK_NONCONVERT),
        // Mouse buttons
        Key::MouseLeft => InjectTarget::Mouse(MouseButton::Left),
        Key::MouseRight => InjectTarget::Mouse(MouseButton::Right),
        Key::MouseMiddle => InjectTarget::Mouse(MouseButton::Middle),
        Key::MouseSide => InjectTarget::Mouse(MouseButton::Side),
        Key::MouseExtra => InjectTarget::Mouse(MouseButton::Extra),

        key => return Err(crate::Error::KeyNotMapped(key)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_known_virtual_key_codes() {
        const MAPPINGS: &[(u16, Key)] = &[
            (kbd::VK_ESCAPE, Key::Escape),
            (kbd::VK_A, Key::A),
            (kbd::VK_0, Key::Digit0),
            (kbd::VK_RETURN, Key::Enter),
            (kbd::VK_SPACE, Key::Space),
            (kbd::VK_UP, Key::Up),
            (kbd::VK_F24, Key::F24),
            (kbd::VK_NUMPAD7, Key::Numpad7),
            (kbd::VK_VOLUME_UP, Key::VolumeUp),
            (kbd::VK_BROWSER_BACK, Key::BrowserBack),
            (kbd::VK_LAUNCH_APP2, Key::Calculator),
        ];
        for &(vk, expected) in MAPPINGS {
            assert_eq!(map_vk(vk, false), Some(expected), "code 0x{vk:02x}");
        }
    }

    #[test]
    fn distinguishes_left_and_right_modifiers() {
        assert_eq!(map_vk(kbd::VK_SHIFT, false), Some(Key::LeftShift));
        assert_eq!(map_vk(kbd::VK_SHIFT, true), Some(Key::RightShift));
        assert_eq!(map_vk(kbd::VK_CONTROL, true), Some(Key::RightCtrl));
        assert_eq!(map_vk(kbd::VK_MENU, false), Some(Key::LeftAlt));
        assert_eq!(map_vk(kbd::VK_LSHIFT, true), Some(Key::LeftShift));
        assert_eq!(map_vk(kbd::VK_RSHIFT, false), Some(Key::RightShift));
    }

    #[test]
    fn preserves_unknown_codes_and_filters_ime_codes() {
        assert_eq!(map_vk(kbd::VK_ATTN, false), Some(Key::Other(kbd::VK_ATTN)));
        assert_eq!(map_vk(kbd::VK_PACKET, false), None);
        assert_eq!(map_vk(kbd::VK_PROCESSKEY, false), None);
    }

    #[test]
    fn every_named_key_maps_or_is_rejected() {
        for key in Key::get_all_keys() {
            match map_key(key) {
                Ok(target) => assert_ne!(target, InjectTarget::Vk(0)),
                Err(crate::Error::KeyNotMapped(k)) => assert_eq!(k, key),
                Err(other) => panic!("unexpected error for {key}: {other:?}"),
            }
        }
    }

    #[test]
    fn injection_roundtrips_through_listening() {
        // Keys whose virtual-key code is shared with another key: Windows
        // has no distinct code for these, so they deliberately do not
        // round-trip.
        const SHARED_CODES: &[Key] = &[
            Key::NumpadEnter, // shares VK_RETURN with Enter
            Key::Katakana,    // shares VK_KANA with Hiragana
            Key::Hangeul,     // VK_HANGUL is the same code as VK_KANA
        ];

        for key in Key::get_all_keys() {
            let Ok(target) = map_key(key) else { continue };
            let InjectTarget::Vk(vk) = target else {
                continue;
            };
            if SHARED_CODES.contains(&key) {
                continue;
            }
            assert_eq!(map_vk(vk, false), Some(key), "key {key} does not roundtrip");
        }
    }

    #[test]
    fn other_codes_pass_through_for_injection() {
        assert_eq!(map_key(Key::Other(0x70)).unwrap(), InjectTarget::Vk(0x70));
    }
}
