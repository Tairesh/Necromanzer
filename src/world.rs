use chunk::Chunk;
use human::character::Character;
use savefile::save;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct World {
    path: PathBuf,
    pub meta: WorldMeta,
    pub avatar: Character,
    chunks: HashMap<(i32, i32), Chunk>,
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

    pub fn load_chunk(&mut self, pos: (i32, i32)) -> &Chunk {
        self.chunks
            .entry(pos)
            .or_insert_with_key(|(x, y)| Chunk::generate(*x, *y))
    }
}
