//! Utilities for creating demos.

use std::{error, fmt, result};

pub mod gl;
pub mod glfw;
pub mod imgui;
mod macros;

/// A specialized result type.
pub type Result<T> = result::Result<T, Error>;

/// Hitchcock error.
#[derive(Debug)]
pub enum Error {
    /// GLFW error.
    Glfw(glfw::Error),

    /// Dear ImGui error.
    ImGui(imgui::Error),
}

impl From<glfw::Error> for Error {
    fn from(err: glfw::Error) -> Self {
        Error::Glfw(err)
    }
}

impl From<imgui::Error> for Error {
    fn from(err: imgui::Error) -> Self {
        Error::ImGui(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Glfw(err) => write!(f, "GLFW error: {err}"),
            Error::ImGui(err) => write!(f, "Dear ImGui error: {err}"),
        }
    }
}

impl error::Error for Error {}
