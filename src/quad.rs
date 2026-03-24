use std::process::abort;
use std::rc::Rc;
use bytemuck::cast_slice;
use glow::{Context, HasContext, NativeBuffer, NativeVertexArray};
use log::error;
use crate::renderer::Renderer;

pub struct Quad {
    vbo: NativeBuffer,
    pub vao: NativeVertexArray,
    ebo: NativeBuffer,
    gl: Rc<Context>,
}

impl Drop for Quad {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self.ebo);
        }
    }
}

impl Quad {
    pub unsafe fn new(gl: &Rc<Context>, x: f32, y: f32, width: f32, height: f32) -> Self {unsafe{
        let vbo = match gl.create_buffer() {
            Ok(buffer) => buffer,
            Err(err) => {
                error!("Failed to create buffer: {:?}", err);
                abort();
            }
        };

        let vao = match gl.create_vertex_array() {
            Ok(array) => array,
            Err(err) => {
                error!("Failed to create buffer: {:?}", err);
                abort();
            }
        };

        let ebo = match gl.create_buffer() {
            Ok(buffer) => buffer,
            Err(err) => {
                error!("Failed to create buffer: {:?}", err);
                abort();
            }
        };

        let vertices: [f32; 16] = [
            x+width, y+height, 1.0, 1.0,
            x+width, y       , 1.0, 0.0,
            x      , y       , 0.0, 0.0,
            x      , y+height, 0.0, 1.0,
        ];


        gl.bind_vertex_array(Some(vao));

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&vertices), glow::STATIC_DRAW);

        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, cast_slice(&Renderer::INDICES), glow::STATIC_DRAW);

        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 4 * 4, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 4 * 4, 2 * 4);
        gl.enable_vertex_attrib_array(1);

        gl.bind_vertex_array(None);

        Quad {
            vbo,
            vao,
            ebo,
            gl: Rc::clone(&gl),
        }
    }}
}