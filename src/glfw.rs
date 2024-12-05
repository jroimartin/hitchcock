//! GLFW bindings.

use std::ffi::{c_void, CString, NulError};

#[repr(C)]
pub struct Window {
    _data: [u8; 0],
    _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct Monitor {
    _data: [u8; 0],
    _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}

#[repr(transparent)]
pub struct GlProc(*const c_void);

unsafe impl Send for GlProc {}
unsafe impl Sync for GlProc {}

mod ffi {
    use std::ffi::{c_char, c_int, c_void};

    use super::{Monitor, Window};

    #[link(name = "glfw")]
    extern "C" {
        pub fn glfwCreateWindow(
            width: c_int,
            height: c_int,
            title: *const c_char,
            monitor: *mut Monitor,
            share: *mut Window,
        ) -> *mut Window;
        pub fn glfwGetProcAddress(procname: *const c_char) -> *const c_void;
        pub fn glfwInit() -> c_int;
        pub fn glfwMakeContextCurrent(window: *mut Window);
        pub fn glfwPollEvents();
        pub fn glfwSwapBuffers(window: *mut Window);
        pub fn glfwTerminate();
        pub fn glfwWindowHint(hint: c_int, value: c_int);
        pub fn glfwWindowShouldClose(window: *mut Window) -> c_int;
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Ffi,
    InvalidCString(NulError),
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Error {
        Error::InvalidCString(err)
    }
}

pub const CONTEXT_VERSION_MAJOR: i32 = 0x00022002;
pub const CONTEXT_VERSION_MINOR: i32 = 0x00022003;
pub const OPENGL_PROFILE: i32 = 0x00022008;

pub const OPENGL_ANY_PROFILE: i32 = 0;
pub const OPENGL_CORE_PROFILE: i32 = 0x00032001;
pub const OPENGL_COMPAT_PROFILE: i32 = 0x00032002;

pub fn init() -> Result<()> {
    if unsafe { ffi::glfwInit() == 0 } {
        return Err(Error::Ffi);
    }
    Ok(())
}

pub fn terminate() {
    unsafe { ffi::glfwTerminate() }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn create_window(
    width: i32,
    height: i32,
    title: &str,
    monitor: *mut Monitor,
    share: *mut Window,
) -> Result<*mut Window> {
    let title = CString::new(title)?;
    let window = unsafe { ffi::glfwCreateWindow(width, height, title.as_ptr(), monitor, share) };
    if window.is_null() {
        return Err(Error::Ffi);
    }
    Ok(window)
}

pub fn get_proc_address(procname: &str) -> Result<GlProc> {
    let procname = CString::new(procname)?;
    let proc = unsafe { ffi::glfwGetProcAddress(procname.as_ptr()) };
    if proc.is_null() {
        return Err(Error::Ffi);
    }
    Ok(GlProc(proc))
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn make_context_current(window: *mut Window) {
    unsafe { ffi::glfwMakeContextCurrent(window) }
}

pub fn poll_events() {
    unsafe { ffi::glfwPollEvents() }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn swap_buffers(window: *mut Window) {
    unsafe { ffi::glfwSwapBuffers(window) }
}

pub fn window_hint(hint: i32, value: i32) {
    unsafe { ffi::glfwWindowHint(hint, value) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn window_should_close(window: *mut Window) -> bool {
    unsafe { ffi::glfwWindowShouldClose(window) != 0 }
}
