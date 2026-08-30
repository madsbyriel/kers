//! # keyrs
//!
//! Cross-platform input events: listen to key presses and mouse button
//! clicks, and (planned) send them back.
//!
//! # Layout
//!
//! - [`keys`]: platform-neutral event vocabulary ([`Key`], [`KeyState`],
//!   [`InputKeyEvent`]). Nothing here may depend on a platform backend.
//! - [`api`]: the [`InputEventListener`] / [`InputEventSender`] traits that
//!   every backend implements.
//! - [`platform`]: per-OS backends, selected at compile time by `target_os`.
//!   Only Linux is implemented so far.
//!
//! # Example
//!
//! ```no_run
//! use keyrs::{default_keyboard, InputEventListener};
//!
//! #[tokio::main]
//! async fn main() -> keyrs::Result<()> {
//!     let mut keyboard = default_keyboard();
//!     loop {
//!         println!("{}", keyboard.next_event().await?);
//!     }
//! }
//! ```

pub mod api;
pub mod keys;
pub mod platform;

mod error;

pub use api::{InputEventListener, InputEventSender};
pub use error::Error;
pub use keys::{DeviceId, InputKeyEvent, Key, KeyState};
pub use platform::{DefaultKeyboard, default_keyboard};

/// The standard result type used throughout keyrs.
pub type Result<T> = std::result::Result<T, Error>;
