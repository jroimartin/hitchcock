//! GLFW bindings.

use std::{
    collections::HashMap,
    error,
    ffi::{c_char, c_int, c_void, CStr, CString, NulError},
    fmt, ptr, result,
    sync::{LazyLock, Mutex},
};

use crate::macros::{define_enum, define_opaque};

#[allow(non_snake_case)]
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
        pub fn glfwSetFramebufferSizeCallback(
            window: *mut c_void,
            callback: *const c_void,
        ) -> *const c_void;
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

/// Request core OpenGL profile.
pub const OPENGL_CORE_PROFILE: i32 = 0x00032001;

/// A specialized result type.
pub type Result<T> = result::Result<T, Error>;

/// GLFW error.
#[derive(Debug)]
pub enum Error {
    /// Error when calling `glfwInit`.
    GlfwInit,

    /// Error when calling `glfwCreateWindow`.
    GlfwCreateWindow,

    /// Error when calling `glfwGetProcAddress`.
    GlfwGetProcAddress,

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
            Error::GlfwInit => write!(f, "failed to initialize GLFW"),
            Error::GlfwCreateWindow => write!(f, "failed to create GLFW window"),
            Error::GlfwGetProcAddress => write!(f, "failed to get function address"),
            Error::InvalidCString(err) => write!(f, "invalid C string: {err}"),
        }
    }
}

impl error::Error for Error {}

define_opaque! {
    pub opaque Window(mut);
    pub opaque Monitor(mut);
    pub opaque GlProc(const);
}

define_enum! {
    pub enum ErrorCode(i32, "Error codes") {
        NoError            => (0, "No error has occurred"),
        NotInitialized     => (0x00010001, "GLFW has not been initialized"),
        NoCurrentContext   => (0x00010002, "No context is current for this thread"),
        InvalidEnum        => (0x00010003, "One of the arguments to the function was an invalid enum value"),
        InvalidValue       => (0x00010004, "One of the arguments to the function was an invalid value"),
        OutOfMemory        => (0x00010005, "A memory allocation failed"),
        ApiUnavailable     => (0x00010006, "GLFW could not find support for the requested API on the system"),
        VersionUnavailable => (0x00010007, "The requested OpenGL or OpenGL ES version is not available"),
        PlatformError      => (0x00010008, "A platform-specific error occurred that does not match any of the more specific categories"),
        FormatUnavailable  => (0x00010009, "The requested format is not supported or available"),
        NoWindowContext    => (0x0001000a, "The specified window does not have an OpenGL or OpenGL ES context"),
    }
}

/// Initializes the GLFW library.
pub fn init() -> Result<()> {
    if unsafe { ffi::glfwInit() == 0 } {
        return Err(Error::GlfwInit);
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
    monitor: Option<Monitor>,
    share: Option<Window>,
) -> Result<Window> {
    let title = CString::new(title)?;
    let monitor = monitor.map_or(ptr::null_mut(), |m| m.as_mut_ptr());
    let share = share.map_or(ptr::null_mut(), |w| w.as_mut_ptr());
    let window = unsafe { ffi::glfwCreateWindow(width, height, title.as_ptr(), monitor, share) };
    if window.is_null() {
        return Err(Error::GlfwCreateWindow);
    }
    Ok(Window(window))
}

/// Returns the address of the specified function for the current
/// context.
pub fn get_proc_address(procname: &str) -> Result<GlProc> {
    let procname = CString::new(procname)?;
    let proc = unsafe { ffi::glfwGetProcAddress(procname.as_ptr()) };
    if proc.is_null() {
        return Err(Error::GlfwGetProcAddress);
    }
    Ok(GlProc(proc))
}

/// Makes the context of the specified window current for the calling
/// thread.
pub fn make_context_current(window: Window) {
    unsafe { ffi::glfwMakeContextCurrent(window.as_mut_ptr()) }
}

/// Processes all pending events.
pub fn poll_events() {
    unsafe { ffi::glfwPollEvents() }
}

/// Error callback.
pub type FnError = fn(error_code: ErrorCode, description: &str);

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

/// Framebuffer size change callback.
pub type FnFramebufferSize = fn(window: Window, width: i32, height: i32);

static FRAMEBUFFER_SIZE_CALLBACKS: LazyLock<Mutex<HashMap<Window, Option<FnFramebufferSize>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

extern "C" fn framebuffer_size_callback(window: *mut c_void, width: c_int, height: c_int) {
    let window = Window(window);
    let cb = FRAMEBUFFER_SIZE_CALLBACKS
        .lock()
        .unwrap()
        .get(&window)
        .expect("unknown GLFW window")
        .expect("GLFW framebuffer size callback is not set");
    cb(window, width, height);
}

/// Sets the framebuffer resize callback for the specified window.
pub fn set_framebuffer_size_callback(window: Window, callback: Option<FnFramebufferSize>) {
    FRAMEBUFFER_SIZE_CALLBACKS
        .lock()
        .unwrap()
        .insert(window, callback);
    let cb = if callback.is_some() {
        framebuffer_size_callback as *const c_void
    } else {
        ptr::null()
    };
    unsafe { ffi::glfwSetFramebufferSizeCallback(window.as_mut_ptr(), cb) };
}

/// Swaps the front and back buffers of the specified window.
pub fn swap_buffers(window: Window) {
    unsafe { ffi::glfwSwapBuffers(window.as_mut_ptr()) }
}

/// Sets the specified window hint to the desired value.
pub fn window_hint(hint: i32, value: i32) {
    unsafe { ffi::glfwWindowHint(hint, value) }
}

/// Checks the close flag of the specified window.
pub fn window_should_close(window: Window) -> bool {
    unsafe { ffi::glfwWindowShouldClose(window.as_mut_ptr()) != 0 }
}
