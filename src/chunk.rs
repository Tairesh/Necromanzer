use arr_macro::arr;
use assets::Assets;
use human::character::random_character;
use maptile::{GraveData, Terrain, Tile};
use rand::distributions::Standard;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

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
    pub tiles: [Tile; (Chunk::SIZE * Chunk::SIZE) as usize], // 32*32
}

impl Chunk {
    pub const SIZE: i32 = 32;

    pub fn generate(seed: u64, assets: Rc<RefCell<Assets>>, _pos: ChunkPos) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut tiles = arr![Tile::new(if rng.gen_bool(0.05) {
            Terrain::Boulder(rng.sample(Standard))
        } else {
            Terrain::Dirt(rng.sample(Standard))
        }); 1024];
        let count = rng.gen_range(5..20);
        let mut blocked_tiles = HashSet::with_capacity(100);
        for _ in 0..count {
            let mut pos = rng.gen_range(0..1024) as usize;
            while blocked_tiles.contains(&pos) {
                pos = rng.gen_range(0..1024) as usize;
            }
            blocked_tiles.insert(pos);
            if pos > 0 {
                blocked_tiles.insert(pos - 1);
            }
            if pos < 1023 {
                blocked_tiles.insert(pos + 1);
            }
            if pos > 31 {
                blocked_tiles.insert(pos - 32);
            }
            if pos < 1023 - 32 {
                blocked_tiles.insert(pos + 32);
            }
            tiles[pos].terrain = Terrain::Grave(
                rng.sample(Standard),
                GraveData {
                    character: random_character(&mut rng, assets.clone()),
                    death_year: rng.gen_range(0..255),
                },
            );
        }
        Chunk { tiles }
    }
}
