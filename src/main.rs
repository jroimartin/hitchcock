//! Hitchcock is a libre demo creation tool.

use hitchcock::{gl, glfw};

const INITIAL_WIDTH: i32 = 800;
const INITIAL_HEIGHT: i32 = 600;

fn main() {
    glfw::init().unwrap();

    glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 4);
    glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 6);
    glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);

    let window =
        glfw::create_window(INITIAL_WIDTH, INITIAL_HEIGHT, "Hitchcock", None, None).unwrap();
    glfw::make_context_current(&window);

    while !glfw::window_should_close(&window) {
        gl::clear_color(1.0, 1.0, 0.0, 1.0);
        gl::clear(gl::COLOR_BUFFER_BIT);

        glfw::swap_buffers(&window);
        glfw::poll_events();
    }

    glfw::terminate()
}
