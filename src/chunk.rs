use arr_macro::arr;
use assets::Assets;
use human::character::random_character;
use maptile::{GraveData, Terrain, Tile};
use rand::distributions::Standard;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
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

#[derive(Hash)]
struct ChunkUnique {
    pos: ChunkPos,
    world_seed: u64,
}

pub struct Chunk {
    pub tiles: [Tile; (Chunk::SIZE * Chunk::SIZE) as usize], // 32*32
}

impl Chunk {
    pub const SIZE: i32 = 16;
    pub const USIZE: usize = (Chunk::SIZE * Chunk::SIZE) as usize;

    pub fn generate(world_seed: u64, assets: Rc<RefCell<Assets>>, pos: ChunkPos) -> Self {
        let mut hasher = DefaultHasher::new();
        let seed = ChunkUnique { pos, world_seed };
        seed.hash(&mut hasher);
        let seed = hasher.finish();
        let mut rng = StdRng::seed_from_u64(seed);
        let mut tiles = arr![Tile::new(if rng.gen_bool(0.01) {
            Terrain::Boulder(rng.sample(Standard))
        } else if rng.gen_bool(0.5) {
            Terrain::DeadGrass(rng.sample(Standard))
        } else if rng.gen_bool(0.1) {
            Terrain::Grass(rng.sample(Standard))
        } else {
            Terrain::Dirt(rng.sample(Standard))
        }); 256];
        let count = rng.gen_range(5..20);
        let mut blocked_tiles = HashSet::with_capacity(100);
        for _ in 0..count {
            let mut pos = rng.gen_range(0..Chunk::USIZE) as usize;
            while blocked_tiles.contains(&pos) {
                pos = rng.gen_range(0..Chunk::USIZE);
            }
            blocked_tiles.insert(pos);
            if pos > 0 {
                blocked_tiles.insert(pos - 1);
            }
            if pos < Chunk::USIZE - 1 {
                blocked_tiles.insert(pos + 1);
            }
            if pos > Chunk::SIZE as usize - 1 {
                blocked_tiles.insert(pos - Chunk::SIZE as usize);
            }
            if pos < Chunk::USIZE - 1 - Chunk::SIZE as usize {
                blocked_tiles.insert(pos + Chunk::SIZE as usize);
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
