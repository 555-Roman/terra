mod app;
mod renderer;
mod quad;

use glow::{HasContext, NativeProgram};
use log::error;
use crate::app::{App, Rendering};
use crate::quad::Quad;
use crate::renderer::Renderer;

pub struct Terra {
    quad0: Option<Quad>,
    quad_program0: Option<NativeProgram>,
    quad1: Option<Quad>,
    quad_program1: Option<NativeProgram>,
}

impl Terra {
    pub fn new() -> Self {
        Self {
            quad0: None,
            quad_program0: None,
            quad1: None,
            quad_program1: None,
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
            self.quad0 = Some(renderer.new_quad(0.0, 0.0, 0.5, 0.5));
            // let fragment_code: &str = "#version 330 core\nin vec2 uv;\nout vec4 color;\nvoid main() {\ncolor = vec4(1.0, 1.0, 0.0, 1.0);\n}";
            let fragment_code: &str = "#version 330 core\nin vec2 uv;\nout vec4 color;\nvoid main() {\ncolor = vec4(uv, 0.0, 1.0);\n}";
            self.quad_program0 = Some(renderer.new_program(fragment_code));

            self.quad1 = Some(renderer.new_quad(-0.1, 0.2, 1.0, 0.2));
            let fragment_code: &str = "#version 330 core\nin vec2 uv;\nout vec4 color;\nvoid main() {\ncolor = vec4(1.0, 0.0, 1.0, 1.0);\n}";
            self.quad_program1 = Some(renderer.new_program(fragment_code));
        }
    }
    fn render(&self, renderer: &Renderer) {
        let gl = &renderer.gl;

        unsafe {
            gl.clear_color(0.0, 1.0, 1.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            renderer.use_program(self.quad_program0.unwrap());
            renderer.render_program_in_use(self.quad0.as_ref().unwrap(), self.quad_program0.unwrap());

            renderer.render_program_new(self.quad1.as_ref().unwrap(), self.quad_program1.unwrap());

            renderer.swap_buffers();
        }
    }
    fn drop(&mut self, renderer: &Renderer) {
        let gl = &renderer.gl;

        unsafe {
            gl.delete_program(self.quad_program0.unwrap());
            gl.delete_program(self.quad_program1.unwrap());
        }
    }
}


fn main() {
    pretty_env_logger::init();
    unsafe { Terra::new().init(); }
}