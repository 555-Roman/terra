mod app;
mod renderer;
mod quad;

use glow::HasContext;
use log::error;
use crate::app::App;
use crate::quad::Quad;
use crate::renderer::Renderer;

pub struct Terra {
    quad: Option<Quad>,
}

impl Terra {
    pub fn new() -> Self {
        Self {
            quad: None,
        }
    }

    pub unsafe fn init(&mut self) {
        let app = App::new("ee", Self::custom_render, self);

        App::run_app(app).unwrap_or_else(|err| {
            error!("Failed to run app!: {:?}", err);
        });
    }

    pub unsafe fn custom_render(&mut self, renderer: &Renderer) {
        let gl = &renderer.gl;
        let _quad = match &self.quad {
            None => unsafe {
                let quad = renderer.new_quad(0.0, 0.0, 10.0, 10.0);
                self.quad = Some(quad);
                self.quad.as_ref().unwrap()
            }
            Some(quad) => quad
        };

        unsafe {
            gl.clear_color(0.0, 1.0, 1.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            renderer.swap_buffers()
        }
    }
}


fn main() {
    pretty_env_logger::init();
    unsafe { Terra::new().init(); }
}