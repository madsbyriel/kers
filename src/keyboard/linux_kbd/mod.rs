use std::time::Duration;

use async_trait::async_trait;
use futures::FutureExt;
use tokio::sync::{mpsc};
use evdev::KeyCode;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

mod device_info;

use crate::{Result, keyboard::{InputEventHandler, linux_kbd::device_info::DeviceInfo}, keys::{InputKeyEvent, Key, KeyState}};

const DEVICE_SCAN_INTERVAL_MS: u64 = 5000;

pub struct LinuxKbd {
    cancel: CancellationToken
}

impl LinuxKbd {
    pub fn new() -> Self {
        let cancel = CancellationToken::new();
        let (device_sender, device_receiver) = mpsc::channel::<DeviceInfo>(100);
        let (key_sender, mut key_receiver) = mpsc::channel::<InputKeyEvent>(100);

        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            send_devices(device_sender, cancel_clone).await;
        });

        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            send_keys(device_receiver, key_sender, cancel_clone).await;
        });

        tokio::spawn(async move {
            loop {
                let e = key_receiver.recv().await;

                if let Some(e) = e {
                    tracing::trace!("Received key event: {}", e);
                }
            }
        });

        Self {
            cancel
        }
    }
}

#[async_trait]
impl InputEventHandler for LinuxKbd {
    async fn get_key_async(&self) -> Result<Key> {
        Ok(Key::Unknown)
    }
}

async fn send_devices(sender: mpsc::Sender<DeviceInfo>, cancel: CancellationToken) {
    async fn find_devices(sender: mpsc::Sender<DeviceInfo>) {
        loop {
            let devices = evdev::enumerate();
            for (_, device) in devices {
                let device_info = match DeviceInfo::try_from(device) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!("Failed to connect device: {}", e);
                        continue;
                    },
                };

                if let Err(e) = sender.send(device_info).await {
                    tracing::error!("Failed to send device, shutting down device sender: {}", e);
                    continue;
                }
            }

            tokio::time::sleep(Duration::from_millis(DEVICE_SCAN_INTERVAL_MS)).await;
        }
    }


    futures::select! {
        _ = find_devices(sender).fuse() => tracing::info!("Device sender shutting down"),
        _ = cancel.cancelled().fuse() => tracing::info!("Device sender cancelled, shutting down"),
    }
}

async fn send_keys(
    receiver: mpsc::Receiver<DeviceInfo>,
    sender: mpsc::Sender<InputKeyEvent>,
    cancel: CancellationToken
) {
    futures::select! {
        _ = key_loop(receiver, sender).fuse() => tracing::info!("Key sender shutting down"),
        _ = cancel.cancelled().fuse() => tracing::info!("Key sender cancelled, shutting down"),
    };
}

async fn key_loop(
    mut receiver: mpsc::Receiver<DeviceInfo>,
    sender: mpsc::Sender<InputKeyEvent>,
) {
    let mut devices = vec![];

    loop {
        loop {
            let v = match receiver.try_recv() {
                Ok(v) => v,
                Err(e) => {
                    match e {
                        mpsc::error::TryRecvError::Empty => {
                            break;
                        },
                        mpsc::error::TryRecvError::Disconnected => {
                            tracing::info!("Device receiver disconnected, shutting down");
                            return;
                        },
                    };
                },
            };

            if devices.contains(&v) {
                continue;
            }

            tracing::trace!("Added device: {}", v.name);
            devices.push(v);
        }

        if devices.is_empty() {
            tokio::time::sleep(Duration::from_millis(1000)).await;
            continue;
        }

        let futures = devices.iter_mut().map(|device| {
            device.stream.next_event().boxed()
        });

        let (input_event, idx, _) = futures::future::select_all(futures).await;
        if let Ok(input_event) = input_event {
            tracing::trace!("Received input event: {:?}", input_event);

            match send_key(input_event, &sender).await {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Error sending key: {}", e);
                },
            };
        }

        if let Err(e) = input_event {
            tracing::error!("Error reading input event: {}", e);
        }
    }
}

async fn send_key(event: evdev::InputEvent, sender: &mpsc::Sender<InputKeyEvent>) -> Result<()> {
    tracing::trace!("Sending key: {:?}", event);
    let value = event.value();
    let key = KeyCode::new(event.code());

    let key_state: KeyState = value.try_into()?;

    let event = InputKeyEvent {
        key: key.into(),
        state: key_state
    };

    sender.send(event).await?;

    Ok(())
}
