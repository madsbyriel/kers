//! Print every key and mouse button event until the `D` key is pressed.
//!
//! Run with: `cargo run --example listen`
//!
//! On Linux this needs read access to `/dev/input/event*` (typically via
//! the `input` group or root).

use keyrs::keys::{Key, KeyState};
use keyrs::{InputEventListener, default_keyboard};

#[tokio::main]
async fn main() -> keyrs::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let mut keyboard = default_keyboard().expect("Couldn't create keyboard");
    println!("Listening for key and mouse button events; press D to exit.");

    loop {
        let event = keyboard.next_event().await?;
        println!("{event}");

        if event.key == Key::D && event.state == KeyState::Down {
            break;
        }
    }

    Ok(())
}
