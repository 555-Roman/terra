pub struct Quad {
    pub size: [f32; 2],
    pub offset: [f32; 2],
}

impl Quad {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Quad {
            size: [width, height],
            offset: [x, y],
        }
    }
}