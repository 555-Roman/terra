use std::collections::HashMap;
use std::rc::Rc;
use glow::{HasContext, NativeProgram};
use crate::chunk::{ChunkData, ChunkPos};
use crate::renderer::Renderer;

pub struct World {
    pub loaded_chunks: HashMap<ChunkPos, ChunkData>,

    renderer: Rc<Renderer>,
    chunk_program: NativeProgram,
}

impl World {
    const CHUNK_FRAGMENT: &str =
"#version 460 core
uniform sampler2D atlas;
uniform vec2 layer0[256];
uniform vec2 layer1[256];
in vec2 uv;
out vec4 color;
void main() {
    color = vec4(fract(uv * 16), 0.0, 1.0);
}";

    pub unsafe fn new(renderer: Rc<Renderer>) -> Self {
        let chunk_program = renderer.new_program(Self::CHUNK_FRAGMENT);

        Self {
            loaded_chunks: HashMap::new(),
            renderer,
            chunk_program
        }
    }

    pub fn new_chunk(&mut self, pos: &ChunkPos) {
        self.loaded_chunks.insert(*pos, ChunkData::new(pos.0, pos.1));
    }

    pub unsafe fn render(&self) {
        let renderer = &self.renderer;
        renderer.use_program(self.chunk_program);
        for (position, chunk) in self.loaded_chunks.iter() {
            renderer.render_program_in_use(&chunk.quad, self.chunk_program);
        }
    }
}

impl Drop for World {
    fn drop(&mut self) {
        let gl = &self.renderer.gl;
        unsafe { gl.delete_program(self.chunk_program); }
    }
}