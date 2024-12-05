//! GLFW bindings.

use std::{
    ffi::{c_void, CString, NulError},
    ptr,
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
