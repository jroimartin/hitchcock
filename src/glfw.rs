//! GLFW bindings.

use std::{
    ffi::{c_char, c_int, c_void, CStr, CString, NulError},
    fmt, ptr,
    sync::Mutex,
};

#[allow(dead_code, non_snake_case)]
mod ffi {
    use std::ffi::{c_char, c_int, c_void};

    #[link(name = "glfw")]
    extern "C" {
        pub fn glfwCreateWindow(
            width: c_int,
            height: c_int,
            title: *const c_char,
            monitor: *mut c_void,
            share: *mut c_void,
        ) -> *mut c_void;
        pub fn glfwGetProcAddress(procname: *const c_char) -> *const c_void;
        pub fn glfwInit() -> c_int;
        pub fn glfwMakeContextCurrent(window: *mut c_void);
        pub fn glfwPollEvents();
        pub fn glfwSetErrorCallback(callback: *const c_void) -> *const c_void;
        pub fn glfwSwapBuffers(window: *mut c_void);
        pub fn glfwTerminate();
        pub fn glfwWindowHint(hint: c_int, value: c_int);
        pub fn glfwWindowShouldClose(window: *mut c_void) -> c_int;
    }
}

/// Context client API major version hint and attribute.
pub const CONTEXT_VERSION_MAJOR: i32 = 0x00022002;

/// Context client API minor version hint and attribute.
pub const CONTEXT_VERSION_MINOR: i32 = 0x00022003;

/// OpenGL profile hint and attribute.
pub const OPENGL_PROFILE: i32 = 0x00022008;

/// Do not request a specific OpenGL profile.
pub const OPENGL_ANY_PROFILE: i32 = 0;

/// Request core OpenGL profile.
pub const OPENGL_CORE_PROFILE: i32 = 0x00032001;

/// Request forward-compatible OpenGL profile.
pub const OPENGL_COMPAT_PROFILE: i32 = 0x00032002;

/// A specialized result type for Glfw.
type Result<T> = std::result::Result<T, Error>;

/// The GLFW error type.
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

/// Opaque window object.
#[repr(transparent)]
pub struct Window(*mut c_void);

/// Opaque monitor object.
#[repr(transparent)]
pub struct Monitor(*mut c_void);

/// Generic function pointer used for returning client API function
/// pointers.
#[repr(transparent)]
pub struct GlProc(*const c_void);

unsafe impl Send for GlProc {}
unsafe impl Sync for GlProc {}

/// Error codes.
pub enum ErrorCode {
    /// No error has occurred.
    NoError,

    /// GLFW has not been initialized.
    NotInitialized,

    /// No context is current for this thread.
    NoCurrentContext,

    /// One of the arguments to the function was an invalid enum
    /// value.
    InvalidEnum,

    /// One of the arguments to the function was an invalid value.
    InvalidValue,

    /// A memory allocation failed.
    OutOfMemory,

    /// GLFW could not find support for the requested API on the
    /// system.
    ApiUnavailable,

    /// The requested OpenGL or OpenGL ES version is not available.
    VersionUnavailable,

    /// A platform-specific error occurred that does not match any of
    /// the more specific categories.
    PlatformError,

    /// The requested format is not supported or available.
    FormatUnavailable,

    /// The specified window does not have an OpenGL or OpenGL ES
    /// context.
    NoWindowContext,

    /// The specified cursor shape is not available.
    CursorUnavailable,

    /// The requested feature is not provided by the platform.
    FeatureUnavailable,

    /// The requested feature is not implemented for the platform.
    FeatureUnimplemented,

    /// Platform unavailable or no matching platform was found.
    PlatformUnavailable,

    /// Uknown error code.
    Unknown(i32),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::NoError => write!(f, "no error"),
            ErrorCode::NotInitialized => write!(f, "not initialized"),
            ErrorCode::NoCurrentContext => write!(f, "no current context"),
            ErrorCode::InvalidEnum => write!(f, "invalid enum"),
            ErrorCode::InvalidValue => write!(f, "invalid value"),
            ErrorCode::OutOfMemory => write!(f, "out of memory"),
            ErrorCode::ApiUnavailable => write!(f, "API unavailable"),
            ErrorCode::VersionUnavailable => write!(f, "version unavailable"),
            ErrorCode::PlatformError => write!(f, "platform error"),
            ErrorCode::FormatUnavailable => write!(f, "format unavailable"),
            ErrorCode::NoWindowContext => write!(f, "no window context"),
            ErrorCode::CursorUnavailable => write!(f, "cursor unavailable"),
            ErrorCode::FeatureUnavailable => write!(f, "feature unavailable"),
            ErrorCode::FeatureUnimplemented => write!(f, "feature unimplemented"),
            ErrorCode::PlatformUnavailable => write!(f, "platform unavailable"),
            ErrorCode::Unknown(code) => write!(f, "uknown error code ({code})"),
        }
    }
}

impl From<i32> for ErrorCode {
    fn from(code: i32) -> ErrorCode {
        match code {
            0 => ErrorCode::NoError,
            0x00010001 => ErrorCode::NotInitialized,
            0x00010002 => ErrorCode::NoCurrentContext,
            0x00010003 => ErrorCode::InvalidEnum,
            0x00010004 => ErrorCode::InvalidValue,
            0x00010005 => ErrorCode::OutOfMemory,
            0x00010006 => ErrorCode::ApiUnavailable,
            0x00010007 => ErrorCode::VersionUnavailable,
            0x00010008 => ErrorCode::PlatformError,
            0x00010009 => ErrorCode::FormatUnavailable,
            0x0001000A => ErrorCode::NoWindowContext,
            0x0001000B => ErrorCode::CursorUnavailable,
            0x0001000C => ErrorCode::FeatureUnavailable,
            0x0001000D => ErrorCode::FeatureUnimplemented,
            0x0001000E => ErrorCode::PlatformUnavailable,
            _ => ErrorCode::Unknown(code),
        }
    }
}

/// Initializes the GLFW library.
pub fn init() -> Result<()> {
    if unsafe { ffi::glfwInit() == 0 } {
        return Err(Error::Ffi);
    }
    Ok(())
}

/// Terminates the GLFW library.
pub fn terminate() {
    unsafe { ffi::glfwTerminate() }
}

/// Creates a window and its associated context.
pub fn create_window(
    width: i32,
    height: i32,
    title: &str,
    monitor: Option<&Monitor>,
    share: Option<&Window>,
) -> Result<Window> {
    let title = CString::new(title)?;
    let monitor = monitor.map_or(ptr::null_mut(), |m| m.0);
    let share = share.map_or(ptr::null_mut(), |w| w.0);
    let window = unsafe { ffi::glfwCreateWindow(width, height, title.as_ptr(), monitor, share) };
    if window.is_null() {
        return Err(Error::Ffi);
    }
    Ok(Window(window))
}

/// Returns the address of the specified function for the current
/// context.
pub fn get_proc_address(procname: &str) -> Result<GlProc> {
    let procname = CString::new(procname)?;
    let proc = unsafe { ffi::glfwGetProcAddress(procname.as_ptr()) };
    if proc.is_null() {
        return Err(Error::Ffi);
    }
    Ok(GlProc(proc))
}

/// Makes the context of the specified window current for the calling
/// thread.
pub fn make_context_current(window: &Window) {
    unsafe { ffi::glfwMakeContextCurrent(window.0) }
}

/// Processes all pending events.
pub fn poll_events() {
    unsafe { ffi::glfwPollEvents() }
}

type FnError = fn(error_code: ErrorCode, description: &str);

static ERROR_CALLBACK: Mutex<Option<FnError>> = Mutex::new(None);

extern "C" fn error_callback(error_code: c_int, description: *const c_char) {
    let cb = ERROR_CALLBACK
        .lock()
        .unwrap()
        .expect("GLFW error callback is not set");
    let description = unsafe { CStr::from_ptr(description) }
        .to_str()
        .expect("GLFW error description is not a valid UTF-8 string");
    cb(error_code.into(), description);
}

/// Sets the error callback.
pub fn set_error_callback(callback: Option<FnError>) {
    *ERROR_CALLBACK.lock().unwrap() = callback;
    let cb = if callback.is_some() {
        error_callback as *const c_void
    } else {
        ptr::null()
    };
    unsafe { ffi::glfwSetErrorCallback(cb) };
}

/// Swaps the front and back buffers of the specified window.
pub fn swap_buffers(window: &Window) {
    unsafe { ffi::glfwSwapBuffers(window.0) }
}

/// Sets the specified window hint to the desired value.
pub fn window_hint(hint: i32, value: i32) {
    unsafe { ffi::glfwWindowHint(hint, value) }
}

/// Checks the close flag of the specified window.
pub fn window_should_close(window: &Window) -> bool {
    unsafe { ffi::glfwWindowShouldClose(window.0) != 0 }
}
