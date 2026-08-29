//! Device manager: keeps one reader task per connected keyboard.

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use tokio::sync::{broadcast, mpsc};
use tokio::task::AbortHandle;
use tokio_util::sync::CancellationToken;

use crate::keys::{DeviceId, InputKeyEvent};

use super::device::DeviceInfo;
use super::mapping::map_input_event;

/// How long a failed device read waits before it is retried.
const READ_RETRY_INTERVAL: Duration = Duration::from_secs(1);

/// Run the device manager until discovery ends or cancellation is
/// requested, then stop all device readers.
pub(super) async fn run_device_manager(
    mut scans: mpsc::Receiver<Vec<DeviceInfo>>,
    events: broadcast::Sender<InputKeyEvent>,
    cancel: CancellationToken,
) {
    let mut manager = DeviceManager::new(events, cancel.clone());

    loop {
        tokio::select! {
            biased;
            _ = cancel.cancelled() => break,
            scan = scans.recv() => match scan {
                Some(devices) => manager.apply_scan(devices),
                None => {
                    tracing::debug!("discovery ended; device manager shutting down");
                    break;
                }
            },
        }
    }

    for handle in manager.readers.values() {
        handle.abort();
    }
}

/// Tracks the set of keyboards currently being read.
struct DeviceManager {
    readers: HashMap<DeviceId, AbortHandle>,
    events: broadcast::Sender<InputKeyEvent>,
    cancel: CancellationToken,
}

impl DeviceManager {
    fn new(events: broadcast::Sender<InputKeyEvent>, cancel: CancellationToken) -> Self {
        Self {
            readers: HashMap::new(),
            events,
            cancel,
        }
    }

    /// Diff one discovery scan against the active reader set.
    fn apply_scan(&mut self, devices: Vec<DeviceInfo>) {
        let seen: HashSet<DeviceId> = devices.iter().map(|device| device.id.clone()).collect();

        // Stop readers for devices that disappeared from the scan.
        let gone: Vec<DeviceId> = self
            .readers
            .keys()
            .filter(|id| !seen.contains(*id))
            .cloned()
            .collect();
        for id in gone {
            if let Some(handle) = self.readers.remove(&id) {
                handle.abort();
                tracing::info!("device removed: {}", id.name);
            }
        }

        // Start readers for devices we are not reading yet.
        for device in devices {
            if self.readers.contains_key(&device.id) {
                continue;
            }
            tracing::info!("device added: {}", device.id.name);
            let id = device.id.clone();
            let handle = tokio::spawn(read_device_events(
                device,
                self.events.clone(),
                self.cancel.clone(),
            ));
            self.readers.insert(id, handle.abort_handle());
        }
    }
}

/// Read events from one device and broadcast them until cancelled.
///
/// Read errors (usually: the device was unplugged) are retried with a
/// delay; the manager removes the device once it disappears from a
/// discovery scan.
async fn read_device_events(
    device: DeviceInfo,
    events: broadcast::Sender<InputKeyEvent>,
    cancel: CancellationToken,
) {
    let mut stream = device.stream;
    let mut error_logged = false;

    loop {
        tokio::select! {
            biased;
            _ = cancel.cancelled() => return,
            result = stream.next_event() => match result {
                Ok(raw) => {
                    error_logged = false;
                    if let Some(event) = map_input_event(raw, &device.id) {
                        // `send` only fails when every receiver is gone,
                        // i.e. during shutdown.
                        if events.send(event).is_err() {
                            tracing::debug!("key event channel closed; dropping event");
                        }
                    }
                }
                Err(error) => {
                    if !error_logged {
                        tracing::warn!("read error on {}: {error}; retrying", device.id.name);
                        error_logged = true;
                    }
                    tokio::time::sleep(READ_RETRY_INTERVAL).await;
                }
            },
        }
    }
}
