mod app;

use std::process::abort;
use bytemuck::cast_slice;
use glow::{Context, HasContext, COLOR_BUFFER_BIT};
use log::error;
use crate::app::App;

unsafe fn my_init(gl: &Context) { unsafe {
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0
    ];

    let vbo = match gl.create_buffer() {
        Ok(buffer) => buffer,
        Err(err) => {
            error!("Failed to create buffer: {:?}", err);
            abort();
        }
    };
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(
        glow::ARRAY_BUFFER,
        // std::slice::from_raw_parts(vertices.as_ptr() as *const u8, vertices.len() * 4),
        cast_slice(&vertices),
        glow::STATIC_DRAW);

    let vertex = "#version 330 core\n
    layout (location = 0) in vec3 aPos;\n
    void main()\n
    {\n
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);\n
    }\0";

    let vertex_shader = match gl.create_shader(glow::VERTEX_SHADER) {
        Ok(shader) => shader,
        Err(err) => {
            error!("Failed to create vertex shader: {:?}", err);
            abort();
        }
    };
    gl.shader_source(vertex_shader, vertex);
    gl.compile_shader(vertex_shader);

    let fragment = "#version 330 core\n
    out vec4 FragColor;\n
    void main()\n
    {\n
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);\n
    }\0";

    let fragment_shader = match gl.create_shader(glow::FRAGMENT_SHADER) {
        Ok(shader) => shader,
        Err(err) => {
            error!("Failed to create fragment shader: {:?}", err);
            abort();
        }
    };
    gl.shader_source(fragment_shader, fragment);
    gl.compile_shader(fragment_shader);

    let shader_program = match gl.create_program() {
        Ok(program) => program,
        Err(err) => {
            error!("Failed to create shader program: {:?}", err);
            abort();
        }
    };
    gl.attach_shader(shader_program, vertex_shader);
    gl.attach_shader(shader_program, fragment_shader);
    gl.link_program(shader_program);

    gl.use_program(Some(shader_program));
    gl.delete_shader(vertex_shader);
    gl.delete_shader(fragment_shader);


}}

unsafe fn my_render(gl: &Context) { unsafe {
    gl.clear_color(0.0, 1.0, 1.0, 1.0);
    gl.clear(COLOR_BUFFER_BIT);
}}

fn main() {
    pretty_env_logger::init();

    let app = App::new("ee", my_init, my_render);

    App::run_app(app).unwrap_or_else(|err| {
        error!("Failed to run app!: {:?}", err);
    });
}