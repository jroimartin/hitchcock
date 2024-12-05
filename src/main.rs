//! Hitchcock is a libre demo creation tool.

use std::ptr;

use hitchcock::{gl, glfw};

const INITIAL_WIDTH: i32 = 800;
const INITIAL_HEIGHT: i32 = 600;

fn main() {
    glfw::init().unwrap();

    glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
    glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 3);
    glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);

    let window = glfw::create_window(
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        "Hitchcock",
        ptr::null_mut(),
        ptr::null_mut(),
    )
    .unwrap();
    glfw::make_context_current(window);

    gl::viewport(0, 0, INITIAL_WIDTH, INITIAL_HEIGHT);

    while !glfw::window_should_close(window) {
        gl::clear_color(0.2, 0.3, 0.3, 1.0);
        gl::clear(gl::COLOR_BUFFER_BIT);

        glfw::swap_buffers(window);
        glfw::poll_events();
    }

    glfw::terminate()
}
