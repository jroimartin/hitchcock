//! [Shaders] lesson of LearnOpenGL. This example introduces
//! additional vertex attributes.
//!
//! [Shaders]: https://learnopengl.com/Getting-started/Shaders

use std::{mem, process};

use hitchcock::{gl, glfw, Result};

const INITIAL_WIDTH: i32 = 800;
const INITIAL_HEIGHT: i32 = 600;

const VERTICES: [f32; 18] = [
    -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // Bottom Left, Red.
    0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // Bottom Right, Green.
    0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // Top, Blue.
];

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 fColor;

    void main()
    {
        gl_Position = vec4(aPos, 1.0);
        fColor = aColor;
    }
    "#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    in vec3 fColor;

    out vec4 FragColor;

    void main()
    {
        FragColor = vec4(fColor, 1.0);
    }
    "#;

fn main() {
    example().unwrap_or_else(|err| {
        println!("Error: {err}");
        process::exit(1);
    });
}

fn example() -> Result<()> {
    glfw::init()?;

    glfw::set_error_callback(Some(glfw_error_callback));

    glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
    glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 3);
    glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);

    let window = glfw::create_window(
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        "LearnOpenGL: Shaders",
        None,
        None,
    )?;
    glfw::make_context_current(window);
    glfw::set_framebuffer_size_callback(window, Some(glfw_framebuffer_size_callback));

    gl::enable(gl::DEBUG_OUTPUT);
    gl::debug_message_callback(gl_debug_callback);

    let vertex_shader = gl::create_shader(gl::VERTEX_SHADER);
    gl::shader_source(vertex_shader, &[VERTEX_SHADER_SOURCE])?;
    gl::compile_shader(vertex_shader);

    let fragment_shader = gl::create_shader(gl::FRAGMENT_SHADER);
    gl::shader_source(fragment_shader, &[FRAGMENT_SHADER_SOURCE])?;
    gl::compile_shader(fragment_shader);

    let shader_program = gl::create_program();
    gl::attach_shader(shader_program, vertex_shader);
    gl::attach_shader(shader_program, fragment_shader);
    gl::link_program(shader_program);
    gl::delete_shader(vertex_shader);
    gl::delete_shader(fragment_shader);

    let vaos = gl::gen_vertex_arrays(1);
    let vbos = gl::gen_buffers(1);

    gl::bind_vertex_array(vaos[0]);
    gl::bind_buffer(gl::ARRAY_BUFFER, vbos[0]);
    gl::buffer_data(gl::ARRAY_BUFFER, &VERTICES, gl::STATIC_DRAW);
    gl::vertex_attrib_pointer(0, 3, gl::FLOAT, false, 6 * mem::size_of::<f32>(), 0);
    gl::enable_vertex_attrib_array(0);
    gl::vertex_attrib_pointer(
        1,
        3,
        gl::FLOAT,
        false,
        6 * mem::size_of::<f32>(),
        3 * mem::size_of::<f32>(),
    );
    gl::enable_vertex_attrib_array(1);
    gl::bind_buffer(gl::ARRAY_BUFFER, gl::Buffer::zero());
    gl::bind_vertex_array(gl::VertexArray::zero());

    while !glfw::window_should_close(window) {
        glfw::poll_events();

        gl::clear_color(0.2, 0.3, 0.3, 1.0);
        gl::clear(gl::COLOR_BUFFER_BIT);

        gl::use_program(shader_program);
        gl::bind_vertex_array(vaos[0]);
        gl::draw_arrays(gl::TRIANGLES, 0, 3);

        glfw::swap_buffers(window);
    }

    gl::delete_vertex_arrays(&vaos);
    gl::delete_buffers(&vbos);
    gl::delete_program(shader_program);

    glfw::terminate();

    Ok(())
}

fn glfw_error_callback(error_code: glfw::ErrorCode, description: &str) {
    eprintln!("GLFW error: {error_code}: {description}");
}

fn glfw_framebuffer_size_callback(_window: glfw::Window, width: i32, height: i32) {
    gl::viewport(0, 0, width, height);
}

fn gl_debug_callback(
    source: gl::DebugSource,
    typ: gl::DebugType,
    id: u32,
    severity: gl::DebugSeverity,
    message: &str,
) {
    eprintln!("GL debug: {typ} ({severity}): {source}: {message} ({id})");
}
