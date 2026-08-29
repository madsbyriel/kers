//! The traits every platform backend implements.
//!
//! Listening and sending are deliberately separate traits: a backend can
//! support either direction on its own, and both can be implemented by the
//! same backend object.

use async_trait::async_trait;

use crate::Result;
use crate::keys::InputKeyEvent;

/// A backend that listens for key and mouse button events.
///
/// [`InputEventListener::next_event`] waits until an event is available and
/// returns it. Concrete backends may additionally offer fan-out
/// subscriptions (see `LinuxKeyboard::subscribe`).
#[async_trait]
pub trait InputEventListener: Send + Sync {
    /// Wait for and return the next key event.
    async fn next_event(&mut self) -> Result<InputKeyEvent>;
}

/// A backend that sends (injects) keyboard events.
///
/// This is the reserved send half of the goal: no backend implements it
/// yet, but defining it next to [`InputEventListener`] fixes the shape so
/// listen and send can live behind the same backend object when they land.
#[async_trait]
pub trait InputEventSender: Send + Sync {
    /// Inject a key event.
    async fn send_event(&mut self, event: InputKeyEvent) -> Result<()>;
}
