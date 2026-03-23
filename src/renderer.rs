use std::num::NonZeroU32;
use std::process::abort;
use bytemuck::cast_slice;
use glow::{Context, HasContext, COLOR_BUFFER_BIT};
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

pub struct Renderer {
    pub surface: Surface<WindowSurface>,
    pub context: PossiblyCurrentContext,
    pub gl: Context,
}

impl Renderer {
    pub const VERTICES: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0
    ];

    pub unsafe fn new(gl_config: &Config, window: &Window) -> Self {
        let (surface, context, gl) = Self::init_gl(gl_config, window);

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
            cast_slice(&Self::VERTICES),
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



        Self {
            surface,
            context,
            gl,
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

    pub unsafe fn render(&self) {
        self.gl.clear_color(0.0, 1.0, 1.0, 1.0);
        self.gl.clear(COLOR_BUFFER_BIT);

        self.surface.swap_buffers(&self.context).unwrap();
    }
}