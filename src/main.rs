mod app;
mod renderer;
mod quad;
mod chunk;
mod world;
mod tiles;

use std::rc::Rc;
use glow::HasContext;
use log::error;
use crate::app::{App, Rendering};
use crate::chunk::ChunkPos;
use crate::renderer::Renderer;
use crate::world::World;

pub struct Terra {
    world: Option<World>,
}

impl Terra {
    pub fn new() -> Self {
        Self {
            world: None,
        }
    }

    pub unsafe fn init(&mut self) {
        let app = App::new("ee", 800, 800, self);

        App::run_app(app).unwrap_or_else(|err| {
            error!("Failed to run app!: {:?}", err);
        });
    }
}

impl Rendering for Terra {
    fn init(&mut self, renderer: Rc<Renderer>) {
        unsafe {
            self.world = Some(World::new(Rc::clone(&renderer)));

            let world = self.world.as_mut().unwrap();

            world.new_chunk(&ChunkPos(0, 0));
        }
    }

    fn render(&self, renderer: Rc<Renderer>) {
        unsafe {
            renderer.clear_screen([0.0, 1.0, 1.0, 1.0]);

            self.world.as_ref().unwrap().render(); // bruh the problem is that render takes self instead of &self

            renderer.swap_buffers();
        }
    }

    fn drop(&mut self, renderer: Rc<Renderer>) {
        renderer;
    }
}


fn main() {
    pretty_env_logger::init();
    unsafe { Terra::new().init(); }
}