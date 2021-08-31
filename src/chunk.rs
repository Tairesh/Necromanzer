use arr_macro::arr;
use maptile::{BoulderDistribution, DirtDistribution, TileBase};
use rand::rngs::StdRng;
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

    pub fn generate(rng: &mut StdRng, _pos: ChunkPos) -> Self {
        Chunk {
            tiles: arr![if rng.gen_bool(0.05) {
                TileBase::Boulder(rng.sample(BoulderDistribution{}))
            } else {
                TileBase::Dirt(rng.sample(DirtDistribution{}))
            }; 1024],
        }
    }
}
