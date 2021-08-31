use arr_macro::arr;
use maptile::{BoulderVariant, DirtVariant, TileBase};
use rand::Rng;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ChunkPos {
    x: i32,
    y: i32,
}

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> Self {
        ChunkPos { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

pub struct Chunk {
    pub tiles: [TileBase; (Chunk::SIZE * Chunk::SIZE) as usize], // 32*32
}

impl Chunk {
    pub const SIZE: i32 = 32;

    pub fn generate(_pos: ChunkPos) -> Self {
        Chunk {
            tiles: arr![if rand::thread_rng().gen_bool(0.05) {
                TileBase::Boulder(rand::random::<BoulderVariant>())
            } else {
                TileBase::Dirt(rand::random::<DirtVariant>())
            }; 1024],
        }
    }
}
