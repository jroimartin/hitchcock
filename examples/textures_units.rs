//! [Textures] lesson of LearnOpenGL. This example introduces texture
//! units.
//!
//! [Textures]: https://learnopengl.com/Getting-started/Textures

use std::{mem, process};

use hitchcock::{gl, glfw, stb_image, Result};

const WALL_JPG: &[u8] = include_bytes!("wall.jpg");
const AWESOMEFACE_PNG: &[u8] = include_bytes!("awesomeface.png");

const INITIAL_WIDTH: i32 = 800;
const INITIAL_HEIGHT: i32 = 600;

const VERTICES: [f32; 20] = [
    // Bottom left corner.
    -0.5, -0.5, 0.0, // Coordinates.
    0.0, 0.0, // Texture coordinates.
    // Bottom right corner.
    0.5, -0.5, 0.0, // Coordinates.
    1.0, 0.0, // Texture coordinates.
    // Top left corner.
    -0.5, 0.5, 0.0, // Coordinates.
    0.0, 1.0, // Texture coordinates.
    // Top right corner.
    0.5, 0.5, 0.0, // Coordinates.
    1.0, 1.0, // Texture coordinates.
];

struct VertexLayout {
    size: usize,
    typ: u32,
    normalized: bool,
    stride: usize,
    pointer: usize,
}

const LAYOUTS: [VertexLayout; 2] = [
    VertexLayout {
        size: 3,
        typ: gl::FLOAT,
        normalized: false,
        stride: 5 * mem::size_of::<f32>(),
        pointer: 0,
    },
    VertexLayout {
        size: 2,
        typ: gl::FLOAT,
        normalized: false,
        stride: 5 * mem::size_of::<f32>(),
        pointer: 3 * mem::size_of::<f32>(),
    },
];

const INDICES: [u32; 6] = [0, 1, 3, 3, 2, 0];

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec2 aTexCoord;

    out vec2 texCoord;

    void main()
    {
        gl_Position = vec4(aPos, 1.0);
        texCoord = aTexCoord;
    }
    "#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    in vec2 texCoord;

    out vec4 FragColor;

    uniform sampler2D uTexture1;
    uniform sampler2D uTexture2;

    void main()
    {
        FragColor = mix(texture(uTexture1, texCoord), texture(uTexture2, texCoord), 0.2);
    }
    "#;

fn main() {
    run().unwrap_or_else(|err| {
        println!("Error: {err}");
        process::exit(1);
    });
}

fn run() -> Result<()> {
    glfw::init()?;

    glfw::set_error_callback(Some(glfw_error_callback));

    let window = build_window()?;
    glfw::make_context_current(window);
    glfw::set_framebuffer_size_callback(window, Some(glfw_framebuffer_size_callback));

    gl::enable(gl::DEBUG_OUTPUT);
    gl::debug_message_callback(gl_debug_callback);

    let shader_program = build_shader_program(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;

    let (vao, vbo, ebo) = build_buffers(&VERTICES, &LAYOUTS, &INDICES);

    let image = stb_image::Image::load_from_memory(WALL_JPG)?;
    let to_wall = build_texture(shader_program, "uTexture1", 0, &image, gl::RGB)?;
    stb_image::set_flip_vertically_on_load(true);
    let image = stb_image::Image::load_from_memory(AWESOMEFACE_PNG)?;
    let to_face = build_texture(shader_program, "uTexture2", 1, &image, gl::RGBA)?;

    while !glfw::window_should_close(window) {
        glfw::poll_events();

        gl::clear_color(0.2, 0.3, 0.3, 1.0);
        gl::clear(gl::COLOR_BUFFER_BIT);

        gl::use_program(shader_program);

        gl::active_texture(gl::TEXTURE0);
        gl::bind_texture(gl::TEXTURE_2D, to_wall);
        gl::active_texture(gl::TEXTURE0 + 1);
        gl::bind_texture(gl::TEXTURE_2D, to_face);

        gl::bind_vertex_array(vao);
        gl::draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0);

        glfw::swap_buffers(window);
    }

    gl::delete_vertex_arrays(&[vao]);
    gl::delete_buffers(&[vbo, ebo]);
    gl::delete_textures(&[to_wall, to_face]);
    gl::delete_program(shader_program);

    glfw::terminate();

    Ok(())
}

fn build_window() -> Result<glfw::Window> {
    glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
    glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 3);
    glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);
    let window = glfw::create_window(
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        "LearnOpenGL: Textures with texture units",
        None,
        None,
    )?;
    Ok(window)
}

fn build_shader_program(vertex_shader_src: &str, fragment_shader_src: &str) -> Result<gl::Program> {
    let vertex_shader = gl::create_shader(gl::VERTEX_SHADER);
    gl::shader_source(vertex_shader, &[vertex_shader_src])?;
    gl::compile_shader(vertex_shader);

    let fragment_shader = gl::create_shader(gl::FRAGMENT_SHADER);
    gl::shader_source(fragment_shader, &[fragment_shader_src])?;
    gl::compile_shader(fragment_shader);

    let shader_program = gl::create_program();
    gl::attach_shader(shader_program, vertex_shader);
    gl::attach_shader(shader_program, fragment_shader);
    gl::link_program(shader_program);
    gl::delete_shader(vertex_shader);
    gl::delete_shader(fragment_shader);

    Ok(shader_program)
}

fn build_buffers(
    vertices: &[f32],
    layouts: &[VertexLayout],
    indices: &[u32],
) -> (gl::VertexArray, gl::Buffer, gl::Buffer) {
    let vaos = gl::gen_vertex_arrays(1);
    let vbos = gl::gen_buffers(1);
    let ebos = gl::gen_buffers(1);

    gl::bind_vertex_array(vaos[0]);
    gl::bind_buffer(gl::ARRAY_BUFFER, vbos[0]);
    gl::buffer_data(gl::ARRAY_BUFFER, vertices, gl::STATIC_DRAW);
    gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, ebos[0]);
    gl::buffer_data(gl::ELEMENT_ARRAY_BUFFER, indices, gl::STATIC_DRAW);

    for (i, layout) in layouts.iter().enumerate() {
        gl::vertex_attrib_pointer(
            i as u32,
            layout.size,
            layout.typ,
            layout.normalized,
            layout.stride,
            layout.pointer,
        );
        gl::enable_vertex_attrib_array(i as u32);
    }

    gl::bind_buffer(gl::ARRAY_BUFFER, gl::Buffer::zero());
    gl::bind_vertex_array(gl::VertexArray::zero());

    (vaos[0], vbos[0], ebos[0])
}

fn build_texture(
    shader_program: gl::Program,
    texture_uniform: &str,
    texture_unit: i32,
    image: &stb_image::Image,
    image_format: u32,
) -> Result<gl::Texture> {
    let tos = gl::gen_textures(1);

    gl::bind_texture(gl::TEXTURE_2D, tos[0]);
    gl::tex_parameter(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.into());
    gl::tex_parameter(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.into());
    gl::tex_parameter(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR.into(),
    );
    gl::tex_parameter(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.into());
    gl::tex_image_2d(gl::TEXTURE_2D, 0, gl::RGB, image, image_format);
    gl::generate_mipmap(gl::TEXTURE_2D);
    gl::use_program(shader_program);
    gl::uniform(
        gl::get_uniform_location(shader_program, texture_uniform)?,
        texture_unit.into(),
    );

    Ok(tos[0])
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
