use std::process::abort;
use std::rc::Rc;
use bytemuck::cast_slice;
use glow::{Context, HasContext, NativeBuffer};
use log::error;

pub struct Quad {
    vbo: NativeBuffer,
    gl: Rc<Context>,
}

impl Drop for Quad {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.vbo);
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

        let vertices: [f32; 16] = [
            x+width, y+height, 1.0, 1.0,
            x+width, y       , 1.0, 0.0,
            x      , y       , 0.0, 0.0,
            x      , y+height, 0.0, 1.0,
        ];

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&vertices), glow::STATIC_DRAW);

        Quad {
            vbo,
            gl: Rc::clone(&gl),
        }
    }}
}