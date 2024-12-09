//! Utilities for creating demos.

use std::ffi::NulError;

pub mod gl;
pub mod glfw;
pub mod imgui;
mod macros;

/// A specialized result type.
type Result<T> = std::result::Result<T, Error>;

/// The error type.
#[derive(Debug)]
pub enum Error {
    /// Error when calling library function.
    Ffi,

    /// Invalid C string.
    InvalidCString(NulError),
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Error {
        Error::InvalidCString(err)
    }
}
