use arr_macro::arr;
use maptile::{BoulderVariant, DirtVariant, TileBase};
use rand::Rng;

pub struct Chunk {
    // x: i32,
    // y: i32,
    pub tiles: [TileBase; 1024], // 32*32
}

impl Chunk {
    pub fn generate(_x: i32, _y: i32) -> Self {
        Chunk {
            // x, y,
            tiles: arr![if rand::thread_rng().gen_bool(0.05) {
                TileBase::Boulder(rand::random::<BoulderVariant>())
            } else {
                TileBase::Dirt(rand::random::<DirtVariant>())
            }; 1024],
        }
    }
}
