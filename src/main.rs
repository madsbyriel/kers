use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::keyboard::{InputEventHandler, linux_kbd::LinuxKbd};

mod keyboard;
mod keys;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let kbd = LinuxKbd::new();
    while true {
        let key = kbd.get_key_async().await;
        let key = match key {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Error: {}", e);
                continue;
            }
        };
    }

    println!("Hello, world!");
}

