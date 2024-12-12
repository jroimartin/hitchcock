//! Dear ImGui bindings.

use std::{
    error,
    ffi::{c_float, c_uchar, CString, NulError},
    fmt, ptr, result,
};

use crate::macros::define_opaque;

#[allow(
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types,
    dead_code
)]
mod ffi {
    use std::ffi::{c_char, c_double, c_float, c_int, c_schar, c_uchar, c_uint, c_ushort, c_void};

    pub type ImGuiBackendFlags = c_int;
    pub type ImGuiConfigFlags = c_int;
    pub type ImGuiID = c_uint;
    pub type ImGuiKeyChord = c_int;
    pub type ImGuiMouseSource = c_int;
    pub type ImGuiSliderFlags = c_int;
    pub type ImGuiWindowFlags = c_int;
    pub type ImS8 = c_schar;
    pub type ImU16 = c_ushort;
    pub type ImWchar = ImWchar16;
    pub type ImWchar16 = c_ushort;

    pub const ImGuiKey_COUNT: c_int = 666;
    pub const ImGuiKey_KeysData_SIZE: c_int = ImGuiKey_NamedKey_COUNT;
    pub const ImGuiKey_NamedKey_BEGIN: c_int = 512;
    pub const ImGuiKey_NamedKey_END: c_int = ImGuiKey_COUNT;
    pub const ImGuiKey_NamedKey_COUNT: c_int = ImGuiKey_NamedKey_END - ImGuiKey_NamedKey_BEGIN;

    #[repr(C)]
    pub struct ImGuiIO {
        pub ConfigFlags: ImGuiConfigFlags,
        pub BackendFlags: ImGuiBackendFlags,
        pub DisplaySize: ImVec2,
        pub DeltaTime: c_float,
        pub IniSavingRate: c_float,
        pub IniFilename: *const c_char,
        pub LogFilename: *const c_char,
        pub UserData: *mut c_void,
        pub Fonts: *mut c_void,
        pub FontGlobalScale: c_float,
        pub FontAllowUserScaling: c_uchar,
        pub FontDefault: *mut c_void,
        pub DisplayFramebufferScale: ImVec2,
        pub ConfigNavSwapGamepadButtons: c_uchar,
        pub ConfigNavMoveSetMousePos: c_uchar,
        pub ConfigNavCaptureKeyboard: c_uchar,
        pub ConfigNavEscapeClearFocusItem: c_uchar,
        pub ConfigNavEscapeClearFocusWindow: c_uchar,
        pub ConfigNavCursorVisibleAuto: c_uchar,
        pub ConfigNavCursorVisibleAlways: c_uchar,
        pub ConfigDockingNoSplit: c_uchar,
        pub ConfigDockingWithShift: c_uchar,
        pub ConfigDockingAlwaysTabBar: c_uchar,
        pub ConfigDockingTransparentPayload: c_uchar,
        pub ConfigViewportsNoAutoMerge: c_uchar,
        pub ConfigViewportsNoTaskBarIcon: c_uchar,
        pub ConfigViewportsNoDecoration: c_uchar,
        pub ConfigViewportsNoDefaultParent: c_uchar,
        pub MouseDrawCursor: c_uchar,
        pub ConfigMacOSXBehaviors: c_uchar,
        pub ConfigInputTrickleEventQueue: c_uchar,
        pub ConfigInputTextCursorBlink: c_uchar,
        pub ConfigInputTextEnterKeepActive: c_uchar,
        pub ConfigDragClickToInputText: c_uchar,
        pub ConfigWindowsResizeFromEdges: c_uchar,
        pub ConfigWindowsMoveFromTitleBarOnly: c_uchar,
        pub ConfigScrollbarScrollByPage: c_uchar,
        pub ConfigMemoryCompactTimer: c_float,
        pub MouseDoubleClickTime: c_float,
        pub MouseDoubleClickMaxDist: c_float,
        pub MouseDragThreshold: c_float,
        pub KeyRepeatDelay: c_float,
        pub KeyRepeatRate: c_float,
        pub ConfigErrorRecovery: c_uchar,
        pub ConfigErrorRecoveryEnableAssert: c_uchar,
        pub ConfigErrorRecoveryEnableDebugLog: c_uchar,
        pub ConfigErrorRecoveryEnableTooltip: c_uchar,
        pub ConfigDebugIsDebuggerPresent: c_uchar,
        pub ConfigDebugHighlightIdConflicts: c_uchar,
        pub ConfigDebugBeginReturnValueOnce: c_uchar,
        pub ConfigDebugBeginReturnValueLoop: c_uchar,
        pub ConfigDebugIgnoreFocusLoss: c_uchar,
        pub ConfigDebugIniSettings: c_uchar,
        pub BackendPlatformName: *const c_char,
        pub BackendRendererName: *const c_char,
        pub BackendPlatformUserData: *mut c_void,
        pub BackendRendererUserData: *mut c_void,
        pub BackendLanguageUserData: *mut c_void,
        pub WantCaptureMouse: c_uchar,
        pub WantCaptureKeyboard: c_uchar,
        pub WantTextInput: c_uchar,
        pub WantSetMousePos: c_uchar,
        pub WantSaveIniSettings: c_uchar,
        pub NavActive: c_uchar,
        pub NavVisible: c_uchar,
        pub Framerate: c_float,
        pub MetricsRenderVertices: c_int,
        pub MetricsRenderIndices: c_int,
        pub MetricsRenderWindows: c_int,
        pub MetricsActiveWindows: c_int,
        pub MouseDelta: ImVec2,
        pub Ctx: *mut c_void,
        pub MousePos: ImVec2,
        pub MouseDown: [c_uchar; 5],
        pub MouseWheel: c_float,
        pub MouseWheelH: c_float,
        pub MouseSource: ImGuiMouseSource,
        pub MouseHoveredViewport: ImGuiID,
        pub KeyCtrl: c_uchar,
        pub KeyShift: c_uchar,
        pub KeyAlt: c_uchar,
        pub KeySuper: c_uchar,
        pub KeyMods: ImGuiKeyChord,
        pub KeysData: [ImGuiKeyData; ImGuiKey_KeysData_SIZE as usize],
        pub WantCaptureMouseUnlessPopupClose: c_uchar,
        pub MousePosPrev: ImVec2,
        pub MouseClickedPos: [ImVec2; 5],
        pub MouseClickedTime: [c_double; 5],
        pub MouseClicked: [c_uchar; 5],
        pub MouseDoubleClicked: [c_uchar; 5],
        pub MouseClickedCount: [ImU16; 5],
        pub MouseClickedLastCount: [ImU16; 5],
        pub MouseReleased: [c_uchar; 5],
        pub MouseDownOwned: [c_uchar; 5],
        pub MouseDownOwnedUnlessPopupClose: [c_uchar; 5],
        pub MouseWheelRequestAxisSwap: c_uchar,
        pub MouseCtrlLeftAsRightClick: c_uchar,
        pub MouseDownDuration: [c_float; 5],
        pub MouseDownDurationPrev: [c_float; 5],
        pub MouseDragMaxDistanceAbs: [ImVec2; 5],
        pub MouseDragMaxDistanceSqr: [c_float; 5],
        pub PenPressure: c_float,
        pub AppFocusLost: c_uchar,
        pub AppAcceptingEvents: c_uchar,
        pub BackendUsingLegacyKeyArrays: ImS8,
        pub BackendUsingLegacyNavInputArray: c_uchar,
        pub InputQueueSurrogate: ImWchar16,
        pub InputQueueCharacters: ImVector_ImWchar,
    }

    #[repr(C)]
    pub struct ImGuiKeyData {
        Down: c_uchar,
        DownDuration: c_float,
        DownDurationPrev: c_float,
        AnalogValue: c_float,
    }

    #[repr(C)]
    pub struct ImVec2 {
        x: f32,
        y: f32,
    }

    #[repr(C)]
    pub struct ImVector_ImWchar {
        Size: c_int,
        Capacity: c_int,
        Data: *mut ImWchar,
    }

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
        pub fn igGetIO() -> *mut ImGuiIO;
        pub fn igNewFrame();
        pub fn igRender();
        pub fn igSameLine(offset_from_start_x: c_float, spacing: c_float);
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

/// Enable keyboard controls.
pub const CONFIG_FLAGS_NAV_ENABLE_KEYBOARD: i32 = 1 << 0;

/// Enable docking mode.
pub const CONFIG_FLAGS_DOCKING_ENABLE: i32 = 1 << 7;

/// Always autoresize window.
pub const WINDOW_FLAGS_ALWAYS_AUTORESIZE: i32 = 1 << 6;

/// A specialized result type.
pub type Result<T> = result::Result<T, Error>;

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
                write!(f, "failed to initialize ImGui GLFW backend")
            }
            Error::ImGuiImplOpenGL3Init => write!(f, "failed to initialize ImGui OpenGL backend"),
            Error::InvalidCString(err) => write!(f, "invalid C string: {err}"),
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
pub fn begin(name: &str, open: Option<&mut bool>, flags: Option<i32>) -> Result<bool> {
    let name = CString::new(name)?;
    let flags = flags.unwrap_or(0);

    let unfolded = match open {
        Some(open) => {
            let mut ig_open: c_uchar = if *open { 1 } else { 0 };
            let unfolded = unsafe {
                ffi::igBegin(
                    name.as_ptr(),
                    &mut ig_open as *mut c_uchar,
                    flags as ffi::ImGuiWindowFlags,
                )
            };
            *open = ig_open != 0;
            unfolded
        }
        None => unsafe {
            ffi::igBegin(
                name.as_ptr(),
                ptr::null_mut(),
                flags as ffi::ImGuiWindowFlags,
            )
        },
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

/// IO state.
pub struct IO(*mut ffi::ImGuiIO);

impl IO {
    /// Sets the configuration flags.
    pub fn set_config_flags(&mut self, flags: i32) {
        unsafe { (*self.0).ConfigFlags = flags as ffi::ImGuiConfigFlags };
    }

    /// Returns the configuration flags.
    pub fn config_flags(&self) -> i32 {
        unsafe { (*self.0).ConfigFlags }
    }

    /// Sets the path of the .ini file. If [`Option:None`] is
    /// provided, it disables automatic load/save. Note that this
    /// function creates a `CString` from `filename` internally that
    /// is leaked.
    pub fn set_ini_filename(&mut self, filename: Option<&str>) -> Result<()> {
        let filename = match filename {
            Some(s) => Box::leak(Box::new(CString::new(s)?)).as_ptr(),
            None => ptr::null(),
        };
        unsafe { (*self.0).IniFilename = filename };
        Ok(())
    }

    /// Sets the path of the .log file. If [`Option:None`] is
    /// provided, it disables logging. Note that this function creates
    /// a `CString` from `filename` internally that is leaked.
    pub fn set_log_filename(&mut self, filename: Option<&str>) -> Result<()> {
        let filename = match filename {
            Some(s) => Box::leak(Box::new(CString::new(s)?)).as_ptr(),
            None => ptr::null(),
        };
        unsafe { (*self.0).LogFilename = filename };
        Ok(())
    }
}

/// Returns the IO state.
pub fn get_io() -> IO {
    let io = unsafe { ffi::igGetIO() };
    IO(io)
}

/// Starts a new frame.
pub fn new_frame() {
    unsafe { ffi::igNewFrame() }
}

/// Renders a frame.
pub fn render() {
    unsafe { ffi::igRender() }
}

/// Arranges widgets and groups horizontally. `offset_from_start_x`
/// must be provided in window coordinates.
pub fn same_line(offset_from_start_x: Option<f32>, spacing: Option<f32>) {
    let offset_from_start_x = offset_from_start_x.unwrap_or(0.0);
    let spacing = spacing.unwrap_or(-1.0);
    unsafe { ffi::igSameLine(offset_from_start_x, spacing) }
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
    format: Option<&str>,
    flags: Option<i32>,
) -> Result<bool> {
    let label = CString::new(label)?;
    let format = format.map_or(CString::new("%.3f"), CString::new)?;
    let flags = flags.unwrap_or(0);

    let changed = unsafe {
        ffi::igSliderFloat(
            label.as_ptr(),
            v as *mut c_float,
            min,
            max,
            format.as_ptr(),
            flags as ffi::ImGuiSliderFlags,
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
