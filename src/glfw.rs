//! GLFW bindings.

use std::{
    collections::HashMap,
    ffi::{c_char, c_int, c_void, CStr, CString, NulError},
    ptr,
    sync::{LazyLock, Mutex},
};

use crate::macros::define_enums;

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

/// Window object pointer.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Window(*mut c_void);

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

/// Monitor object pointer.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Monitor(*mut c_void);

unsafe impl Send for Monitor {}
unsafe impl Sync for Monitor {}

/// Generic function pointer used for returning client API function
/// pointers.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct GlProc(*const c_void);

unsafe impl Send for GlProc {}
unsafe impl Sync for GlProc {}

define_enums! {
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
    monitor: Option<Monitor>,
    share: Option<Window>,
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
pub fn make_context_current(window: Window) {
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

type FnFramebufferSize = fn(window: Window, width: i32, height: i32);

static FRAMEBUFFER_SIZE_CALLBACKS: LazyLock<Mutex<HashMap<Window, Option<FnFramebufferSize>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

extern "C" fn framebuffer_size_callback(window: *mut c_void, width: c_int, height: c_int) {
    let window = Window(window);
    let cb = FRAMEBUFFER_SIZE_CALLBACKS
        .lock()
        .unwrap()
        .get(&window)
        .expect("Unknown GLFW window")
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
    unsafe { ffi::glfwSetFramebufferSizeCallback(window.0, cb) };
}

/// Swaps the front and back buffers of the specified window.
pub fn swap_buffers(window: Window) {
    unsafe { ffi::glfwSwapBuffers(window.0) }
}

/// Sets the specified window hint to the desired value.
pub fn window_hint(hint: i32, value: i32) {
    unsafe { ffi::glfwWindowHint(hint, value) }
}

/// Checks the close flag of the specified window.
pub fn window_should_close(window: Window) -> bool {
    unsafe { ffi::glfwWindowShouldClose(window.0) != 0 }
}
