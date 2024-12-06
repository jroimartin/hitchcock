//! Hitchcock is a libre demo creation tool.

use hitchcock::{gl, glfw};

const INITIAL_WIDTH: i32 = 800;
const INITIAL_HEIGHT: i32 = 600;

fn main() {
    glfw::init().unwrap();

    glfw::set_error_callback(Some(glfw_error_callback));

    glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
    glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 3);
    glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);

    let window =
        glfw::create_window(INITIAL_WIDTH, INITIAL_HEIGHT, "Hitchcock", None, None).unwrap();

    glfw::make_context_current(window);

    gl::enable(gl::DEBUG_OUTPUT);
    gl::debug_message_callback(gl_error_callback);

    glfw::set_framebuffer_size_callback(window, Some(glfw_framebuffer_size_callback));

    while !glfw::window_should_close(window) {
        gl::clear_color(1.0, 0.0, 0.0, 1.0);
        gl::clear(gl::COLOR_BUFFER_BIT);

        glfw::swap_buffers(window);
        glfw::poll_events();
    }

    glfw::terminate()
}

fn glfw_error_callback(error_code: glfw::ErrorCode, description: &str) {
    eprintln!("GLFW error: {description} ({error_code})");
}

fn glfw_framebuffer_size_callback(_window: glfw::Window, width: i32, height: i32) {
    eprintln!("resize window: size={}x{}", width, height);
}

fn gl_error_callback(source: u32, typ: u32, id: u32, severity: u32, length: i32, message: &str) {
    eprintln!("GL error: source={source}, type={typ}, id={id}, severity={severity}, length={length}, message={message}");
}
