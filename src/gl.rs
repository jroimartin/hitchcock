//! OpenGL bindings.

use std::{
    error,
    ffi::{c_void, CStr, CString, NulError},
    fmt, mem, ptr, result,
    sync::Mutex,
};

use crate::{macros::define_enum, Vec4};

#[allow(non_snake_case)]
mod ffi {
    use std::ffi::{c_char, c_float, c_int, c_uchar, c_uint, c_void};

    pub type GLenum = c_uint;
    pub type GLboolean = c_uchar;
    pub type GLbitfield = c_uint;
    pub type GLchar = c_char;
    pub type GLint = c_int;
    pub type GLuint = c_uint;
    pub type GLsizei = c_int;
    pub type GLsizeiptr = usize;
    pub type GLfloat = c_float;

    macro_rules! glfn {
        ($name:ident, $once:ident, $ret:ty $(, $pname:ident: $ptype:ty)*) => {
            static $once: std::sync::OnceLock<fn($($ptype),*) -> $ret> = std::sync::OnceLock::new();
            pub unsafe fn $name($($pname: $ptype),*) -> $ret {
                let f = $once.get_or_init(|| unsafe {
                    std::mem::transmute::<crate::glfw::GlProc, fn($($ptype),*) -> $ret>(
                        crate::glfw::get_proc_address(stringify!($name)).expect("failed to get OpenGL proc address"),
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
    glfn![glClearColor, GL_CLEAR_COLOR, (), red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat];
    glfn![glCompileShader, GL_COMPILE_SHADER, (), shader: GLuint];
    glfn![glCreateProgram, GL_CREATE_PROGRAM, GLuint];
    glfn![glCreateShader, GL_CREATE_SHADER, GLuint, typ: GLenum];
    glfn![glDebugMessageCallback, GL_DEBUG_MESSAGE_CALLBACK, (), callback: *const c_void, user_param: *const c_void];
    glfn![glDeleteBuffers, GL_DELETE_BUFFERS, (), n: GLsizei, buffers: *const GLuint];
    glfn![glDeleteProgram, GL_DELETE_PROGRAM, (), program: GLuint];
    glfn![glDeleteShader, GL_DELETE_SHADER, (), shader: GLuint];
    glfn![glDeleteVertexArrays, GL_DELETE_VERTEX_ARRAYS, (), n: GLsizei, arrays: *const GLuint];
    glfn![glDrawArrays, GL_DRAW_ARRAYS, (), mode: GLenum, first: GLint, count: GLsizei];
    glfn![glDrawElements, GL_DRAW_ELEMENTS, (), mode: GLenum, count: GLsizei, typ: GLenum, indices: *const c_void];
    glfn![glEnable, GL_ENABLE, (), cap: GLenum];
    glfn![glEnableVertexAttribArray, GL_ENABLE_VERTEX_ATTRIB_ARRAY, (), index: GLuint];
    glfn![glGenBuffers, GL_GEN_BUFFERS, (), n: GLsizei, buffers: *mut GLuint];
    glfn![glGenVertexArrays, GL_GEN_VERTEX_ARRAYS, (), n: GLsizei, arrays: *mut GLuint];
    glfn![glGetError, GL_GET_ERROR, GLenum];
    glfn![glGetUniformLocation, GL_GET_UNIFORM_LOCATION, GLint, program: GLuint, name: *const GLchar];
    glfn![glLinkProgram, GL_LINK_PROGRAM, (), program: GLuint];
    glfn![glShaderSource, GL_SHADER_SOURCE, (), shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint];
    glfn![glUniform4f, GL_UNIFORM4F, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat];
    glfn![glUseProgram, GL_USE_PROGRAM, (), program: GLuint];
    glfn![glVertexAttribPointer, GL_VERTEX_ATTRIB_POINTER, (), index: GLuint, size: GLint, typ: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void];
    glfn![glViewport, GL_VIEWPORT, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei];
}

/// Indicates the buffers currently enabled for color writing.
pub const COLOR_BUFFER_BIT: u32 = 0x00004000;

/// If enabled, debug messages are produced by a debug context.
pub const DEBUG_OUTPUT: u32 = 0x92e0;

/// Vertex data.
pub const ARRAY_BUFFER: u32 = 0x8892;

/// Indices used for indexed rendering.
pub const ELEMENT_ARRAY_BUFFER: u32 = 0x8893;

/// The data store contents are modified by the application, and used
/// as the source for GL drawing and image specification commands. The
/// data store contents will be modified once and used many times.
pub const STATIC_DRAW: u32 = 0x88E4;

/// Fragment shader type.
pub const FRAGMENT_SHADER: u32 = 0x8B30;

/// Vertext shader type.
pub const VERTEX_SHADER: u32 = 0x8B31;

/// Unsigned integer data type.
pub const UNSIGNED_INT: u32 = 0x1405;

/// Float data type.
pub const FLOAT: u32 = 0x1406;

/// Triangles primitive.
pub const TRIANGLES: u32 = 0x0004;

/// A specialized result type.
pub type Result<T> = result::Result<T, Error>;

/// OpenGL error.
#[derive(Debug)]
pub enum Error {
    /// Non-active uniform variable in program.
    NonActiveUniform(String),

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
            Error::NonActiveUniform(s) => write!(f, "non-active uniform variable in program: {s}"),
            Error::InvalidCString(err) => write!(f, "invalid C string: {err}"),
        }
    }
}

impl error::Error for Error {}

/// Shader object.
#[derive(Clone, Copy)]
pub struct Shader(ffi::GLuint);

/// Program object.
#[derive(Clone, Copy)]
pub struct Program(ffi::GLuint);

/// Vertex array object.
#[derive(Clone, Copy)]
pub struct VertexArray(ffi::GLuint);

impl VertexArray {
    /// Returns the reserved vertex array zero.
    pub fn zero() -> VertexArray {
        VertexArray(0)
    }
}

/// Buffer object.
#[derive(Clone, Copy)]
pub struct Buffer(ffi::GLuint);

impl Buffer {
    /// Returns the reserved buffer object zero.
    pub fn zero() -> Buffer {
        Buffer(0)
    }
}

/// Uniform value.
pub enum Uniform {
    /// vec4 uniform.
    Vec4(ffi::GLfloat, ffi::GLfloat, ffi::GLfloat, ffi::GLfloat),
}

impl From<Vec4<f32>> for Uniform {
    fn from(v: Vec4<f32>) -> Uniform {
        Uniform::Vec4(v.x, v.y, v.z, v.w)
    }
}

/// Uniform location.
#[derive(Clone, Copy)]
pub struct UniformLocation(ffi::GLint);

define_enum! {
    pub enum DebugSource(u32, "Debug source") {
        Api            => (0x8246, "API"),
        WindowSystem   => (0x8247, "Window system"),
        ShaderCompiler => (0x8248, "Shader compiler"),
        ThirdParty     => (0x8249, "Third party"),
        Application    => (0x824a, "Application"),
        Other          => (0x824b, "Other"),
    }

    pub enum DebugType(u32, "Debug type") {
        Error              => (0x824c, "Error"),
        DeprecatedBehavior => (0x824d, "Deprecated behavior"),
        UndefinedBehavior  => (0x824e, "Undefined behavior"),
        Portability        => (0x824f, "Portability"),
        Performance        => (0x8250, "Performance"),
        Marker             => (0x8268, "Marker"),
        PushGroup          => (0x8269, "Push group"),
        PopGroup           => (0x826a, "Pop group"),
        Other              => (0x8251, "Other"),
    }

    pub enum DebugSeverity(u32, "Debug severity") {
        High         => (0x9146, "High"),
        Medium       => (0x9147, "Medium"),
        Low          => (0x9148, "Low"),
        Notification => (0x826b, "Notification"),
    }
}

/// Attaches a shader object to a program object.
pub fn attach_shader(program: Program, shader: Shader) {
    unsafe { ffi::glAttachShader(program.0, shader.0) }
}

/// Binds a named buffer object.
pub fn bind_buffer(target: u32, buffer: Buffer) {
    unsafe { ffi::glBindBuffer(target, buffer.0) }
}

/// Binds a vertex array object.
pub fn bind_vertex_array(array: VertexArray) {
    unsafe { ffi::glBindVertexArray(array.0) }
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
pub fn compile_shader(shader: Shader) {
    unsafe { ffi::glCompileShader(shader.0) }
}

/// Creates a program object.
pub fn create_program() -> Program {
    let program = unsafe { ffi::glCreateProgram() };
    Program(program)
}

/// Creates a shader object.
pub fn create_shader(typ: u32) -> Shader {
    let shader = unsafe { ffi::glCreateShader(typ) };
    Shader(shader)
}

/// Debug callback.
pub type FnDebug =
    fn(source: DebugSource, typ: DebugType, id: u32, severity: DebugSeverity, message: &str);

static DEBUG_CALLBACK: Mutex<Option<FnDebug>> = Mutex::new(None);

extern "C" fn debug_callback(
    source: ffi::GLenum,
    typ: ffi::GLenum,
    id: ffi::GLuint,
    severity: ffi::GLenum,
    _length: ffi::GLsizei,
    message: *const ffi::GLchar,
    _user_param: *const c_void,
) {
    let cb = DEBUG_CALLBACK
        .lock()
        .unwrap()
        .expect("GL error callback is not set");
    let message = unsafe { CStr::from_ptr(message) }
        .to_str()
        .expect("GL error message is not a valid UTF-8 string");
    cb(source.into(), typ.into(), id, severity.into(), message);
}

/// Specifies a callback to receive debugging messages from the GL.
pub fn debug_message_callback(callback: FnDebug) {
    *DEBUG_CALLBACK.lock().unwrap() = Some(callback);
    unsafe { ffi::glDebugMessageCallback(debug_callback as *const c_void, ptr::null()) }
}

/// Deletes named buffer objects.
pub fn delete_buffers(buffers: &[Buffer]) {
    unsafe {
        ffi::glDeleteBuffers(
            buffers.len() as ffi::GLsizei,
            buffers.as_ptr() as *const ffi::GLuint,
        )
    }
}

/// Deletes a program object.
pub fn delete_program(program: Program) {
    unsafe { ffi::glDeleteProgram(program.0) }
}

/// Deletes a shader object.
pub fn delete_shader(shader: Shader) {
    unsafe { ffi::glDeleteShader(shader.0) }
}

/// Deletes vertex array objects.
pub fn delete_vertex_arrays(arrays: &[VertexArray]) {
    unsafe {
        ffi::glDeleteVertexArrays(
            arrays.len() as ffi::GLsizei,
            arrays.as_ptr() as *const ffi::GLuint,
        )
    }
}

/// Renders primitives from array data.
pub fn draw_arrays(mode: u32, first: i32, count: i32) {
    unsafe { ffi::glDrawArrays(mode, first, count) }
}

/// Renders primitives from array data using the provided indices.
pub fn draw_elements(mode: u32, count: usize, typ: u32, indices: usize) {
    unsafe { ffi::glDrawElements(mode, count as ffi::GLsizei, typ, indices as *const c_void) }
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
pub fn gen_buffers(n: usize) -> Vec<Buffer> {
    let buffers = vec![Buffer::zero(); n];
    unsafe { ffi::glGenBuffers(n as ffi::GLsizei, buffers.as_ptr() as *mut ffi::GLuint) };
    buffers
}

/// Generates vertex array object names.
pub fn gen_vertex_arrays(n: usize) -> Vec<VertexArray> {
    let arrays = vec![VertexArray::zero(); n];
    unsafe { ffi::glGenVertexArrays(n as ffi::GLsizei, arrays.as_ptr() as *mut ffi::GLuint) };
    arrays
}

/// Returns the value of the error flag.
pub fn get_error() -> u32 {
    unsafe { ffi::glGetError() }
}

/// Returns the location of a uniform variable.
pub fn get_uniform_location(program: Program, name: &str) -> Result<UniformLocation> {
    let cname = CString::new(name)?;
    let loc = unsafe { ffi::glGetUniformLocation(program.0, cname.as_ptr() as *const ffi::GLchar) };
    if loc == -1 {
        return Err(Error::NonActiveUniform(name.into()));
    }
    Ok(UniformLocation(loc))
}

/// Links a program object.
pub fn link_program(program: Program) {
    unsafe { ffi::glLinkProgram(program.0) }
}

/// Replaces the source code in a shader object.
pub fn shader_source(shader: Shader, sources: &[&str]) -> Result<()> {
    let count = sources.len();
    let csources = sources
        .iter()
        .map(|s| CString::new(*s).map_err(|err| err.into()))
        .collect::<Result<Vec<CString>>>()?;
    let strings: Vec<*const ffi::GLchar> = csources.iter().map(|s| s.as_ptr()).collect();
    let lengths: Vec<i32> = sources.iter().map(|s| s.len() as ffi::GLint).collect();
    unsafe {
        ffi::glShaderSource(
            shader.0,
            count as ffi::GLsizei,
            strings.as_ptr(),
            lengths.as_ptr(),
        )
    };
    Ok(())
}

/// Specify the value of a uniform variable for the current program
/// object.
pub fn uniform(location: UniformLocation, uniform: Uniform) {
    match uniform {
        Uniform::Vec4(v0, v1, v2, v3) => unsafe {
            ffi::glUniform4f(
                location.0,
                v0 as ffi::GLfloat,
                v1 as ffi::GLfloat,
                v2 as ffi::GLfloat,
                v3 as ffi::GLfloat,
            )
        },
    }
}

/// Installs a program object as part of current rendering state.
pub fn use_program(program: Program) {
    unsafe { ffi::glUseProgram(program.0) }
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
            size as ffi::GLint,
            typ,
            normalized,
            stride as ffi::GLsizei,
            pointer as *const c_void,
        )
    }
}

/// Sets the viewport.
pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { ffi::glViewport(x, y, width, height) }
}
