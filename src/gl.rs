//! OpenGL bindings.

#[allow(dead_code, non_snake_case)]
mod ffi {
    use std::ffi::{c_char, c_double, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

    pub type GLenum = c_uint;
    pub type GLboolean = c_uchar;
    pub type GLbitfield = c_uint;
    pub type GLvoid = c_void;
    pub type GLbyte = c_char;
    pub type GLshort = c_short;
    pub type GLint = c_int;
    pub type GLubyte = c_uchar;
    pub type GLushort = c_ushort;
    pub type GLuint = c_uint;
    pub type GLsizei = c_int;
    pub type GLfloat = c_float;
    pub type GLclampf = c_float;
    pub type GLdouble = c_double;
    pub type GLclampd = c_double;

    macro_rules! glfn {
        ($name:ident, $once:ident, $ret:ty $(, $param:ident: $type:ty)*) => {
            static $once: std::sync::OnceLock<fn($($type),*)> = std::sync::OnceLock::new();
            pub unsafe fn $name($($param: $type),*) -> $ret {
                let f = $once.get_or_init(|| unsafe {
                    std::mem::transmute::<crate::glfw::GlProc, fn($($type),*) -> $ret>(
                        crate::glfw::get_proc_address(stringify!($name)).expect("failed to get proc address"),
                    )
                });
                f($($param),*)
            }
        }
    }

    glfn![glClear, GL_CLEAR, (), mask: GLbitfield];
    glfn![glClearColor, GL_CLEAR_COLOR, (), red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf];
    glfn![glViewport, GL_VIEWPORT, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei];
}

/// Indicates the buffers currently enabled for color writing.
pub const COLOR_BUFFER_BIT: u32 = 0x00004000;

/// Sets the viewport.
pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { ffi::glViewport(x, y, width, height) }
}

/// Specifies clear values for the color buffers.
pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { ffi::glClearColor(red, green, blue, alpha) }
}

/// Clears buffers to preset values.
pub fn clear(mask: u32) {
    unsafe { ffi::glClear(mask) }
}
