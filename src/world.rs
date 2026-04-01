use std::collections::HashMap;
use std::rc::Rc;
use glow::{Context, HasContext, NativeProgram};
use crate::chunk::{ChunkData, ChunkPos};
use crate::renderer::Renderer;

pub struct World {
    pub loaded_chunks: HashMap<ChunkPos, ChunkData>,

    renderer: Rc<Renderer>,
    chunk_program: NativeProgram,
}

impl World {
    // what type is layer0 and layer1???? [TileWithData; CHUNK_SIZE * CHUNK_SIZE] in the shader idfk i not good with shaderfs
    const CHUNK_FRAGMENT: &str = "
#version 330 core\n
uniform sampler2D atlas;\n
uniform  layer0;\n
uniform  layer1;\n
in vec2 uv;\n
out vec4 color;\n
void main() {\n
    color = vec4(1.0, 0.0, 0.0, 1.0);\n
}";

    pub unsafe fn new(renderer: Rc<Renderer> /* ??? that works ig*/) -> Self {
        let chunk_program = renderer.new_program(Self::CHUNK_FRAGMENT);
        

        Self {
            loaded_chunks: HashMap::new(),
            renderer,
            chunk_program
        }
    }

    pub unsafe fn render(self) {
        let renderer = &self.renderer;
    }
}

impl Drop for World {
    fn drop(&mut self) {
        let gl = &self.renderer.gl;
        unsafe { gl.delete_program(self.chunk_program); }
    }
}