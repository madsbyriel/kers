use std::{collections::HashSet, path::PathBuf, sync::Arc};
use anyhow::{anyhow, bail};
use async_trait::async_trait;
use evdev::{Device, KeyCode};
use futures::FutureExt;
use tokio::sync::Mutex;

use crate::{Error, Result, keyboard::{DeviceType, InputEventHandler}, keys::Key};

struct DeviceInfo {
    name: String,
    stream: evdev::EventStream,
    path: Option<String>,
    device_type: DeviceType,
}

impl TryFrom<Device> for DeviceInfo {
    type Error = Error;

    fn try_from(value: Device) -> Result<Self> {
        let name = value.name().unwrap_or("Unknown device").to_string();
        let path = value.physical_path().map(|v| v.to_string());
        let supported_keys = match value.supported_keys() {
            None => bail!("No keys available for device: {name}"),
            Some(v) => {
                let mut key_set = HashSet::new();
                for s in v {
                    key_set.insert(s);
                }
                key_set
            },
        };
        let stream = value.into_event_stream()?;
        let device_type;
        if supported_keys.contains(&KeyCode::KEY_ENTER) {
            device_type = DeviceType::Keyboard;
        }
        else if supported_keys.contains(&KeyCode::BTN_LEFT) {
            device_type = DeviceType::Mouse;
        }
        else {
            bail!("Unsupported device type: {name}");
        }

        Ok(Self {
            name,
            path,
            stream,
            device_type: device_type,
        })
    }
}

pub struct LinuxKbd {
    devices: Arc<Mutex<Vec<DeviceInfo>>>
}

impl LinuxKbd {
    pub fn new() -> Self {
        let mut devices = Vec::new();
        for (_, device) in evdev::enumerate() {
            let info: DeviceInfo = match device.try_into() {
                Ok(v) => v,
                Err(e) => {
                    tracing::debug!("Failed to connect device: {}", e);
                    continue;
                },
            };

            match info.device_type {
                DeviceType::Keyboard => tracing::info!("Connected keyboard: {}", info.name),
                DeviceType::Mouse => tracing::info!("Connected mouse: {}", info.name),
            }

            devices.push(info);
        }

        Self {
            devices: Arc::new(Mutex::new(devices))
        }
    }
}

#[async_trait]
impl InputEventHandler for LinuxKbd {
    async fn get_key_async(&self) -> Result<Key> {
        let mut devices = self.devices.lock().await;
        let device_iter = devices.iter_mut();

        let all_events = device_iter.map(|d| d.stream.next_event().boxed()).collect::<Vec<_>>();
        let (first_event, index, _) = futures::future::select_all(all_events).await;
        let first_event = match first_event {
            Ok(v) => v,
            Err(e) => {
                bail!("Error getting event: {}", e);
            },
        };

        let device = match devices.get(index) {
            Some(v) => v,
            None => {
                bail!("Error getting device info");
            },
        };

        let code = KeyCode::new(first_event.code());
        let value = first_event.value();
        let name = &device.name;
        tracing::trace!("{name}: {code:?} {value}");

        Ok(Key::from(code))
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
