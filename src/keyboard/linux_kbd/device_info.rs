use std::{pin::Pin, sync::Arc};

use evdev::Device;
use tokio::sync::Mutex;

use crate::{Result, Error};

pub struct DeviceInfo {
    pub path: String,
    pub name: String,
    pub id: evdev::InputId,
    pub stream: evdev::EventStream,
}

impl PartialEq for DeviceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.path == other.path && self.name == other.name
    }
}

impl std::fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeviceInfo {}", self.name)
    }
}

impl TryFrom<Device> for DeviceInfo {
    type Error = Error;

    fn try_from(value: Device) -> Result<Self> {
        let id = value.input_id();
        let path = value.physical_path().unwrap_or("Unknown path").to_string();
        let name = value.name().unwrap_or("Unknown device").to_string();
        let stream = value.into_event_stream()?;

        Ok(Self {
            name,
            id,
            stream,
            path,
        })
    }
}
