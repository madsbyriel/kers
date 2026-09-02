//! Inject a few key events through the backend's virtual device.
//!
//! Run with: `cargo run --example send`
//!
//! On Linux this needs write access to `/dev/uinput`. The events are sent
//! as raw key codes, so a plain `Key::A` press produces lowercase "a"
//! unless a shift modifier is held.

use std::time::Duration;

use keyrs::keys::{Key, KeyState};
use keyrs::{InputEventSender, default_keyboard};

#[tokio::main]
async fn main() -> keyrs::Result<()> {
    let mut keyboard = default_keyboard().expect("Couldn't create keyboard");
    println!("Injecting 'a', 'b', 'c' key presses...");

    for key in [Key::A, Key::B, Key::C] {
        keyboard.send_event(key, KeyState::Down).await?;
        tokio::time::sleep(Duration::from_millis(50)).await;
        keyboard.send_event(key, KeyState::Up).await?;
        tokio::time::sleep(Duration::from_millis(150)).await;
    }

    Ok(())
}
