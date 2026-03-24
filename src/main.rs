mod app;
mod renderer;
mod quad;

use glow::{HasContext, NativeProgram};
use log::{debug, error, info};
use crate::app::{App, Rendering};
use crate::quad::Quad;
use crate::renderer::Renderer;

pub struct Terra {
    quad: Option<Quad>,
    quad_program: Option<NativeProgram>,
}

impl Terra {
    pub fn new() -> Self {
        Self {
            quad: None,
            quad_program: None,
        }
    }

    pub unsafe fn init(&mut self) {
        let app = App::new("ee", self);

        App::run_app(app).unwrap_or_else(|err| {
            error!("Failed to run app!: {:?}", err);
        });
    }
}

impl Rendering for Terra {
    fn init(&mut self, renderer: &Renderer) {
        unsafe {
            self.quad = Some(renderer.new_quad(0.0, 0.0, 100.0, 100.0));
            let fragment_code: &str = "#version 330 core\nin vec2 texcoords;\nout vec4 color;\nvoid main() {\ncolor = vec4(1.0, 1.0, 0.0, 1.0);\n}";
            self.quad_program = Some(renderer.new_program(fragment_code))
        }
    }
    fn render(&mut self, renderer: &Renderer) {
        let gl = &renderer.gl;

        unsafe {
            gl.clear_color(0.0, 1.0, 1.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            gl.bind_vertex_array(Some(self.quad.as_ref().unwrap().vao));
            gl.use_program(self.quad_program);
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);

            renderer.swap_buffers();
        }
    }
    fn drop(&mut self, renderer: &Renderer) {
        let gl = &renderer.gl;

        unsafe {
            gl.delete_program(self.quad_program.unwrap());
        }
    }
}


fn main() {
    pretty_env_logger::init();
    unsafe { Terra::new().init(); }
}