use std::num::NonZeroU32;
use std::process::abort;
use std::rc::Rc;
use bytemuck::cast_slice;
use glow::{Context, HasContext, NativeBuffer, NativeProgram, NativeShader, NativeVertexArray};
use glutin::config::Config;
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::{GlDisplay, GlSurface, NotCurrentGlContext, PossiblyCurrentGlContext};
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::GlWindow;
use log::error;
use winit::dpi::PhysicalSize;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::Window;
use crate::quad::Quad;

pub struct Renderer {
    pub surface: Surface<WindowSurface>,
    pub context: PossiblyCurrentContext,
    pub gl: Rc<Context>,

    vertex_shader: NativeShader,
    vao: NativeVertexArray,
    vbo: NativeBuffer,
    ebo: NativeBuffer,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_shader(self.vertex_shader);
            self.gl.delete_buffer(self.ebo);
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_vertex_array(self.vao);
        }
    }
}

impl Renderer {
    pub const VERTICES: [f32; 8] = [
        1.0, 1.0,
        1.0, 0.0,
        0.0, 0.0,
        0.0, 1.0,
    ];
    pub const INDICES: [u32; 6] = [
        0, 1, 3,   // first triangle
        1, 2, 3    // second triangle
    ];
    pub const VERTEX_SHADER: &str = "
#version 330 core\n
layout (location = 0) in vec2 aUV;\n\
uniform vec2 size;\n
uniform vec2 offset;\n
out vec2 uv;\n
void main() {\n
    gl_Position = vec4(aUV*size + offset, 0.0, 1.0);\n
    uv = aUV;\n
}";

    pub unsafe fn new(gl_config: &Config, window: &Window) -> Self {
        let (surface, context, gl) = Self::init_gl(gl_config, window);

        let vao: NativeVertexArray;
        let vbo: NativeBuffer;
        let ebo: NativeBuffer;
        let vertex_shader: NativeShader;

        unsafe {
            /* - SHADER COMPILATION - */
            vertex_shader = match gl.create_shader(glow::VERTEX_SHADER) {
                Ok(shader) => shader,
                Err(err) => {
                    error!("Failed to create vertex shader: {:?}", err);
                    abort();
                }
            };
            gl.shader_source(vertex_shader, Self::VERTEX_SHADER);
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                error!("Failed to compile vertex shader: {:?}", gl.get_shader_info_log(vertex_shader));
                abort();
            }
            /* - SHADER COMPILATION - */

            /* - SETUP VERTEX DATA AND ATTRIBUTES - */
            vao = match gl.create_vertex_array() {
                Ok(array) => array,
                Err(err) => {
                    error!("Failed to create buffer: {:?}", err);
                    abort();
                }
            };
            gl.bind_vertex_array(Some(vao));

            vbo = match gl.create_buffer() {
                Ok(buffer) => buffer,
                Err(err) => {
                    error!("Failed to create buffer: {:?}", err);
                    abort();
                }
            };
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&Renderer::VERTICES), glow::STATIC_DRAW);

            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 2 * 4, 0);
            gl.enable_vertex_attrib_array(0);

            ebo = match gl.create_buffer() {
                Ok(buffer) => buffer,
                Err(err) => {
                    error!("Failed to create buffer: {:?}", err);
                    abort();
                }
            };
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, cast_slice(&Renderer::INDICES), glow::STATIC_DRAW);
            /* - SETUP VERTEX DATA AND ATTRIBUTES - */
        }


        Self {
            surface,
            context,
            gl: Rc::new(gl),

            vertex_shader,
            vao,
            vbo,
            ebo,
        }
    }

    fn init_gl(gl_config: &Config, window: &Window) -> (Surface<WindowSurface>, PossiblyCurrentContext, Context) {
        let raw_window_handle = window.window_handle().unwrap_or_else(|err| {
            error!("why tf did that not work?? {:?}", err);
            abort()
        }).as_raw();

        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
        let gl_display = gl_config.display();

        unsafe {
            let gl_context = gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
                error!("ur device is too old noob, skill issue");
                abort()
            }).treat_as_possibly_current();

            let attrs = window
                .build_surface_attributes(Default::default())
                .unwrap_or_else(|_| {
                    error!("ur hardware dont work or smth prob too old skill issue");
                    abort()
                });
            let gl_surface = gl_config.display().create_window_surface(&gl_config, &attrs).unwrap();

            gl_context.make_current(&gl_surface).unwrap();

            let gl = Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));

            gl_surface
                .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .unwrap();

            (gl_surface, gl_context, gl)
        }
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        let width = unsafe { NonZeroU32::new_unchecked(size.width) };
        let height = unsafe { NonZeroU32::new_unchecked(size.height) };

        self.surface.resize(&self.context, width, height);
    }

    pub unsafe fn new_program(&self, fragment_code: &str) -> NativeProgram {unsafe{
        let gl = &self.gl;
        let fragment_shader = match gl.create_shader(glow::FRAGMENT_SHADER) {
            Ok(shader) => shader,
            Err(err) => {
                error!("Failed to create fragment shader: {:?}", err);
                abort();
            }
        };
        gl.shader_source(fragment_shader, fragment_code);
        gl.compile_shader(fragment_shader);
        if !gl.get_shader_compile_status(fragment_shader) {
            error!("Failed to compile fragment shader: {:?}", gl.get_shader_info_log(fragment_shader));
            error!("{:?}", fragment_code);
            abort();
        }

        let shader_program = match gl.create_program() {
            Ok(program) => program,
            Err(err) => {
                error!("Failed to create shader program: {:?}", err);
                abort();
            }
        };
        gl.attach_shader(shader_program, self.vertex_shader);
        gl.attach_shader(shader_program, fragment_shader);
        gl.link_program(shader_program);
        if !gl.get_program_link_status(shader_program) {
            error!("Failed to link shader: {:?}", gl.get_program_info_log(shader_program));
            abort();
        }
        gl.delete_shader(fragment_shader);

        shader_program
    }}

    pub unsafe fn swap_buffers(&self) {
        self.surface.swap_buffers(&self.context).unwrap();
    }

    pub unsafe fn clear_screen(&self, color: [f32; 4]) {unsafe{
        let gl = &self.gl;

        gl.clear_color(color[0], color[1], color[2], color[3]);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }}

    pub unsafe fn use_program(&self, program: NativeProgram) {unsafe{
        let gl = &self.gl;
        gl.use_program(Some(program));
    }}
    pub unsafe fn render_program_in_use(&self, quad: &Quad, program: NativeProgram) {unsafe{
        let gl = &self.gl;

        let size_location = gl.get_uniform_location(program, "size").unwrap();
        gl.uniform_2_f32(Some(&size_location), quad.size[0], quad.size[1]);
        let offset_location = gl.get_uniform_location(program, "offset").unwrap();
        gl.uniform_2_f32(Some(&offset_location), quad.offset[0], quad.offset[1]);

        gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
    }}
    pub unsafe fn render_program_new(&self, quad: &Quad, program: NativeProgram) {unsafe{
        self.use_program(program);
        self.render_program_in_use(quad, program);
    }}

    pub unsafe fn set_uniform(&self, program: NativeProgram, name: &str, values: &[f32]) {
        let gl = &self.gl;

        let uniform_location = gl.get_uniform_location(program, name);
        gl.uniform_2_f32_slice(uniform_location.as_ref(), values);
    }
}