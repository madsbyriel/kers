use std::fmt::Display;

use anyhow::bail;
use evdev::KeyCode;

use crate::{Error, Result};

#[derive(Debug)]
pub enum Key {
    Unknown,
    A,
    B,
    C,
    D,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Unknown => write!(f, "Unknown"),
            Key::A => write!(f, "A"),
            Key::B => write!(f, "B"),
            Key::C => write!(f, "C"),
            Key::D => write!(f, "D"),
        }
    }
}

#[derive(Debug)]
pub enum KeyState {
    Down,
    Up,
    Hold,
}

impl Display for KeyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyState::Down => write!(f, "Down"),
            KeyState::Up => write!(f, "Up"),
            KeyState::Hold => write!(f, "Hold"),
        }
    }
}

#[derive(Debug)]
pub struct InputKeyEvent {
    pub key: Key,
    pub state: KeyState
}

impl Display for InputKeyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InputKeyEvent {{ key: {}, state: {} }}", self.key, self.state)
    }
}

impl From<KeyCode> for Key {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::KEY_A => Key::A,
            KeyCode::KEY_B => Key::B,
            KeyCode::KEY_C => Key::C,
            KeyCode::KEY_D => Key::D,
            _ => Key::Unknown,
        }
    }
}

impl TryFrom<i32> for KeyState {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self> {
        match value {
            0 => Ok(KeyState::Up),
            1 => Ok(KeyState::Down),
            2 => Ok(KeyState::Hold),
            _ => bail!("Invalid key state: {}", value),
        }
    }
}
