//! OpenGL bindings.

use std::{
    ffi::{c_void, CStr},
    ptr,
    sync::Mutex,
};

#[allow(dead_code, non_snake_case)]
mod ffi {
    use std::ffi::{c_char, c_double, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

    pub type GLenum = c_uint;
    pub type GLboolean = c_uchar;
    pub type GLbitfield = c_uint;
    pub type GLvoid = c_void;
    pub type GLbyte = c_char;
    pub type GLchar = c_char;
    pub type GLshort = c_short;
    pub type GLint = c_int;
    pub type GLubyte = c_uchar;
    pub type GLuchar = c_uchar;
    pub type GLushort = c_ushort;
    pub type GLuint = c_uint;
    pub type GLsizei = c_int;
    pub type GLfloat = c_float;
    pub type GLclampf = c_float;
    pub type GLdouble = c_double;
    pub type GLclampd = c_double;

    macro_rules! glfn {
        ($name:ident, $once:ident, $ret:ty $(, $pname:ident: $ptype:ty)*) => {
            static $once: std::sync::OnceLock<fn($($ptype),*) -> $ret> = std::sync::OnceLock::new();
            pub unsafe fn $name($($pname: $ptype),*) -> $ret {
                let f = $once.get_or_init(|| unsafe {
                    std::mem::transmute::<crate::glfw::GlProc, fn($($ptype),*) -> $ret>(
                        crate::glfw::get_proc_address(stringify!($name)).expect("failed to get proc address"),
                    )
                });
                f($($pname),*)
            }
        }
    }

    glfn![glClear, GL_CLEAR, (), mask: GLbitfield];
    glfn![glClearColor, GL_CLEAR_COLOR, (), red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf];
    glfn![glDebugMessageCallback, GL_DEBUG_MESSAGE_CALLBACK, (), callback: *const c_void, userParam: *const c_void];
    glfn![glEnable, GL_ENABLE, (), cap: GLenum];
    glfn![glGetError, GL_GET_ERROR, GLenum];
    glfn![glViewport, GL_VIEWPORT, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei];
}

/// Indicates the buffers currently enabled for color writing.
pub const COLOR_BUFFER_BIT: u32 = 0x00004000;

/// If enabled, debug messages are produced by a debug context.
pub const DEBUG_OUTPUT: u32 = 0x92e0;

/// Clears buffers to preset values.
pub fn clear(mask: u32) {
    unsafe { ffi::glClear(mask) }
}

/// Specifies clear values for the color buffers.
pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { ffi::glClearColor(red, green, blue, alpha) }
}

type FnError = fn(source: u32, typ: u32, id: u32, severity: u32, length: i32, message: &str);

static ERROR_CALLBACK: Mutex<Option<FnError>> = Mutex::new(None);

extern "C" fn error_callback(
    source: ffi::GLenum,
    typ: ffi::GLenum,
    id: ffi::GLuint,
    severity: ffi::GLenum,
    length: ffi::GLsizei,
    message: *const ffi::GLchar,
    _user_param: *const c_void,
) {
    let cb = ERROR_CALLBACK
        .lock()
        .unwrap()
        .expect("GL error callback is not set");
    let message = unsafe { CStr::from_ptr(message) }
        .to_str()
        .expect("GL error message is not a valid UTF-8 string");
    cb(source, typ, id, severity, length, message);
}

/// Specifies a callback to receive debugging messages from the GL.
pub fn debug_message_callback(callback: FnError) {
    *ERROR_CALLBACK.lock().unwrap() = Some(callback);
    unsafe { ffi::glDebugMessageCallback(error_callback as *const c_void, ptr::null()) }
}

/// Enables server-side GL capabilities.
pub fn enable(cap: u32) {
    unsafe { ffi::glEnable(cap) }
}

/// Returns the value of the error flag.
pub fn get_error() -> u32 {
    unsafe { ffi::glGetError() }
}

/// Sets the viewport.
pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { ffi::glViewport(x, y, width, height) }
}
