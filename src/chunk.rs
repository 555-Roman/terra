use crate::quad::Quad;
use crate::tiles::{TileType, TileWithData};

pub const CHUNK_SIZE: usize = 16;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct ChunkPos(pub i32, pub i32);

pub struct ChunkData {
    pub layer0: [TileWithData; CHUNK_SIZE * CHUNK_SIZE],
    pub layer1: [TileWithData; CHUNK_SIZE * CHUNK_SIZE],
    pub quad: Quad,
}

impl ChunkData {
    pub fn new(x: i32, y: i32) -> Self  {
        // let quad = Quad::new(x as f32, y as f32, CHUNK_SIZE as f32, CHUNK_SIZE as f32);
        let quad = Quad::new(x as f32, y as f32, 1.0, 1.0);

        Self {
            layer0: [TileWithData::new(TileType::WATER); CHUNK_SIZE * CHUNK_SIZE],
            layer1: [TileWithData::new(TileType::AIR); CHUNK_SIZE * CHUNK_SIZE],
            quad,
        }
    }
}