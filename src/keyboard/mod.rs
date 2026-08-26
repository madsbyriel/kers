use async_trait::async_trait;

use crate::keys::Key;
use crate::Result;

pub mod linux_kbd;

#[async_trait]
pub trait InputEventHandler {
    async fn get_key_async(&self) -> Result<Key>;
}

pub enum DeviceType {
    Keyboard,
    Mouse,
}
