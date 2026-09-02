//! Error types for keyrs.

use std::fmt::{self, Display, Formatter};
use std::io;

/// Errors returned by keyrs.
///
/// Hand-written instead of deriving via `thiserror` to keep the dependency
/// surface minimal; switch to `thiserror` if the variant list grows.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// An I/O error from the underlying platform backend (e.g. a device
    /// that could not be read).
    Io(io::Error),

    /// The event source has shut down; no further events will be delivered.
    EventSourceClosed,

    KeyNotMapped(crate::keys::Key),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(error) => write!(f, "device I/O error: {error}"),
            Error::EventSourceClosed => write!(f, "event source closed"),
            Error::KeyNotMapped(key) => write!(f, "key not mapped: {key:?}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(error) => Some(error),
            Error::EventSourceClosed => None,
            Error::KeyNotMapped(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}
