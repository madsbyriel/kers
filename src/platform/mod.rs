//! Platform backends.
//!
//! Exactly one backend is compiled per target OS and converts that
//! platform's native event model into the [`crate::keys`] vocabulary.
//! Backends must never leak their platform types into [`crate::keys`] or
//! [`crate::api`].
//!
//! Currently only Linux is implemented; other platforms fail to compile
//! until their backend exists (see the `compile_error!` below).

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(not(target_os = "linux"))]
compile_error!(
    "keyrs: no keyboard backend exists for this platform yet (only Linux is implemented)"
);

/// The keyboard backend selected for the current platform.
#[cfg(target_os = "linux")]
pub type DefaultKeyboard = linux::LinuxKeyboard;

/// Create the keyboard backend for the current platform.
///
/// Must be called from within a Tokio runtime.
#[cfg(target_os = "linux")]
pub fn default_keyboard() -> DefaultKeyboard {
    DefaultKeyboard::new()
}
