//! Linux backend, built on [`evdev`] (`/dev/input/event*`).
//!
//! # Architecture
//!
//! - a discovery task polls [`evdev::enumerate`] every few seconds and
//!   forwards the current device set;
//! - the device manager diffs each scan against the devices it is already
//!   reading: it starts one reader task per new input device and aborts
//!   readers for devices that disappeared;
//! - each reader task maps raw evdev events into [`InputKeyEvent`]s and
//!   broadcasts them;
//! - [`LinuxKeyboard`] consumes the broadcast to serve
//!   [`InputEventListener`] and any additional subscribers;
//! - sending goes the other way: [`LinuxKeyboard`] injects events through
//!   a uinput [`VirtualDevice`] created at startup, so injected keys reach
//!   the system exactly like events from a physical keyboard.
//!
//! Polling is a placeholder: a udev/inotify hotplug watcher can replace the
//! discovery task later without changing the rest.
//!
//! Raw key codes carried by [`crate::keys::Key::Other`] are Linux evdev key
//! codes (keyboard keys live in the `KEY_*` range, mouse buttons in the
//! `BTN_*` range of the same code space).

mod device;
mod manager;
mod mapping;

use std::time::Duration;

use async_trait::async_trait;
use evdev::uinput::VirtualDevice;
use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;

use crate::api::InputEventListener;
use crate::keys::InputKeyEvent;
use crate::{InputEventSender, Key, KeyState, Result};

use device::DeviceInfo;
use manager::run_device_manager;

/// How often the system device list is re-scanned.
const DEVICE_SCAN_INTERVAL: Duration = Duration::from_secs(5);

/// The default Linux keyboard backend.
///
/// Created with [`LinuxKeyboard::new`]; must be called from within a Tokio
/// runtime.
pub struct LinuxKeyboard {
    events: broadcast::Receiver<InputKeyEvent>,
    sender: broadcast::Sender<InputKeyEvent>,
    cancel: CancellationToken,
    v_device: VirtualDevice,
}

impl LinuxKeyboard {
    /// Start listening to all connected devices that report key or button
    /// events (keyboards and mice), and open the uinput virtual device
    /// used to send events back.
    ///
    /// Requires read access to `/dev/input/event*` (to listen) and write
    /// access to `/dev/uinput` (to send).
    pub fn new() -> Result<Self> {
        let cancel = CancellationToken::new();
        let (sender, events) = broadcast::channel::<InputKeyEvent>(1024);
        let (device_sender, device_receiver) = mpsc::channel::<Vec<DeviceInfo>>(64);

        tokio::spawn(discover_devices(device_sender, cancel.clone()));
        tokio::spawn(run_device_manager(
            device_receiver,
            sender.clone(),
            cancel.clone(),
        ));

        let v_device = match create_virtual_device() {
            Ok(v) => Ok(v),
            Err(e) => {
                cancel.cancel();
                Err(e)
            }
        }?;

        Ok(Self {
            events,
            sender,
            cancel,
            v_device,
        })
    }

    /// Subscribe an additional consumer to the event stream.
    ///
    /// Every subscriber receives every event; a subscriber that falls behind
    /// gets `broadcast::error::RecvError::Lagged` from `recv()` and may
    /// miss events.
    pub fn subscribe(&self) -> broadcast::Receiver<InputKeyEvent> {
        self.sender.subscribe()
    }
}

impl Drop for LinuxKeyboard {
    fn drop(&mut self) {
        // Stop discovery, the device manager, and all per-device readers.
        self.cancel.cancel();
    }
}

#[async_trait]
impl InputEventListener for LinuxKeyboard {
    async fn next_event(&mut self) -> Result<InputKeyEvent> {
        loop {
            match self.events.recv().await {
                Ok(event) => return Ok(event),
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    tracing::warn!("{skipped} key events were dropped: consumer too slow");
                }
                Err(broadcast::error::RecvError::Closed) => {
                    return Err(crate::Error::EventSourceClosed);
                }
            }
        }
    }
}

#[async_trait]
impl InputEventSender for LinuxKeyboard {
    async fn send_event(&mut self, key: Key, state: KeyState) -> Result<()> {
        // The virtual device advertises exactly the keys `get_key_set`
        // produced, i.e. every key that maps back to an evdev code.
        // Keys outside that set (`Key::Other`) fail here with
        // `Error::KeyNotMapped` instead of a kernel-level rejection.
        let code = evdev::KeyCode::try_from(key)?;

        let event = evdev::InputEvent::new(evdev::EventType::KEY.0, code.code(), state.to_value());

        // `emit` appends the SYN_REPORT that tells the input core the
        // event is complete.
        self.v_device.emit(&[event])?;

        Ok(())
    }
}

/// Poll the system device list and forward each full scan to the device
/// manager.
async fn discover_devices(sender: mpsc::Sender<Vec<DeviceInfo>>, cancel: CancellationToken) {
    loop {
        let mut devices = Vec::new();
        for (path, device) in evdev::enumerate() {
            match DeviceInfo::from_device(device, path) {
                Ok(Some(info)) => {
                    tracing::trace!("found device: {}", info.id.name);
                    devices.push(info);
                }
                Ok(None) => {} // not a keyboard
                Err(error) => tracing::warn!("skipping device: {error}"),
            }
        }

        if sender.send(devices).await.is_err() {
            tracing::debug!("device manager gone; discovery shutting down");
            return;
        }

        tokio::select! {
            biased;
            _ = cancel.cancelled() => {
                tracing::debug!("discovery cancelled; shutting down");
                return;
            }
            _ = tokio::time::sleep(DEVICE_SCAN_INTERVAL) => {}
        }
    }
}

fn get_key_set() -> evdev::AttributeSet<evdev::KeyCode> {
    let mut keys = evdev::AttributeSet::new();

    // Support all keys we can map from the crates keyset to evdev keycodes.
    for key in Key::get_all_keys() {
        let key_code = match evdev::KeyCode::try_from(key) {
            Ok(v) => v,
            Err(_) => continue,
        };
        keys.insert(key_code);
    }

    keys
}

fn create_virtual_device() -> Result<VirtualDevice> {
    let builder = VirtualDevice::builder()?;
    let keys = get_key_set();

    let virtual_device = builder
        .name("keyrs virtual keyboard")
        .with_keys(&keys)?
        .build()?;
    Ok(virtual_device)
}
