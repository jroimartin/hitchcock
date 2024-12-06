//! OpenGL bindings.

use std::{
    ffi::{c_void, CStr},
    mem, ptr,
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
    pub type GLsizeiptr = usize;

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

    glfn![glAttachShader, GL_ATTACH_SHADER, (), program: GLuint, shader: GLuint];
    glfn![glBindBuffer, GL_BIND_BUFFER, (), target: GLenum, buffer: GLuint];
    glfn![glBindVertexArray, GL_BIND_VERTEX_ARRAY, (), array: GLuint];
    glfn![glBufferData, GL_BUFFER_DATA, (), target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum];
    glfn![glClear, GL_CLEAR, (), mask: GLbitfield];
    glfn![glClearColor, GL_CLEAR_COLOR, (), red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf];
    glfn![glCompileShader, GL_COMPILE_SHADER, (), shader: GLuint];
    glfn![glCreateProgram, GL_CREATE_PROGRAM, GLuint];
    glfn![glCreateShader, GL_CREATE_SHADER, GLuint, shaderType: GLenum];
    glfn![glDebugMessageCallback, GL_DEBUG_MESSAGE_CALLBACK, (), callback: *const c_void, userParam: *const c_void];
    glfn![glDeleteBuffers, GL_DELETE_BUFFERS, (), n: GLsizei, buffers: *mut GLuint];
    glfn![glDeleteProgram, GL_DELETE_PROGRAM, (), program: GLuint];
    glfn![glDeleteShader, GL_DELETE_SHADER, (), shader: GLuint];
    glfn![glDeleteVertexArrays, GL_DELETE_VERTEX_ARRAYS, (), n: GLsizei, arrays: *mut GLuint];
    glfn![glDrawArrays, GL_DRAW_ARRAYS, (), mode: GLenum, first: GLint, count: GLsizei];
    glfn![glEnable, GL_ENABLE, (), cap: GLenum];
    glfn![glEnableVertexAttribArray, GL_ENABLE_VERTEX_ATTRIB_ARRAY, (), index: GLuint];
    glfn![glGenBuffers, GL_GEN_BUFFERS, (), n: GLsizei, buffers: *mut GLuint];
    glfn![glGenVertexArrays, GL_GEN_VERTEX_ARRAYS, (), n: GLsizei, arrays: *mut GLuint];
    glfn![glGetError, GL_GET_ERROR, GLenum];
    glfn![glLinkProgram, GL_LINK_PROGRAM, (), program: GLuint];
    glfn![glShaderSource, GL_SHADER_SOURCE, (), shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint];
    glfn![glUseProgram, GL_USE_PROGRAM, (), program: GLuint];
    glfn![glVertexAttribPointer, GL_VERTEX_ATTRIB_POINTER, (), index: GLuint, size: GLint, typ: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void];
    glfn![glViewport, GL_VIEWPORT, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei];
}

/// Indicates the buffers currently enabled for color writing.
pub const COLOR_BUFFER_BIT: u32 = 0x00004000;

/// If enabled, debug messages are produced by a debug context.
pub const DEBUG_OUTPUT: u32 = 0x92e0;

/// Vertex attributes.
pub const ARRAY_BUFFER: u32 = 0x8892;

/// The data store contents are modified by the application, and used
/// as the source for GL drawing and image specification commands. The
/// data store contents will be modified once and used many times.
pub const STATIC_DRAW: u32 = 0x88E4;

/// Fragment shader type.
pub const FRAGMENT_SHADER: u32 = 0x8B30;

/// Vertext shader type.
pub const VERTEX_SHADER: u32 = 0x8B31;

/// Float data type.
pub const FLOAT: u32 = 0x1406;

/// Triangles primitive.
pub const TRIANGLES: u32 = 0x0004;

/// Attaches a shader object to a program object.
pub fn attach_shader(program: u32, shader: u32) {
    unsafe { ffi::glAttachShader(program, shader) }
}

/// Binds a named buffer object.
pub fn bind_buffer(target: u32, buffer: u32) {
    unsafe { ffi::glBindBuffer(target, buffer) }
}

/// Binds a vertex array object.
pub fn bind_vertex_array(array: u32) {
    unsafe { ffi::glBindVertexArray(array) }
}

/// Creates and initializes a buffer object's data store.
pub fn buffer_data<T>(target: u32, data: &[T], usage: u32) {
    unsafe {
        ffi::glBufferData(
            target,
            mem::size_of_val(data),
            data.as_ptr() as *const c_void,
            usage,
        )
    }
}

/// Clears buffers to preset values.
pub fn clear(mask: u32) {
    unsafe { ffi::glClear(mask) }
}

/// Specifies clear values for the color buffers.
pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { ffi::glClearColor(red, green, blue, alpha) }
}

/// Compiles a shader object.
pub fn compile_shader(shader: u32) {
    unsafe { ffi::glCompileShader(shader) }
}

/// Creates a program object.
pub fn create_program() -> u32 {
    unsafe { ffi::glCreateProgram() }
}

/// Creates a shader object.
pub fn create_shader(typ: u32) -> u32 {
    unsafe { ffi::glCreateShader(typ) }
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

/// Deletes named buffer objects.
pub fn delete_buffers(buffers: Vec<u32>) {
    unsafe { ffi::glDeleteBuffers(buffers.len() as i32, buffers.as_ptr() as *mut u32) }
}

/// Deletes a program object.
pub fn delete_program(program: u32) {
    unsafe { ffi::glDeleteProgram(program) }
}

/// Deletes a shader object.
pub fn delete_shader(shader: u32) {
    unsafe { ffi::glDeleteShader(shader) }
}

/// Deletes vertex array objects.
pub fn delete_vertex_arrays(arrays: Vec<u32>) {
    unsafe { ffi::glDeleteVertexArrays(arrays.len() as i32, arrays.as_ptr() as *mut u32) }
}

/// Renders primitives from array data.
pub fn draw_arrays(mode: u32, first: i32, count: i32) {
    unsafe { ffi::glDrawArrays(mode, first, count) }
}

/// Enables server-side GL capabilities.
pub fn enable(cap: u32) {
    unsafe { ffi::glEnable(cap) }
}

/// Enables a generic vertex attribute array.
pub fn enable_vertex_attrib_array(index: u32) {
    unsafe { ffi::glEnableVertexAttribArray(index) }
}

/// Generates buffer object names.
pub fn gen_buffers(n: usize) -> Vec<u32> {
    let buffers = vec![0u32; n];
    unsafe { ffi::glGenBuffers(n as i32, buffers.as_ptr() as *mut ffi::GLuint) };
    buffers
}

/// Generates vertex array object names.
pub fn gen_vertex_arrays(n: usize) -> Vec<u32> {
    let arrays = vec![0u32; n];
    unsafe { ffi::glGenVertexArrays(n as i32, arrays.as_ptr() as *mut ffi::GLuint) };
    arrays
}

/// Returns the value of the error flag.
pub fn get_error() -> u32 {
    unsafe { ffi::glGetError() }
}

/// Links a program object.
pub fn link_program(program: u32) {
    unsafe { ffi::glLinkProgram(program) }
}

/// Replaces the source code in a shader object.
pub fn shader_source(shader: u32, sources: &[&str]) {
    let count = sources.len();
    let strings: Vec<*const ffi::GLchar> = sources
        .iter()
        .map(|s| s.as_ptr() as *const ffi::GLchar)
        .collect();
    let lengths: Vec<i32> = sources.iter().map(|s| s.len() as i32).collect();
    unsafe { ffi::glShaderSource(shader, count as i32, strings.as_ptr(), lengths.as_ptr()) }
}

/// Installs a program object as part of current rendering state.
pub fn use_program(program: u32) {
    unsafe { ffi::glUseProgram(program) }
}

/// Defines an array of generic vertex attribute data.
pub fn vertex_attrib_pointer(
    index: u32,
    size: usize,
    typ: u32,
    normalized: bool,
    stride: usize,
    pointer: usize,
) {
    let normalized = if normalized { 1 } else { 0 };
    unsafe {
        ffi::glVertexAttribPointer(
            index,
            size as i32,
            typ,
            normalized,
            stride as i32,
            pointer as *const c_void,
        )
    }
}

/// Sets the viewport.
pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { ffi::glViewport(x, y, width, height) }
}
