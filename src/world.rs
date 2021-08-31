use chunk::{Chunk, ChunkPos};
use human::character::Character;
use maptile::{TileBase, TilePos};
use savefile::save;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct World {
    path: PathBuf,
    pub meta: WorldMeta,
    pub avatar: Character,
    chunks: HashMap<ChunkPos, Chunk>,
}

#[derive(Debug, Clone)]
pub struct WorldMeta {
    pub name: String,
    pub seed: String,
}

impl World {
    pub fn new(path: PathBuf, meta: WorldMeta, avatar: Character) -> Self {
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
        self.chunks
            .entry(pos)
            .or_insert_with_key(|pos| Chunk::generate(*pos))
    }

    pub fn load_tile(&mut self, pos: TilePos) -> &TileBase {
        let (chunk, pos) = pos.chunk_and_pos();
        let chunk = self.load_chunk(chunk);
        &chunk.tiles[pos]
    }
}
