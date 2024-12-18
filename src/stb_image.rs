//! stb_image bindings.

use std::{
    error,
    ffi::{c_int, c_void, CString, NulError},
    fmt,
    path::Path,
    result, slice,
};

#[allow(non_camel_case_types)]
mod ffi {
    use std::ffi::{c_char, c_int, c_uchar, c_void};

    pub type stbi_uc = c_uchar;

    #[link(name = "stb_image")]
    extern "C" {
        pub fn stbi_load(
            filename: *const c_char,
            x: *mut c_int,
            y: *mut c_int,
            channels_in_file: *mut c_int,
            desired_channels: c_int,
        ) -> *mut stbi_uc;
        pub fn stbi_image_free(retval_from_stbi_load: *mut c_void);
    }
}

/// Specialized result type.
pub type Result<T> = result::Result<T, Error>;

/// stb_image error.
#[derive(Debug)]
pub enum Error {
    /// Failed to load image.
    Load,

    /// Invalid UTF-8 string.
    InvalidUtf8,

    /// Invalid C string.
    InvalidCString(NulError),
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Error {
        Error::InvalidCString(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Load => write!(f, "failed to load image"),
            Error::InvalidUtf8 => write!(f, "invalid UTF-8 string"),
            Error::InvalidCString(err) => write!(f, "invalid C string: {err}"),
        }
    }
}

impl error::Error for Error {}

/// Represents an image.
pub struct Image {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    channels: usize,
}

impl Image {
    /// Parses an image file.
    pub fn load<P: AsRef<Path>>(filename: P) -> Result<Image> {
        let filename = CString::new(filename.as_ref().to_str().ok_or(Error::InvalidUtf8)?)?;

        let mut c_width: c_int = 0;
        let mut c_height: c_int = 0;
        let mut c_channels: c_int = 0;

        let retval = unsafe {
            ffi::stbi_load(
                filename.as_ptr(),
                &mut c_width as *mut c_int,
                &mut c_height as *mut c_int,
                &mut c_channels as *mut c_int,
                0,
            )
        };
        if retval.is_null() {
            return Err(Error::Load);
        }

        let len = (c_width * c_height * c_channels) as usize;
        let pixels = unsafe { slice::from_raw_parts(retval, len).to_vec() };

        unsafe { ffi::stbi_image_free(retval as *mut c_void) };

        Ok(Image {
            pixels,
            width: c_width as usize,
            height: c_height as usize,
            channels: c_channels as usize,
        })
    }

    /// Returns the pixel data of the image.
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    /// Returns the image width in pixels.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the image height in pixels.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the number of image components.
    pub fn channels(&self) -> usize {
        self.channels
    }
}
