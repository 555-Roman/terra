use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PerTileData {
    pub animation_frame: i8,
}

pub struct TileTexture(u32, u32);

pub struct TileProperties {
    frames: Vec<TileTexture>,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TileType {
    AIR,
    WATER,
    GROUND
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct TileWithData(TileType, PerTileData);

impl TileWithData {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            0: tile_type,
            1: PerTileData { animation_frame: 0 },
        }
    }
}

pub const TILE_PROPERTIES: LazyLock<HashMap<TileType, TileProperties>> = LazyLock::new(|| {
   let mut map = HashMap::new();
    map.insert(TileType::WATER, TileProperties {
        frames: vec![TileTexture(0, 0)]
    });
    map.insert(TileType::GROUND, TileProperties {
        frames: vec![TileTexture(0, 0)]
    });
    map
});