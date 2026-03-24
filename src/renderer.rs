use std::num::NonZeroU32;
use std::process::abort;
use std::rc::Rc;
use bytemuck::cast_slice;
use glow::{Context, HasContext, NativeBuffer, NativeShader, NativeVertexArray};
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

    vao: NativeVertexArray,
    ebo: NativeBuffer,
    vertex_shader: NativeShader,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_shader(self.vertex_shader);
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self.ebo);
        }
    }
}

impl Renderer {
    pub const INDICES: [u32; 6] = [
        0, 1, 3,   // first triangle
        1, 2, 3    // second triangle
    ];
    pub const VERTEX_SHADER: &str = "
#version 330 core\n
layout (location = 0) in vec2 aPos;\n
out vec2 texcoords;\n
void main()\n
{\n
   gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);\n
}";

    pub unsafe fn new(gl_config: &Config, window: &Window) -> Self {
        let (surface, context, gl) = Self::init_gl(gl_config, window);

        let vao: NativeVertexArray;
        let ebo: NativeBuffer;
        let vertex_shader: NativeShader;

        unsafe {
            /* - SHADER COMPILATION AND PROGRAM LINKING - */
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
            ebo = match gl.create_buffer() {
                Ok(buffer) => buffer,
                Err(err) => {
                    error!("Failed to create buffer: {:?}", err);
                    abort();
                }
            };

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, cast_slice(&Self::INDICES), glow::STATIC_DRAW);

            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 4 * 4, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 4 * 4, 2 * 4);
            gl.enable_vertex_attrib_array(1);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            /* - SETUP VERTEX DATA AND ATTRIBUTES - */
        }


        Self {
            surface,
            context,
            gl: Rc::new(gl),

            vao,
            ebo,
            vertex_shader,
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

    pub unsafe fn new_quad(&self, x: f32, y: f32, width: f32, height: f32) -> Quad {unsafe{
        Quad::new(&self.gl, x, y, width, height)
    }}

    pub unsafe fn swap_buffers(&self) {
        self.surface.swap_buffers(&self.context).unwrap();
    }
}