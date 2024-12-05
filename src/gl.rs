//! OpenGL bindings.

#![allow(non_upper_case_globals)]

pub const COLOR_BUFFER_BIT: u32 = 0x00004000;

macro_rules! glfn {
    ($gl_name:ident, $name:ident $(, $param:ident: $type:ty)*) => {
        static $gl_name: std::sync::OnceLock<fn($($type),*)> = std::sync::OnceLock::new();
        pub fn $name($($param: $type),*) {
            let f = $gl_name.get_or_init(|| unsafe {
                std::mem::transmute::<crate::glfw::GlProc, fn($($type),*)>(
                    crate::glfw::get_proc_address(stringify!($gl_name)).unwrap(),
                )
            });
            f($($param),*)
        }
    };
}

glfn!(glViewport, viewport, x: i32, y: i32, width: i32, height: i32);
glfn!(glClearColor, clear_color, red: f32, green: f32, blue: f32, alpha: f32);
glfn!(glClear, clear, mask: u32);
