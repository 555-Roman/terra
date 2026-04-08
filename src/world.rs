use std::collections::HashMap;
use std::rc::Rc;
use glow::{HasContext, NativeProgram};
use log::log;
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
    int x = int(floor(uv.x * 16));
    int y = int(floor(uv.y * 16));
    int index = y * 16 + x;
    vec2 value0 = layer0[index];
    vec2 value1 = layer1[index];
    if (value0 == vec2(0.1, 0.2))
        color = vec4(0.0, 1.0, 0.0, 1.0);
    else if (value0 == vec2(0.3, 0.4))
        color = vec4(0.0, 1.0, 1.0, 1.0);
    else
        color = vec4(1.0, 0.0, 0.0, 1.0);
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

        let mut test_layer0: [f32; 512] = [0.0; 512];
        for x in 0..16 {
            for y in 0..16 {
                if x*x + y*y <= 8*8 {
                    test_layer0[y*32 + x*2 + 0] = 0.1;
                    test_layer0[y*32 + x*2 + 1] = 0.2;
                } else {
                    test_layer0[y*32 + x*2 + 0] = 0.3;
                    test_layer0[y*32 + x*2 + 1] = 0.4;
                }
            }
        }

        let mut test_layer1: [f32; 512] = [0.0; 512];

        for (position, chunk) in self.loaded_chunks.iter() {
            renderer.set_uniform(self.chunk_program, "layer0", &test_layer0);
            renderer.set_uniform(self.chunk_program, "layer1", &test_layer1);
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