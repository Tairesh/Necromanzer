#![allow(dead_code)]
use arrayvec::ArrayVec;
use assets::game_data::GameData;
use human::character::Character;
use map::item::{Item, ItemType};
use map::pos::ChunkPos;
use map::terrains_impl::{Boulder, Dirt, Grass, Grave, GraveData, GraveVariant};
use map::tile::Tile;
use rand::distributions::Standard;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct ChunkUnique {
    pos: ChunkPos,
    world_seed: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Chunk {
    pub pos: ChunkPos,
    pub tiles: ArrayVec<Tile, { Chunk::USIZE }>,
}

impl Chunk {
    pub const SIZE: i32 = 32;
    pub const USIZE: usize = (Chunk::SIZE * Chunk::SIZE) as usize;

    pub fn generate(world_seed: String, game_data: &GameData, pos: ChunkPos) -> Self {
        let mut hasher = DefaultHasher::new();
        let seed = ChunkUnique { pos, world_seed };
        seed.hash(&mut hasher);
        let seed = hasher.finish();
        let mut rng = StdRng::seed_from_u64(seed);
        let mut tiles = ArrayVec::new();
        for _ in 0..Chunk::USIZE {
            tiles.push(Tile::new(if rng.gen_bool(0.01) {
                Boulder::new(rng.sample(Standard)).into()
            } else if rng.gen_bool(0.5) {
                Grass::new(rng.sample(Standard)).into()
            } else {
                Dirt::new(rng.sample(Standard)).into()
            }));
        }
        let count: usize = rng.gen_range(5..20);
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
            if rng.gen_bool(0.1) {
                tiles
                    .get_mut(pos)
                    .unwrap()
                    .items
                    .push(Item::new(ItemType::Shovel));
            } else {
                let death_year = rng.gen_range(200..=255);
                tiles.get_mut(pos).unwrap().terrain = Grave::new(
                    if death_year < 200 {
                        GraveVariant::Old
                    } else {
                        GraveVariant::New
                    },
                    GraveData {
                        character: Character::random(&mut rng, game_data),
                        death_year,
                    },
                )
                .into();
            }
        }
        Chunk { pos, tiles }
    }
}
