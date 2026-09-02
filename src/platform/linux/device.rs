//! An open evdev device wrapped with its cross-platform identity.

use crate::Result;
use crate::keys::DeviceId;
use std::path::PathBuf;

/// An open evdev device and its [`DeviceId`].
pub(super) struct DeviceInfo {
    pub id: DeviceId,
    pub stream: evdev::EventStream,
}

impl DeviceInfo {
    /// Build [`DeviceInfo`] from an open evdev device.
    ///
    /// Returns `Ok(None)` when the device reports no key or button events,
    /// and `Err` when its event stream cannot be created.
    pub fn from_device(device: evdev::Device, node: PathBuf) -> Result<Option<Self>> {
        if !has_key_events(&device) {
            return Ok(None);
        }

        let id = DeviceId {
            name: device.name().unwrap_or("unnamed device").to_string(),
            location: device
                .physical_path()
                .map(str::to_string)
                .unwrap_or_else(|| node.display().to_string()),
        };
        let stream = device.into_event_stream()?;

        Ok(Some(Self { id, stream }))
    }
}

/// The device reports at least one supported key or button code. This
/// covers keyboards as well as mice: mouse buttons (including the side
/// buttons) live in the `BTN_*` range of the same code space.
fn has_key_events(device: &evdev::Device) -> bool {
    let Some(keys) = device.supported_keys() else {
        return false;
    };

    keys.iter().any(|key| key.code() > 0)
}
