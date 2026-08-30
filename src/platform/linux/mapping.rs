//! Conversion from evdev's event model into the platform-neutral
//! [`crate::keys`] vocabulary.
//!
//! This is where Linux-specific knowledge lives; nothing outside
//! `platform::linux` may touch evdev types.

use crate::keys::{DeviceId, InputKeyEvent, Key, KeyState};

/// Map a raw evdev key code to a [`Key`].
///
/// Covers every key or button a typical keyboard or mouse reports; anything
/// else falls back to [`Key::Other`] with the raw code preserved.
pub(super) fn map_key(code: evdev::KeyCode) -> Key {
    use evdev::KeyCode as K;

    match code {
        // Main keys (US layout)
        K::KEY_ESC => Key::Escape,
        K::KEY_1 => Key::Digit1,
        K::KEY_2 => Key::Digit2,
        K::KEY_3 => Key::Digit3,
        K::KEY_4 => Key::Digit4,
        K::KEY_5 => Key::Digit5,
        K::KEY_6 => Key::Digit6,
        K::KEY_7 => Key::Digit7,
        K::KEY_8 => Key::Digit8,
        K::KEY_9 => Key::Digit9,
        K::KEY_0 => Key::Digit0,
        K::KEY_MINUS => Key::Minus,
        K::KEY_EQUAL => Key::Equal,
        K::KEY_BACKSPACE => Key::Backspace,
        K::KEY_TAB => Key::Tab,
        K::KEY_Q => Key::Q,
        K::KEY_W => Key::W,
        K::KEY_E => Key::E,
        K::KEY_R => Key::R,
        K::KEY_T => Key::T,
        K::KEY_Y => Key::Y,
        K::KEY_U => Key::U,
        K::KEY_I => Key::I,
        K::KEY_O => Key::O,
        K::KEY_P => Key::P,
        K::KEY_LEFTBRACE => Key::LeftBracket,
        K::KEY_RIGHTBRACE => Key::RightBracket,
        K::KEY_ENTER => Key::Enter,
        K::KEY_A => Key::A,
        K::KEY_B => Key::B,
        K::KEY_C => Key::C,
        K::KEY_D => Key::D,
        K::KEY_F => Key::F,
        K::KEY_G => Key::G,
        K::KEY_H => Key::H,
        K::KEY_J => Key::J,
        K::KEY_K => Key::K,
        K::KEY_L => Key::L,
        K::KEY_SEMICOLON => Key::Semicolon,
        K::KEY_APOSTROPHE => Key::Apostrophe,
        K::KEY_GRAVE => Key::Grave,
        K::KEY_BACKSLASH => Key::Backslash,
        K::KEY_Z => Key::Z,
        K::KEY_X => Key::X,
        K::KEY_V => Key::V,
        K::KEY_N => Key::N,
        K::KEY_M => Key::M,
        K::KEY_COMMA => Key::Comma,
        K::KEY_DOT => Key::Dot,
        K::KEY_SLASH => Key::Slash,
        K::KEY_102ND => Key::IntlBackslash,
        K::KEY_SPACE => Key::Space,
        // Modifiers and locks
        K::KEY_LEFTCTRL => Key::LeftCtrl,
        K::KEY_RIGHTCTRL => Key::RightCtrl,
        K::KEY_LEFTSHIFT => Key::LeftShift,
        K::KEY_RIGHTSHIFT => Key::RightShift,
        K::KEY_LEFTALT => Key::LeftAlt,
        K::KEY_RIGHTALT => Key::RightAlt,
        K::KEY_LEFTMETA => Key::LeftMeta,
        K::KEY_RIGHTMETA => Key::RightMeta,
        K::KEY_COMPOSE => Key::Compose,
        K::KEY_FN => Key::Fn,
        K::KEY_MENU | K::KEY_CONTEXT_MENU => Key::Menu,
        K::KEY_CAPSLOCK => Key::CapsLock,
        K::KEY_NUMLOCK => Key::NumLock,
        K::KEY_SCROLLLOCK => Key::ScrollLock,
        // Navigation
        K::KEY_INSERT => Key::Insert,
        K::KEY_DELETE => Key::Delete,
        K::KEY_HOME => Key::Home,
        K::KEY_END => Key::End,
        K::KEY_PAGEUP => Key::PageUp,
        K::KEY_PAGEDOWN => Key::PageDown,
        K::KEY_UP => Key::Up,
        K::KEY_DOWN => Key::Down,
        K::KEY_LEFT => Key::Left,
        K::KEY_RIGHT => Key::Right,
        K::KEY_SYSRQ => Key::PrintScreen,
        K::KEY_PAUSE => Key::Pause,
        // Function keys
        K::KEY_F1 => Key::F1,
        K::KEY_F2 => Key::F2,
        K::KEY_F3 => Key::F3,
        K::KEY_F4 => Key::F4,
        K::KEY_F5 => Key::F5,
        K::KEY_F6 => Key::F6,
        K::KEY_F7 => Key::F7,
        K::KEY_F8 => Key::F8,
        K::KEY_F9 => Key::F9,
        K::KEY_F10 => Key::F10,
        K::KEY_F11 => Key::F11,
        K::KEY_F12 => Key::F12,
        K::KEY_F13 => Key::F13,
        K::KEY_F14 => Key::F14,
        K::KEY_F15 => Key::F15,
        K::KEY_F16 => Key::F16,
        K::KEY_F17 => Key::F17,
        K::KEY_F18 => Key::F18,
        K::KEY_F19 => Key::F19,
        K::KEY_F20 => Key::F20,
        K::KEY_F21 => Key::F21,
        K::KEY_F22 => Key::F22,
        K::KEY_F23 => Key::F23,
        K::KEY_F24 => Key::F24,
        // Numpad
        K::KEY_KP0 => Key::Numpad0,
        K::KEY_KP1 => Key::Numpad1,
        K::KEY_KP2 => Key::Numpad2,
        K::KEY_KP3 => Key::Numpad3,
        K::KEY_KP4 => Key::Numpad4,
        K::KEY_KP5 => Key::Numpad5,
        K::KEY_KP6 => Key::Numpad6,
        K::KEY_KP7 => Key::Numpad7,
        K::KEY_KP8 => Key::Numpad8,
        K::KEY_KP9 => Key::Numpad9,
        K::KEY_KPENTER => Key::NumpadEnter,
        K::KEY_KPPLUS => Key::NumpadPlus,
        K::KEY_KPMINUS => Key::NumpadMinus,
        K::KEY_KPASTERISK => Key::NumpadAsterisk,
        K::KEY_KPSLASH => Key::NumpadSlash,
        K::KEY_KPDOT => Key::NumpadDot,
        K::KEY_KPEQUAL => Key::NumpadEqual,
        K::KEY_KPCOMMA => Key::NumpadComma,
        K::KEY_KPJPCOMMA => Key::KpJpComma,
        // Mouse buttons
        K::BTN_LEFT => Key::MouseLeft,
        K::BTN_RIGHT => Key::MouseRight,
        K::BTN_MIDDLE => Key::MouseMiddle,
        K::BTN_SIDE => Key::MouseSide,
        K::BTN_EXTRA => Key::MouseExtra,
        K::BTN_FORWARD => Key::MouseForward,
        K::BTN_BACK => Key::MouseBack,
        other => Key::Other(other.code()),
    }
}

/// Convert a raw evdev event into an [`InputKeyEvent`].
///
/// Returns `None` for non-key events (`EV_SYN` markers, relative/absolute
/// axes, ...) and for key events with an unknown state value.
pub(super) fn map_input_event(
    event: evdev::InputEvent,
    device: &DeviceId,
) -> Option<InputKeyEvent> {
    if event.event_type() != evdev::EventType::KEY {
        return None;
    }

    Some(InputKeyEvent {
        key: map_key(evdev::KeyCode::new(event.code())),
        state: KeyState::from_value(event.value())?,
        device: device.clone(),
        timestamp: event.timestamp(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    /// Every key code keyrs knows about, and the variant it must map to.
    /// Generated together with the enum and `map_key` from one key list.
    const MAPPINGS: &[(u16, Key)] = &[
        // Main keys (US layout)
        (evdev::KeyCode::KEY_ESC.code(), Key::Escape),
        (evdev::KeyCode::KEY_1.code(), Key::Digit1),
        (evdev::KeyCode::KEY_2.code(), Key::Digit2),
        (evdev::KeyCode::KEY_3.code(), Key::Digit3),
        (evdev::KeyCode::KEY_4.code(), Key::Digit4),
        (evdev::KeyCode::KEY_5.code(), Key::Digit5),
        (evdev::KeyCode::KEY_6.code(), Key::Digit6),
        (evdev::KeyCode::KEY_7.code(), Key::Digit7),
        (evdev::KeyCode::KEY_8.code(), Key::Digit8),
        (evdev::KeyCode::KEY_9.code(), Key::Digit9),
        (evdev::KeyCode::KEY_0.code(), Key::Digit0),
        (evdev::KeyCode::KEY_MINUS.code(), Key::Minus),
        (evdev::KeyCode::KEY_EQUAL.code(), Key::Equal),
        (evdev::KeyCode::KEY_BACKSPACE.code(), Key::Backspace),
        (evdev::KeyCode::KEY_TAB.code(), Key::Tab),
        (evdev::KeyCode::KEY_Q.code(), Key::Q),
        (evdev::KeyCode::KEY_W.code(), Key::W),
        (evdev::KeyCode::KEY_E.code(), Key::E),
        (evdev::KeyCode::KEY_R.code(), Key::R),
        (evdev::KeyCode::KEY_T.code(), Key::T),
        (evdev::KeyCode::KEY_Y.code(), Key::Y),
        (evdev::KeyCode::KEY_U.code(), Key::U),
        (evdev::KeyCode::KEY_I.code(), Key::I),
        (evdev::KeyCode::KEY_O.code(), Key::O),
        (evdev::KeyCode::KEY_P.code(), Key::P),
        (evdev::KeyCode::KEY_LEFTBRACE.code(), Key::LeftBracket),
        (evdev::KeyCode::KEY_RIGHTBRACE.code(), Key::RightBracket),
        (evdev::KeyCode::KEY_ENTER.code(), Key::Enter),
        (evdev::KeyCode::KEY_A.code(), Key::A),
        (evdev::KeyCode::KEY_B.code(), Key::B),
        (evdev::KeyCode::KEY_C.code(), Key::C),
        (evdev::KeyCode::KEY_D.code(), Key::D),
        (evdev::KeyCode::KEY_F.code(), Key::F),
        (evdev::KeyCode::KEY_G.code(), Key::G),
        (evdev::KeyCode::KEY_H.code(), Key::H),
        (evdev::KeyCode::KEY_J.code(), Key::J),
        (evdev::KeyCode::KEY_K.code(), Key::K),
        (evdev::KeyCode::KEY_L.code(), Key::L),
        (evdev::KeyCode::KEY_SEMICOLON.code(), Key::Semicolon),
        (evdev::KeyCode::KEY_APOSTROPHE.code(), Key::Apostrophe),
        (evdev::KeyCode::KEY_GRAVE.code(), Key::Grave),
        (evdev::KeyCode::KEY_BACKSLASH.code(), Key::Backslash),
        (evdev::KeyCode::KEY_Z.code(), Key::Z),
        (evdev::KeyCode::KEY_X.code(), Key::X),
        (evdev::KeyCode::KEY_V.code(), Key::V),
        (evdev::KeyCode::KEY_N.code(), Key::N),
        (evdev::KeyCode::KEY_M.code(), Key::M),
        (evdev::KeyCode::KEY_COMMA.code(), Key::Comma),
        (evdev::KeyCode::KEY_DOT.code(), Key::Dot),
        (evdev::KeyCode::KEY_SLASH.code(), Key::Slash),
        (evdev::KeyCode::KEY_102ND.code(), Key::IntlBackslash),
        (evdev::KeyCode::KEY_SPACE.code(), Key::Space),
        // Modifiers and locks
        (evdev::KeyCode::KEY_LEFTCTRL.code(), Key::LeftCtrl),
        (evdev::KeyCode::KEY_RIGHTCTRL.code(), Key::RightCtrl),
        (evdev::KeyCode::KEY_LEFTSHIFT.code(), Key::LeftShift),
        (evdev::KeyCode::KEY_RIGHTSHIFT.code(), Key::RightShift),
        (evdev::KeyCode::KEY_LEFTALT.code(), Key::LeftAlt),
        (evdev::KeyCode::KEY_RIGHTALT.code(), Key::RightAlt),
        (evdev::KeyCode::KEY_LEFTMETA.code(), Key::LeftMeta),
        (evdev::KeyCode::KEY_RIGHTMETA.code(), Key::RightMeta),
        (evdev::KeyCode::KEY_COMPOSE.code(), Key::Compose),
        (evdev::KeyCode::KEY_FN.code(), Key::Fn),
        (evdev::KeyCode::KEY_MENU.code(), Key::Menu),
        (evdev::KeyCode::KEY_CONTEXT_MENU.code(), Key::Menu),
        (evdev::KeyCode::KEY_CAPSLOCK.code(), Key::CapsLock),
        (evdev::KeyCode::KEY_NUMLOCK.code(), Key::NumLock),
        (evdev::KeyCode::KEY_SCROLLLOCK.code(), Key::ScrollLock),
        // Navigation
        (evdev::KeyCode::KEY_INSERT.code(), Key::Insert),
        (evdev::KeyCode::KEY_DELETE.code(), Key::Delete),
        (evdev::KeyCode::KEY_HOME.code(), Key::Home),
        (evdev::KeyCode::KEY_END.code(), Key::End),
        (evdev::KeyCode::KEY_PAGEUP.code(), Key::PageUp),
        (evdev::KeyCode::KEY_PAGEDOWN.code(), Key::PageDown),
        (evdev::KeyCode::KEY_UP.code(), Key::Up),
        (evdev::KeyCode::KEY_DOWN.code(), Key::Down),
        (evdev::KeyCode::KEY_LEFT.code(), Key::Left),
        (evdev::KeyCode::KEY_RIGHT.code(), Key::Right),
        (evdev::KeyCode::KEY_SYSRQ.code(), Key::PrintScreen),
        (evdev::KeyCode::KEY_PAUSE.code(), Key::Pause),
        // Function keys
        (evdev::KeyCode::KEY_F1.code(), Key::F1),
        (evdev::KeyCode::KEY_F2.code(), Key::F2),
        (evdev::KeyCode::KEY_F3.code(), Key::F3),
        (evdev::KeyCode::KEY_F4.code(), Key::F4),
        (evdev::KeyCode::KEY_F5.code(), Key::F5),
        (evdev::KeyCode::KEY_F6.code(), Key::F6),
        (evdev::KeyCode::KEY_F7.code(), Key::F7),
        (evdev::KeyCode::KEY_F8.code(), Key::F8),
        (evdev::KeyCode::KEY_F9.code(), Key::F9),
        (evdev::KeyCode::KEY_F10.code(), Key::F10),
        (evdev::KeyCode::KEY_F11.code(), Key::F11),
        (evdev::KeyCode::KEY_F12.code(), Key::F12),
        (evdev::KeyCode::KEY_F13.code(), Key::F13),
        (evdev::KeyCode::KEY_F14.code(), Key::F14),
        (evdev::KeyCode::KEY_F15.code(), Key::F15),
        (evdev::KeyCode::KEY_F16.code(), Key::F16),
        (evdev::KeyCode::KEY_F17.code(), Key::F17),
        (evdev::KeyCode::KEY_F18.code(), Key::F18),
        (evdev::KeyCode::KEY_F19.code(), Key::F19),
        (evdev::KeyCode::KEY_F20.code(), Key::F20),
        (evdev::KeyCode::KEY_F21.code(), Key::F21),
        (evdev::KeyCode::KEY_F22.code(), Key::F22),
        (evdev::KeyCode::KEY_F23.code(), Key::F23),
        (evdev::KeyCode::KEY_F24.code(), Key::F24),
        // Numpad
        (evdev::KeyCode::KEY_KP0.code(), Key::Numpad0),
        (evdev::KeyCode::KEY_KP1.code(), Key::Numpad1),
        (evdev::KeyCode::KEY_KP2.code(), Key::Numpad2),
        (evdev::KeyCode::KEY_KP3.code(), Key::Numpad3),
        (evdev::KeyCode::KEY_KP4.code(), Key::Numpad4),
        (evdev::KeyCode::KEY_KP5.code(), Key::Numpad5),
        (evdev::KeyCode::KEY_KP6.code(), Key::Numpad6),
        (evdev::KeyCode::KEY_KP7.code(), Key::Numpad7),
        (evdev::KeyCode::KEY_KP8.code(), Key::Numpad8),
        (evdev::KeyCode::KEY_KP9.code(), Key::Numpad9),
        (evdev::KeyCode::KEY_KPENTER.code(), Key::NumpadEnter),
        (evdev::KeyCode::KEY_KPPLUS.code(), Key::NumpadPlus),
        (evdev::KeyCode::KEY_KPMINUS.code(), Key::NumpadMinus),
        (evdev::KeyCode::KEY_KPASTERISK.code(), Key::NumpadAsterisk),
        (evdev::KeyCode::KEY_KPSLASH.code(), Key::NumpadSlash),
        (evdev::KeyCode::KEY_KPDOT.code(), Key::NumpadDot),
        (evdev::KeyCode::KEY_KPEQUAL.code(), Key::NumpadEqual),
        (evdev::KeyCode::KEY_KPCOMMA.code(), Key::NumpadComma),
        (evdev::KeyCode::KEY_KPJPCOMMA.code(), Key::KpJpComma),
        // Mouse buttons
        (evdev::KeyCode::BTN_LEFT.code(), Key::MouseLeft),
        (evdev::KeyCode::BTN_RIGHT.code(), Key::MouseRight),
        (evdev::KeyCode::BTN_MIDDLE.code(), Key::MouseMiddle),
        (evdev::KeyCode::BTN_SIDE.code(), Key::MouseSide),
        (evdev::KeyCode::BTN_EXTRA.code(), Key::MouseExtra),
        (evdev::KeyCode::BTN_FORWARD.code(), Key::MouseForward),
        (evdev::KeyCode::BTN_BACK.code(), Key::MouseBack),
    ];

    fn device() -> DeviceId {
        DeviceId {
            name: "test device".into(),
            location: "test location".into(),
        }
    }

    fn key_event(code: u16, value: i32) -> evdev::InputEvent {
        evdev::InputEvent::new(evdev::EventType::KEY.0, code, value)
    }

    #[test]
    fn maps_all_known_key_codes() {
        let device = device();
        for &(code, expected) in MAPPINGS {
            let event = key_event(code, 1);
            let mapped = map_input_event(event, &device).expect("known key should map");
            assert_eq!(mapped.key, expected, "code 0x{code:02x} mapped incorrectly");
        }
    }

    #[test]
    fn preserves_unknown_key_codes() {
        let code = evdev::KeyCode::KEY_MACRO.code();
        let event = key_event(code, 1);
        let mapped = map_input_event(event, &device()).expect("key event should map");
        assert_eq!(mapped.key, Key::Other(code));
    }

    #[test]
    fn maps_key_states() {
        assert_eq!(
            map_input_event(key_event(1, 0), &device()).unwrap().state,
            KeyState::Up
        );
        assert_eq!(
            map_input_event(key_event(1, 1), &device()).unwrap().state,
            KeyState::Down
        );
        assert_eq!(
            map_input_event(key_event(1, 2), &device()).unwrap().state,
            KeyState::Hold
        );
    }

    #[test]
    fn rejects_unknown_state_values() {
        let event = key_event(evdev::KeyCode::KEY_A.code(), 3);
        assert!(map_input_event(event, &device()).is_none());
    }

    #[test]
    fn filters_non_key_events() {
        let sync = evdev::InputEvent::new(evdev::EventType::SYNCHRONIZATION.0, 0, 0);
        assert!(map_input_event(sync, &device()).is_none());

        let relative = evdev::InputEvent::new(evdev::EventType::RELATIVE.0, 0, 5);
        assert!(map_input_event(relative, &device()).is_none());
    }

    #[test]
    fn carries_device_and_timestamp() {
        let device = device();
        let event = key_event(evdev::KeyCode::KEY_D.code(), 1);
        let mapped = map_input_event(event, &device).expect("key event should map");
        assert_eq!(mapped.device, device);
        assert_eq!(mapped.timestamp, SystemTime::UNIX_EPOCH);
    }
}
