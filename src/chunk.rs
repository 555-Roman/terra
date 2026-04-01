use crate::tiles::TileWithData;

pub const CHUNK_SIZE: usize = 16;

pub struct ChunkPos(pub i32, pub i32);

pub struct ChunkData {
    pub layer0: [TileWithData; CHUNK_SIZE * CHUNK_SIZE],
    pub layer1: [TileWithData; CHUNK_SIZE * CHUNK_SIZE]
}