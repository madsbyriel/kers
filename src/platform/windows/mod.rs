//! Windows backend, built on the low-level keyboard and mouse hooks
//! (`WH_KEYBOARD_LL` / `WH_MOUSE_LL`) for listening and
//! [`SendInput`] for sending.
//!
//! # Architecture
//!
//! - one dedicated OS thread installs both low-level hooks and runs a
//!   Windows message loop for their lifetime. Windows dispatches the hook
//!   callbacks in that thread, so they must not block;
//! - each callback converts its message into an [`InputKeyEvent`] and
//!   pushes it into a Tokio broadcast channel (a synchronous, non-blocking
//!   operation — the channel overwrites its oldest value when full);
//! - [`WindowsKeyboard::next_event`] consumes that channel on the Tokio
//!   side. Dropping the [`WindowsKeyboard`] posts `WM_QUIT` to the hook
//!   thread, which unhooks and exits;
//! - sending goes the other way: [`WindowsKeyboard::send_event`] injects
//!   the event with [`SendInput`], so it reaches the system exactly like
//!   input from a physical device.
//!
//! Low-level hooks report system-wide events without needing a window or
//! elevation, but they do not identify the physical device an event came
//! from, so every event is attributed to a fixed
//! [`crate::keys::DeviceId`] ("Windows keyboard" / "Windows mouse").
//!
//! [`Key::Other`] codes are Windows virtual-key codes (`VK_*`).
//!
//! # Limitations
//!
//! - [`KeyState::Hold`] cannot be produced when sending: Windows
//!   synthesizes auto-repeat itself, so a hold is sent as a plain press.
//! - UIPI blocks [`SendInput`] events from reaching processes running at
//!   a higher integrity level.

mod mapping;

use std::cell::RefCell;
use std::io;
use std::mem::size_of;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};

use async_trait::async_trait;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use windows_sys::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::SystemInformation::GetTickCount64;
use windows_sys::Win32::System::Threading::GetCurrentThreadId;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYEVENTF_KEYUP, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN,
    MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, MOUSEINPUT, SendInput,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, KBDLLHOOKSTRUCT, LLKHF_EXTENDED, LLKHF_UP, MSG, MSLLHOOKSTRUCT,
    PostThreadMessageW, SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL, WH_MOUSE_LL,
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_QUIT, WM_RBUTTONDOWN,
    WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP, XBUTTON1, XBUTTON2,
};

use crate::api::InputEventListener;
use crate::keys::{DeviceId, InputKeyEvent, Key, KeyState};
use crate::{Error, InputEventSender, Result};

use mapping::{InjectTarget, MouseButton, map_key, map_vk};

/// How large the event buffer shared with subscribers is.
const EVENT_CAPACITY: usize = 1024;

/// The default Windows keyboard backend.
///
/// Created with [`WindowsKeyboard::new`]; unlike the Linux backend it can
/// be created outside a Tokio runtime (only [`InputEventListener`] usage
/// needs one).
pub struct WindowsKeyboard {
    events: broadcast::Receiver<InputKeyEvent>,
    sender: broadcast::Sender<InputKeyEvent>,
    cancel: CancellationToken,
    hook_thread_id: Arc<AtomicU32>,
    hook_thread: Option<JoinHandle<()>>,
}

// The sender the hook callbacks publish to.
//
// Hook callbacks run in the hook thread (see [`run_hook_thread`]), so
// they read this thread-local without synchronization.
thread_local! {
    static HOOK_SENDER: RefCell<Option<broadcast::Sender<InputKeyEvent>>> = const {
        RefCell::new(None)
    };
}

impl WindowsKeyboard {
    /// Start listening to system-wide keyboard and mouse button events.
    pub fn new() -> Result<Self> {
        let cancel = CancellationToken::new();
        let (sender, events) = broadcast::channel::<InputKeyEvent>(EVENT_CAPACITY);

        let hook_thread_id = Arc::new(AtomicU32::new(0));
        let (result_sender, result_receiver) = std::sync::mpsc::sync_channel(1);

        let hook_thread = std::thread::Builder::new()
            .name("keyrs-windows-hook".to_string())
            .spawn({
                let sender = sender.clone();
                let cancel = cancel.clone();
                let hook_thread_id = hook_thread_id.clone();
                move || {
                    // Report hook installation failures back to `new`.
                    let result = run_hook_thread(sender, cancel, &hook_thread_id);
                    let _ = result_sender.send(result);
                }
            })
            .map_err(Error::from)?;

        // Block until the hooks are installed (or installation failed).
        let result = result_receiver.recv().unwrap_or_else(|_| {
            Err(io::Error::other("hook thread exited before installing hooks").into())
        });
        result?;

        Ok(Self {
            events,
            sender,
            cancel,
            hook_thread_id,
            hook_thread: Some(hook_thread),
        })
    }

    /// Subscribe an additional consumer to the event stream.
    ///
    /// Every subscriber receives every event; a subscriber that falls
    /// behind gets `broadcast::error::RecvError::Lagged` from `recv()` and
    /// may miss events.
    pub fn subscribe(&self) -> broadcast::Receiver<InputKeyEvent> {
        self.sender.subscribe()
    }
}

impl Drop for WindowsKeyboard {
    fn drop(&mut self) {
        // Wake the hook thread's message loop so it unhooks and exits.
        self.cancel.cancel();
        let thread_id = self.hook_thread_id.load(Ordering::Relaxed);
        if thread_id != 0 {
            // SAFETY: `WM_QUIT` with a zero payload is a valid message for
            // any existing thread.
            unsafe {
                PostThreadMessageW(thread_id, WM_QUIT, 0, 0);
            }
        }
        if let Some(thread) = self.hook_thread.take() {
            let _ = thread.join();
        }
    }
}

#[async_trait]
impl InputEventListener for WindowsKeyboard {
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
impl InputEventSender for WindowsKeyboard {
    async fn send_event(&mut self, key: Key, state: KeyState) -> Result<()> {
        inject(key, state)
    }
}

/// Install both hooks and run the message loop that keeps them alive,
/// until `WM_QUIT` arrives or the cancellation token fires.
fn run_hook_thread(
    sender: broadcast::Sender<InputKeyEvent>,
    cancel: CancellationToken,
    thread_id: &AtomicU32,
) -> Result<()> {
    unsafe {
        // SAFETY: `GetCurrentThreadId` has no preconditions.
        thread_id.store(GetCurrentThreadId(), Ordering::Relaxed);

        // The keyboard may have been dropped before this thread got here;
        // install nothing in that case.
        if cancel.is_cancelled() {
            return Ok(());
        }

        HOOK_SENDER.with(|slot| *slot.borrow_mut() = Some(sender));

        // SAFETY: a null module name selects the executable's module,
        // which is correct for low-level hooks installed by the current
        // process.
        let module = GetModuleHandleW(std::ptr::null());
        if module.is_null() {
            return Err(io::Error::last_os_error().into());
        }

        // SAFETY: the hook procedures are `unsafe extern "system"`
        // functions that stay valid until the hooks are removed.
        let keyboard_hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), module, 0);
        let mouse_hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), module, 0);
        if keyboard_hook.is_null() || mouse_hook.is_null() {
            let error = io::Error::last_os_error();
            if !keyboard_hook.is_null() {
                UnhookWindowsHookEx(keyboard_hook);
            }
            if !mouse_hook.is_null() {
                UnhookWindowsHookEx(mouse_hook);
            }
            return Err(error.into());
        }

        let mut message = std::mem::zeroed::<MSG>();
        loop {
            // The low-level hook callbacks are dispatched by the system
            // while this call waits. `GetMessageW` returns 0 for `WM_QUIT`
            // and -1 on error; both end the loop.
            let result = GetMessageW(&mut message, std::ptr::null_mut(), 0, 0);
            if result <= 0 {
                break;
            }
        }

        UnhookWindowsHookEx(keyboard_hook);
        UnhookWindowsHookEx(mouse_hook);
    }
    Ok(())
}

/// Publish one event from a hook callback.
///
/// Must only be called from the hook thread: it reads the thread-local
/// sender. `broadcast::Sender::send` never blocks (it overwrites the
/// oldest value when the buffer is full) and only fails once every
/// receiver is gone.
fn publish(event: InputKeyEvent) {
    HOOK_SENDER.with(|slot| {
        if let Some(sender) = slot.borrow().as_ref() {
            let _ = sender.send(event);
        }
    });
}

unsafe extern "system" fn keyboard_proc(code: i32, _wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        // SAFETY: for a keyboard hook callback, `lparam` points to a
        // `KBDLLHOOKSTRUCT` that is valid for the duration of this call.
        let info = unsafe { &*(lparam as *const KBDLLHOOKSTRUCT) };
        if let Some(key) = map_vk(info.vkCode as u16, info.flags & LLKHF_EXTENDED != 0) {
            publish(InputKeyEvent {
                key,
                state: if info.flags & LLKHF_UP != 0 {
                    KeyState::Up
                } else {
                    KeyState::Down
                },
                device: keyboard_device().clone(),
                timestamp: tick_timestamp(info.time),
            });
        }
    }
    // SAFETY: `CallNextHookEx` with a null hook handle is valid for
    // low-level hooks, which ignore the handle.
    unsafe { CallNextHookEx(std::ptr::null_mut(), code, _wparam, lparam) }
}

unsafe extern "system" fn mouse_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        // SAFETY: for a mouse hook callback, `lparam` points to a
        // `MSLLHOOKSTRUCT` that is valid for the duration of this call.
        let info = unsafe { &*(lparam as *const MSLLHOOKSTRUCT) };
        let message = wparam as u32;

        let (key, down) = match message {
            WM_LBUTTONDOWN => (Some(Key::MouseLeft), true),
            WM_LBUTTONUP => (Some(Key::MouseLeft), false),
            WM_RBUTTONDOWN => (Some(Key::MouseRight), true),
            WM_RBUTTONUP => (Some(Key::MouseRight), false),
            WM_MBUTTONDOWN => (Some(Key::MouseMiddle), true),
            WM_MBUTTONUP => (Some(Key::MouseMiddle), false),
            WM_XBUTTONDOWN | WM_XBUTTONUP => {
                // The high word of `mouseData` is the pressed X button.
                let key = match (info.mouseData >> 16) as u16 {
                    XBUTTON1 => Some(Key::MouseSide),
                    XBUTTON2 => Some(Key::MouseExtra),
                    _ => None,
                };
                (key, message == WM_XBUTTONDOWN)
            }
            _ => (None, false),
        };

        if let Some(key) = key {
            publish(InputKeyEvent {
                key,
                state: if down { KeyState::Down } else { KeyState::Up },
                device: mouse_device().clone(),
                timestamp: tick_timestamp(info.time),
            });
        }
    }
    // SAFETY: `CallNextHookEx` with a null hook handle is valid for
    // low-level hooks, which ignore the handle.
    unsafe { CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam) }
}

/// The fixed [`DeviceId`] attributed to keyboard events.
fn keyboard_device() -> &'static DeviceId {
    static ID: std::sync::LazyLock<DeviceId> = std::sync::LazyLock::new(|| DeviceId {
        name: "Windows keyboard".to_string(),
        location: "WH_KEYBOARD_LL".to_string(),
    });
    &ID
}

/// The fixed [`DeviceId`] attributed to mouse button events.
fn mouse_device() -> &'static DeviceId {
    static ID: std::sync::LazyLock<DeviceId> = std::sync::LazyLock::new(|| DeviceId {
        name: "Windows mouse".to_string(),
        location: "WH_MOUSE_LL".to_string(),
    });
    &ID
}

/// Convert a hook `time` field (milliseconds since boot, 32-bit) into a
/// wall-clock [`SystemTime`].
fn tick_timestamp(ticks: u32) -> SystemTime {
    // SAFETY: `GetTickCount64` has no preconditions.
    let now = unsafe { GetTickCount64() };
    // The event happened within the last 32-bit wrap period (~49.7 days).
    let mut event = (now & !0xFFFF_FFFF) | u64::from(ticks);
    if event > now {
        event = event.wrapping_sub(1u64 << 32);
    }
    SystemTime::now() - Duration::from_millis(now.saturating_sub(event))
}

/// Inject one event with [`SendInput`].
fn inject(key: Key, state: KeyState) -> Result<()> {
    // `Hold` has no Windows equivalent: the system synthesizes
    // auto-repeat while a key is physically held, so a repeat event is
    // sent as a plain press.
    let up = state == KeyState::Up;

    let input = match map_key(key)? {
        InjectTarget::Vk(vk) => INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: 0,
                    dwFlags: if up { KEYEVENTF_KEYUP } else { 0 },
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        InjectTarget::Mouse(button) => {
            let (flags, mouse_data) = match (button, up) {
                (MouseButton::Left, false) => (MOUSEEVENTF_LEFTDOWN, 0),
                (MouseButton::Left, true) => (MOUSEEVENTF_LEFTUP, 0),
                (MouseButton::Right, false) => (MOUSEEVENTF_RIGHTDOWN, 0),
                (MouseButton::Right, true) => (MOUSEEVENTF_RIGHTUP, 0),
                (MouseButton::Middle, false) => (MOUSEEVENTF_MIDDLEDOWN, 0),
                (MouseButton::Middle, true) => (MOUSEEVENTF_MIDDLEUP, 0),
                (MouseButton::Side, false) => (MOUSEEVENTF_XDOWN, u32::from(XBUTTON1)),
                (MouseButton::Side, true) => (MOUSEEVENTF_XUP, u32::from(XBUTTON1)),
                (MouseButton::Extra, false) => (MOUSEEVENTF_XDOWN, u32::from(XBUTTON2)),
                (MouseButton::Extra, true) => (MOUSEEVENTF_XUP, u32::from(XBUTTON2)),
            };
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: mouse_data,
                        dwFlags: flags,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            }
        }
    };

    // SAFETY: `input` is fully initialized and `size_of::<INPUT>()`
    // matches the size the OS expects.
    let sent = unsafe { SendInput(1, &input, size_of::<INPUT>() as i32) };
    if sent == 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}
