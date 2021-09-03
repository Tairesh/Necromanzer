use assets::Assets;
use human::character::random_character;
use map::pos::ChunkPos;
use map::terrains::{GraveData, Terrain};
use map::tile::Tile;
use rand::distributions::Standard;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Hash)]
struct ChunkUnique {
    pos: ChunkPos,
    world_seed: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Chunk {
    pub pos: ChunkPos,
    pub tiles: Vec<Tile>,
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
        let mut tiles = Vec::with_capacity(Chunk::USIZE);
        for _ in 0..Chunk::USIZE {
            tiles.push(Tile::new(if rng.gen_bool(0.01) {
                Terrain::Boulder(rng.sample(Standard))
            } else if rng.gen_bool(0.5) {
                Terrain::Grass(rng.sample(Standard))
            } else if rng.gen_bool(0.1) {
                Terrain::DeadGrass(rng.sample(Standard))
            } else {
                Terrain::Dirt(rng.sample(Standard))
            }));
        }
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
            tiles.get_mut(pos).unwrap().terrain = Terrain::Grave(
                rng.sample(Standard),
                GraveData {
                    character: random_character(&mut rng, assets.clone()),
                    death_year: rng.gen_range(0..255),
                },
            );
        }
        Chunk { pos, tiles }
    }
}
