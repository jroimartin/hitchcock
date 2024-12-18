//! Utilities for creating demos.

use std::{error, fmt, result};

pub mod gl;
pub mod glfw;
pub mod imgui;
mod macros;
pub mod stb_image;

/// A specialized result type.
pub type Result<T> = result::Result<T, Error>;

/// This error types aggregates the rest of errors used by the crate.
#[derive(Debug)]
pub enum Error {
    /// GLFW error.
    Glfw(glfw::Error),

    /// OpenGL error.
    Gl(gl::Error),

    /// Dear ImGui error.
    ImGui(imgui::Error),
}

impl From<glfw::Error> for Error {
    fn from(err: glfw::Error) -> Error {
        Error::Glfw(err)
    }
}

impl From<gl::Error> for Error {
    fn from(err: gl::Error) -> Error {
        Error::Gl(err)
    }
}

impl From<imgui::Error> for Error {
    fn from(err: imgui::Error) -> Error {
        Error::ImGui(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Glfw(err) => write!(f, "GLFW error: {err}"),
            Error::Gl(err) => write!(f, "OpenGL error: {err}"),
            Error::ImGui(err) => write!(f, "Dear ImGui error: {err}"),
        }
    }
}

impl error::Error for Error {}

/// 2-dimensional vector.
#[derive(Copy, Clone)]
pub struct Vec2<T>(pub T, pub T);

impl<T> From<Vec2<T>> for [T; 2] {
    fn from(v: Vec2<T>) -> [T; 2] {
        [v.0, v.1]
    }
}

impl<T: Copy> From<[T; 2]> for Vec2<T> {
    fn from(v: [T; 2]) -> Vec2<T> {
        Vec2(v[0], v[1])
    }
}

/// 3-dimensional vector.
#[derive(Copy, Clone)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T> From<Vec3<T>> for [T; 3] {
    fn from(v: Vec3<T>) -> [T; 3] {
        [v.0, v.1, v.2]
    }
}

impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(v: [T; 3]) -> Vec3<T> {
        Vec3(v[0], v[1], v[2])
    }
}

/// 4-dimensional vector.
#[derive(Copy, Clone)]
pub struct Vec4<T>(pub T, pub T, pub T, pub T);

impl<T> From<Vec4<T>> for [T; 4] {
    fn from(v: Vec4<T>) -> [T; 4] {
        [v.0, v.1, v.2, v.3]
    }
}

impl<T: Copy> From<[T; 4]> for Vec4<T> {
    fn from(v: [T; 4]) -> Vec4<T> {
        Vec4(v[0], v[1], v[2], v[3])
    }
}
