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
        "LearnOpenGL: Textures with texture units",
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
    let ebos = gl::gen_buffers(1);

    gl::bind_vertex_array(vaos[0]);
    gl::bind_buffer(gl::ARRAY_BUFFER, vbos[0]);
    gl::buffer_data(gl::ARRAY_BUFFER, &VERTICES, gl::STATIC_DRAW);
    gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, ebos[0]);
    gl::buffer_data(gl::ELEMENT_ARRAY_BUFFER, &INDICES, gl::STATIC_DRAW);
    gl::vertex_attrib_pointer(0, 3, gl::FLOAT, false, 5 * mem::size_of::<f32>(), 0);
    gl::enable_vertex_attrib_array(0);
    gl::vertex_attrib_pointer(
        1,
        2,
        gl::FLOAT,
        false,
        5 * mem::size_of::<f32>(),
        3 * mem::size_of::<f32>(),
    );
    gl::enable_vertex_attrib_array(1);

    gl::bind_buffer(gl::ARRAY_BUFFER, gl::Buffer::zero());
    gl::bind_vertex_array(gl::VertexArray::zero());

    let tos = gl::gen_textures(2);

    let image = stb_image::Image::load_from_memory(WALL_JPG)?;
    gl::bind_texture(gl::TEXTURE_2D, tos[0]);
    gl::tex_parameter(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.into());
    gl::tex_parameter(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.into());
    gl::tex_parameter(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR.into(),
    );
    gl::tex_parameter(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.into());
    gl::tex_image_2d(
        gl::TEXTURE_2D,
        0,
        gl::RGB,
        image.width(),
        image.height(),
        gl::RGB,
        gl::UNSIGNED_BYTE,
        image.pixels(),
    );
    gl::generate_mipmap(gl::TEXTURE_2D);

    stb_image::set_flip_vertically_on_load(true);
    let image = stb_image::Image::load_from_memory(AWESOMEFACE_PNG)?;
    gl::bind_texture(gl::TEXTURE_2D, tos[1]);
    gl::tex_parameter(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.into());
    gl::tex_parameter(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.into());
    gl::tex_parameter(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR.into(),
    );
    gl::tex_parameter(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.into());
    gl::tex_image_2d(
        gl::TEXTURE_2D,
        0,
        gl::RGB,
        image.width(),
        image.height(),
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        image.pixels(),
    );
    gl::generate_mipmap(gl::TEXTURE_2D);

    gl::use_program(shader_program);
    gl::uniform(
        gl::get_uniform_location(shader_program, "uTexture1")?,
        gl::Uniform::from(0),
    );
    gl::uniform(
        gl::get_uniform_location(shader_program, "uTexture2")?,
        gl::Uniform::from(1),
    );

    while !glfw::window_should_close(window) {
        glfw::poll_events();

        gl::clear_color(0.2, 0.3, 0.3, 1.0);
        gl::clear(gl::COLOR_BUFFER_BIT);

        gl::use_program(shader_program);

        gl::active_texture(gl::TEXTURE0);
        gl::bind_texture(gl::TEXTURE_2D, tos[0]);
        gl::active_texture(gl::TEXTURE0 + 1);
        gl::bind_texture(gl::TEXTURE_2D, tos[1]);

        gl::bind_vertex_array(vaos[0]);
        gl::draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0);

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
