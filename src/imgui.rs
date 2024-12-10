//! Dear ImGui bindings.

use std::{
    error,
    ffi::{c_float, c_uchar, c_void, CString, NulError},
    fmt, ptr, result,
};

use crate::macros::define_opaque;

mod ffi {
    use std::ffi::{c_char, c_float, c_int, c_uchar, c_void};

    pub type ImGuiWindowFlags = c_int;
    pub type ImGuiSliderFlags = c_int;

    extern "C" {
        pub fn igBegin(
            name: *const c_char,
            p_open: *mut c_uchar,
            flags: ImGuiWindowFlags,
        ) -> c_uchar;
        pub fn igCheckbox(label: *const c_char, v: *mut c_uchar) -> c_uchar;
        pub fn igCreateContext(shared_font_atlas: *mut c_void) -> *mut c_void;
        pub fn igDestroyContext(ctx: *mut c_void);
        pub fn igEnd();
        pub fn igGetDrawData() -> *mut c_void;
        pub fn igNewFrame();
        pub fn igRender();
        pub fn igShowDemoWindow(p_open: *mut c_uchar);
        pub fn igSliderFloat(
            label: *const c_char,
            v: *mut c_float,
            v_min: c_float,
            v_max: c_float,
            format: *const c_char,
            flags: ImGuiSliderFlags,
        ) -> c_uchar;
        pub fn igText(fmt: *const c_char, ...);
    }
}

/// A specialized result type.
type Result<T> = result::Result<T, Error>;

/// Dear ImGui error.
#[derive(Debug)]
pub enum Error {
    /// Error when calling `ImGui_ImplGlfw_InitForOpenGL`.
    ImGuiImplGlfwInitForOpenGL,

    /// Error when calling `ImGui_ImplOpenGL3_Init`.
    ImGuiImplOpenGL3Init,

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
            Error::ImGuiImplGlfwInitForOpenGL => {
                write!(f, "Failed to initialize ImGui GLFW backend")
            }
            Error::ImGuiImplOpenGL3Init => write!(f, "Failed to initialize ImGui OpenGL backend"),
            Error::InvalidCString(_) => write!(f, "Invalid C string"),
        }
    }
}

impl error::Error for Error {}

define_opaque! {
    pub opaque Context(mut);
    pub opaque FontAtlas(mut);
    pub opaque DrawData(mut);
}

/// Pushes a new window to the stack to start appending widgets to
/// it. If `open` is [`Option::Some`], it shows a window-closing
/// widget in the upper-right corner of the window, which clicking
/// will set the boolean to false when clicked. The function returns
/// false if the window is collapsed.
pub fn begin(name: &str, open: Option<&mut bool>, flags: u32) -> Result<bool> {
    let name = CString::new(name)?;
    let flags = flags as ffi::ImGuiWindowFlags;

    let unfolded = match open {
        Some(open) => {
            let mut ig_open: c_uchar = if *open { 1 } else { 0 };
            let unfolded =
                unsafe { ffi::igBegin(name.as_ptr(), &mut ig_open as *mut c_uchar, flags) };
            *open = ig_open != 0;
            unfolded
        }
        None => unsafe { ffi::igBegin(name.as_ptr(), ptr::null_mut(), flags) },
    };
    Ok(unfolded != 0)
}

/// Adds a checkbox widget. `checked` reports whether the checkbox is
/// checked. The function returns whether the checkbox has changed.
pub fn checkbox(label: &str, checked: &mut bool) -> Result<bool> {
    let label = CString::new(label)?;
    let mut ig_checked: c_uchar = if *checked { 1 } else { 0 };
    let changed = unsafe { ffi::igCheckbox(label.as_ptr(), &mut ig_checked as *mut c_uchar) };
    *checked = ig_checked != 0;
    Ok(changed != 0)
}

/// Creates a context.
pub fn create_context(font_atlas: Option<FontAtlas>) -> Context {
    let font_atlas = font_atlas.map_or(ptr::null_mut(), |fa| fa.as_mut_ptr());
    let ctx = unsafe { ffi::igCreateContext(font_atlas) };
    Context(ctx)
}

/// Destroys the specified context. If [`Option::None`], destroy
/// current context.
pub fn destroy_context(ctx: Option<Context>) {
    let ctx = ctx.map_or(ptr::null_mut(), |c| c.as_mut_ptr());
    unsafe { ffi::igDestroyContext(ctx) };
}

/// Pop window from the stack.
pub fn end() {
    unsafe { ffi::igEnd() }
}

/// Returns the draw data required to render a frame.
pub fn get_draw_data() -> DrawData {
    let draw_data = unsafe { ffi::igGetDrawData() };
    DrawData(draw_data)
}

/// Starts a new frame.
pub fn new_frame() {
    unsafe { ffi::igNewFrame() }
}

/// Renders a frame.
pub fn render() {
    unsafe { ffi::igRender() }
}

/// Shows the Deam ImGui demo window. If `open` is [`Option::Some`],
/// it shows a window-closing widget in the upper-right corner of the
/// window, which clicking will set the boolean to false when
/// clicked.
pub fn show_demo_window(open: Option<&mut bool>) {
    match open {
        Some(open) => {
            let mut ig_open: c_uchar = if *open { 1 } else { 0 };
            unsafe { ffi::igShowDemoWindow(&mut ig_open as *mut c_uchar) };
            *open = ig_open != 0;
        }
        None => unsafe { ffi::igShowDemoWindow(ptr::null_mut()) },
    }
}

/// Adds a slider float widget. `v` reports the selected value. The
/// function returns whether the slider value has changed.
pub fn slider_float(
    label: &str,
    v: &mut f32,
    min: f32,
    max: f32,
    format: &str,
    flags: u32,
) -> Result<bool> {
    let label = CString::new(label)?;
    let format = CString::new(format)?;
    let flags = flags as ffi::ImGuiSliderFlags;

    let changed = unsafe {
        ffi::igSliderFloat(
            label.as_ptr(),
            v as *mut c_float,
            min,
            max,
            format.as_ptr(),
            flags,
        )
    };
    Ok(changed != 0)
}

/// Adds a text widget.
pub fn text(s: &str) -> Result<()> {
    let s = CString::new(s)?;
    unsafe { ffi::igText(s.as_ptr()) };
    Ok(())
}

/// Dear ImGui GLFW backend.
pub mod glfw {
    use super::{Error, Result};

    mod ffi {
        use std::ffi::{c_int, c_void};

        extern "C" {
            pub fn ImGui_ImplGlfw_InitForOpenGL(
                window: *mut c_void,
                install_callbacks: c_int,
            ) -> c_int;
            pub fn ImGui_ImplGlfw_NewFrame();
            pub fn ImGui_ImplGlfw_Shutdown();
        }
    }

    /// Initializes the GLFW backend for OpenGL.
    pub fn init_for_opengl(window: crate::glfw::Window, install_callbacks: bool) -> Result<()> {
        let install_callbacks = if install_callbacks { 1 } else { 0 };
        let retval =
            unsafe { ffi::ImGui_ImplGlfw_InitForOpenGL(window.as_mut_ptr(), install_callbacks) };
        if retval == 0 {
            Err(Error::ImGuiImplGlfwInitForOpenGL)
        } else {
            Ok(())
        }
    }

    /// Starts a frame.
    pub fn new_frame() {
        unsafe { ffi::ImGui_ImplGlfw_NewFrame() }
    }

    /// Shutdowns the GLFW backend.
    pub fn shutdown() {
        unsafe { ffi::ImGui_ImplGlfw_Shutdown() }
    }
}

/// Dear ImGui OpenGL backend.
pub mod opengl {
    use std::ffi::CString;

    use super::{DrawData, Error, Result};

    mod ffi {
        use std::ffi::{c_char, c_int, c_void};

        extern "C" {
            pub fn ImGui_ImplOpenGL3_Init(glsl_version: *const c_char) -> c_int;
            pub fn ImGui_ImplOpenGL3_NewFrame();
            pub fn ImGui_ImplOpenGL3_RenderDrawData(draw_data: *mut c_void);
            pub fn ImGui_ImplOpenGL3_Shutdown();
        }
    }

    /// Initializes the OpenGL backend.
    pub fn init(glsl_version: &str) -> Result<()> {
        let glsl_version = CString::new(glsl_version)?;
        let retval = unsafe { ffi::ImGui_ImplOpenGL3_Init(glsl_version.as_ptr()) };
        if retval == 0 {
            Err(Error::ImGuiImplOpenGL3Init)
        } else {
            Ok(())
        }
    }

    /// Starts a frame.
    pub fn new_frame() {
        unsafe { ffi::ImGui_ImplOpenGL3_NewFrame() }
    }

    /// Renders draw data.
    pub fn render_draw_data(draw_data: DrawData) {
        unsafe { ffi::ImGui_ImplOpenGL3_RenderDrawData(draw_data.as_mut_ptr()) }
    }

    /// Shutdowns the OpenGL backend.
    pub fn shutdown() {
        unsafe { ffi::ImGui_ImplOpenGL3_Shutdown() }
    }
}
