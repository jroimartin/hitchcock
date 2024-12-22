//! Utilities for creating demos.

use std::{error, fmt, ops, result};

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

    /// stb_image error.
    StbImage(stb_image::Error),
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

impl From<stb_image::Error> for Error {
    fn from(err: stb_image::Error) -> Error {
        Error::StbImage(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Glfw(err) => write!(f, "GLFW error: {err}"),
            Error::Gl(err) => write!(f, "OpenGL error: {err}"),
            Error::ImGui(err) => write!(f, "Dear ImGui error: {err}"),
            Error::StbImage(err) => write!(f, "stb_image error: {err}"),
        }
    }
}

impl error::Error for Error {}

macro_rules! define_vec {
    ($name:ident, $n:expr) => {
        #[doc = concat!($n, "-dimensional vector.")]
        #[derive(Copy, Clone, Default)]
        #[repr(C)]
        pub struct $name<T>([T; $n]);

        impl<T> std::convert::From<$name<T>> for [T; $n] {
            fn from(v: $name<T>) -> [T; $n] {
                v.0
            }
        }

        impl<T: std::marker::Copy> std::convert::From<[T; $n]> for $name<T> {
            fn from(v: [T; $n]) -> $name<T> {
                $name(v)
            }
        }

        impl<T> std::ops::Deref for $name<T> {
            type Target = [T];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $name<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

define_vec!(Vec2, 2);
define_vec!(Vec3, 3);
define_vec!(Vec4, 4);

macro_rules! define_mat {
    ($name:ident, $n:expr, $m:expr) => {
        #[doc = concat!($n, "x", $m, " matrix.")]
        #[derive(Copy, Clone, Default)]
        #[repr(C)]
        pub struct $name<T>([[T; $n]; $m]);

        impl<T> std::convert::From<$name<T>> for [[T; $n]; $m] {
            fn from(v: $name<T>) -> [[T; $n]; $m] {
                v.0
            }
        }

        impl<T: std::marker::Copy> std::convert::From<[[T; $n]; $m]> for $name<T> {
            fn from(v: [[T; $n]; $m]) -> $name<T> {
                $name(v)
            }
        }

        impl<T> std::ops::Deref for $name<T> {
            type Target = [[T; $n]];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $name<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<T> $name<T> {
            /// Returns a raw pointer to the matrix data.
            pub fn as_ptr(&self) -> *const T {
                self.0.as_ptr() as *const T
            }
        }
    };
}

define_mat!(Mat4, 4, 4);

// TODO: write a proper matrix arithmetic library.
impl ops::Mul<Mat4<f32>> for Mat4<f32> {
    type Output = Mat4<f32>;

    fn mul(self, rhs: Mat4<f32>) -> Self::Output {
        let mut result = Mat4::default();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self[i][k] * rhs[k][j]
                }
            }
        }
        result
    }
}

// TODO: write a proper matrix transformation library.
impl Mat4<f32> {
    /// Returns the identity matrix.
    pub fn identity() -> Mat4<f32> {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }

    /// Returns a scaling matrix.
    pub fn scale(x: f32, y: f32, z: f32) -> Mat4<f32> {
        [
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }

    /// Returns a translation matrix.
    pub fn translate(x: f32, y: f32, z: f32) -> Mat4<f32> {
        [
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }
}
