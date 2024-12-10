//! Hitchcock is a libre demo creation tool.

use std::mem;

use hitchcock::{gl, glfw, imgui, Result};

const INITIAL_WIDTH: i32 = 800;
const INITIAL_HEIGHT: i32 = 600;

const VERTICES: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    void main()
    {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;

    void main()
    {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn main() {
    run().unwrap_or_else(|err| {
        println!("Error: {err}");
        std::process::exit(1);
    });
}

fn run() -> Result<()> {
    glfw::init()?;

    glfw::set_error_callback(Some(glfw_error_callback));

    glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
    glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 3);
    glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);

    let window = glfw::create_window(INITIAL_WIDTH, INITIAL_HEIGHT, "Hitchcock", None, None)?;
    glfw::make_context_current(window);
    glfw::set_framebuffer_size_callback(window, Some(glfw_framebuffer_size_callback));

    gl::enable(gl::DEBUG_OUTPUT);
    gl::debug_message_callback(gl_debug_callback);

    let vertex_shader = gl::create_shader(gl::VERTEX_SHADER);
    gl::shader_source(vertex_shader, &[VERTEX_SHADER_SOURCE]);
    gl::compile_shader(vertex_shader);

    let fragment_shader = gl::create_shader(gl::FRAGMENT_SHADER);
    gl::shader_source(fragment_shader, &[FRAGMENT_SHADER_SOURCE]);
    gl::compile_shader(fragment_shader);

    let shader_program = gl::create_program();
    gl::attach_shader(shader_program, vertex_shader);
    gl::attach_shader(shader_program, fragment_shader);
    gl::link_program(shader_program);
    gl::delete_shader(vertex_shader);
    gl::delete_shader(fragment_shader);

    let vaos = gl::gen_vertex_arrays(1);
    gl::bind_vertex_array(vaos[0]);

    let vbos = gl::gen_buffers(1);
    gl::bind_buffer(gl::ARRAY_BUFFER, vbos[0]);
    gl::buffer_data(gl::ARRAY_BUFFER, &VERTICES, gl::STATIC_DRAW);

    gl::vertex_attrib_pointer(0, 3, gl::FLOAT, false, 3 * mem::size_of::<f32>(), 0);
    gl::enable_vertex_attrib_array(0);

    gl::bind_buffer(gl::ARRAY_BUFFER, gl::Buffer::zero());
    gl::bind_vertex_array(gl::VertexArray::zero());

    let ig_ctx = imgui::create_context(None);
    imgui::glfw::init_for_opengl(window, true)?;
    imgui::opengl::init("#version 330 core")?;

    let mut demo_open = true;
    let mut window_open = true;
    let mut checkbox_checked = false;
    let mut slider_value = 0.0;

    while !glfw::window_should_close(window) {
        glfw::poll_events();

        imgui::opengl::new_frame();
        imgui::glfw::new_frame();
        imgui::new_frame();

        if demo_open {
            imgui::show_demo_window(Some(&mut demo_open));
        }

        if window_open {
            if imgui::begin("Dear ImGui window", Some(&mut window_open), 0)? {
                imgui::text("Text widget")?;

                let checkbox_changed = imgui::checkbox("Checkbox widget", &mut checkbox_checked)?;
                if checkbox_changed {
                    println!("Checkbox changed!");
                }
                if checkbox_checked {
                    println!("Checkbox checked!");
                }

                let slider_changed = imgui::slider_float(
                    "Slider float widget",
                    &mut slider_value,
                    0.0,
                    100.0,
                    "value = %.3f",
                    0,
                )?;
                if slider_changed {
                    println!("Slider value changed!");
                }
                if slider_value > 10.0 && slider_value < 11.0 {
                    println!("Slider value between 10.0 and 11.0 ({slider_value})");
                }
            }
            imgui::end();
        }

        gl::clear_color(0.2, 0.3, 0.3, 1.0);
        gl::clear(gl::COLOR_BUFFER_BIT);

        gl::use_program(shader_program);
        gl::bind_vertex_array(vaos[0]);
        gl::draw_arrays(gl::TRIANGLES, 0, 3);

        imgui::render();
        imgui::opengl::render_draw_data(imgui::get_draw_data());

        glfw::swap_buffers(window);
    }

    imgui::opengl::shutdown();
    imgui::glfw::shutdown();
    imgui::destroy_context(Some(ig_ctx));

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
