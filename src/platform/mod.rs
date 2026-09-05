//! Platform backends.
//!
//! Exactly one backend is compiled per target OS and converts that
//! platform's native event model into the [`crate::keys`] vocabulary.
//! Backends must never leak their platform types into [`crate::keys`] or
//! [`crate::api`].
//!
//! Linux and Windows are implemented; other platforms fail to compile
//! until their backend exists (see the `compile_error!` below).

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
compile_error!(
    "keyrs: no keyboard backend exists for this platform yet (Linux and Windows are implemented)"
);

/// The keyboard backend selected for the current platform.
#[cfg(target_os = "linux")]
pub type DefaultKeyboard = linux::LinuxKeyboard;
#[cfg(target_os = "windows")]
pub type DefaultKeyboard = windows::WindowsKeyboard;

/// Create the keyboard backend for the current platform.
///
/// The Linux backend must be called from within a Tokio runtime; the
/// Windows backend can be created anywhere.
#[cfg(target_os = "linux")]
pub fn default_keyboard() -> crate::Result<DefaultKeyboard> {
    DefaultKeyboard::new()
}

/// Create the keyboard backend for the current platform.
///
/// The Linux backend must be called from within a Tokio runtime; the
/// Windows backend can be created anywhere.
#[cfg(target_os = "windows")]
pub fn default_keyboard() -> crate::Result<DefaultKeyboard> {
    DefaultKeyboard::new()
}
