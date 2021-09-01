use avatar::Avatar;
use chunk::{Chunk, ChunkPos};
use maptile::{TileBase, TilePos};
use savefile::save;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct WorldMeta {
    pub name: String,
    pub seed: u64,
    pub current_tick: f64,
}

pub struct World {
    path: PathBuf,
    pub meta: WorldMeta,
    pub avatar: Avatar,
    chunks: HashMap<ChunkPos, Chunk>,
    last_tick: SystemTime,
}

impl World {
    pub fn new(path: PathBuf, meta: WorldMeta, avatar: Avatar) -> Self {
        Self {
            path,
            meta,
            avatar,
            chunks: HashMap::new(),
            last_tick: SystemTime::now(),
        }
    }

    pub fn save(&mut self) {
        save(&self.path, self)
            .map_err(|e| panic!("Error on saving world to {:?}: {}", self.path, e))
            .ok();
    }

    pub fn load_chunk(&mut self, pos: ChunkPos) -> &Chunk {
        let seed = self.meta.seed;
        self.chunks
            .entry(pos)
            .or_insert_with_key(|pos| Chunk::generate(seed, *pos))
    }

    pub fn load_tile(&mut self, pos: TilePos) -> &TileBase {
        let (chunk, pos) = pos.chunk_and_pos();
        let chunk = self.load_chunk(chunk);
        &chunk.tiles[pos]
    }

    pub fn tick(&mut self) {
        // doing actions that should be done
        if let Some(action) = self.avatar.action {
            if action.finish <= self.meta.current_tick {
                action.act(&mut self.avatar);
            }
        }
        // adding time if it's time
        let elapsed = SystemTime::now()
            .duration_since(self.last_tick)
            .unwrap()
            .as_millis();
        if elapsed > 100 {
            if let Some(action) = &self.avatar.action {
                let mut amount = action.finish - self.meta.current_tick;
                if amount > 0.5 {
                    amount = 0.5;
                }
                self.meta.current_tick += amount;
            }
        }
        // println!("{}", self.meta.current_tick);
    }
}
