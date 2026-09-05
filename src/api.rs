//! The traits every platform backend implements.
//!
//! Listening and sending are deliberately separate traits: a backend can
//! support either direction on its own, and both can be implemented by the
//! same backend object.

use async_trait::async_trait;

use crate::Result;
use crate::keys::{InputKeyEvent, Key, KeyState};

/// A backend that listens for key and mouse button events.
///
/// [`InputEventListener::next_event`] waits until an event is available and
/// returns it. Concrete backends may additionally offer fan-out
/// subscriptions (see the `subscribe` methods on the concrete backends).
#[async_trait]
pub trait InputEventListener: Send + Sync {
    /// Wait for and return the next key event.
    async fn next_event(&mut self) -> Result<InputKeyEvent>;
}

/// A backend that sends (injects) keyboard events.
///
/// The backend decides which device the events go out through (on Linux,
/// a uinput virtual device; on Windows, the `SendInput` API), so no
/// device id is part of the signature. An injected event flows through
/// the system like an event from a physical keyboard, so a listening
/// backend on the same machine receives it too.
#[async_trait]
pub trait InputEventSender: Send + Sync {
    /// Inject a key event.
    async fn send_event(&mut self, key: Key, state: KeyState) -> Result<()>;
}
