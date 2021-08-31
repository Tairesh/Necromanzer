use avatar::Avatar;
use chunk::{Chunk, ChunkPos};
use maptile::{TileBase, TilePos};
use rand::rngs::StdRng;
use rand::SeedableRng;
use savefile::save;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct World {
    path: PathBuf,
    pub meta: WorldMeta,
    pub avatar: Avatar,
    chunks: HashMap<ChunkPos, Chunk>,
}

#[derive(Debug, Clone)]
pub struct WorldMeta {
    pub name: String,
    pub seed: u64,
}

impl World {
    pub fn new(path: PathBuf, meta: WorldMeta, avatar: Avatar) -> Self {
        Self {
            path,
            meta,
            avatar,
            chunks: HashMap::new(),
        }
    }

    pub fn save(&mut self) {
        save(&self.path, self)
            .map_err(|e| panic!("Error on saving world to {:?}: {}", self.path, e))
            .ok();
    }

    pub fn load_chunk(&mut self, pos: ChunkPos) -> &Chunk {
        let seed = self.meta.seed;
        self.chunks.entry(pos).or_insert_with_key(|pos| {
            let mut rng = StdRng::seed_from_u64(seed);
            Chunk::generate(&mut rng, *pos)
        })
    }

    pub fn load_tile(&mut self, pos: TilePos) -> &TileBase {
        let (chunk, pos) = pos.chunk_and_pos();
        let chunk = self.load_chunk(chunk);
        &chunk.tiles[pos]
    }
}
