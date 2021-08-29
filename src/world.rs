use human::character::Character;
use savefile::save;
use std::path::PathBuf;

pub struct World {
    path: PathBuf,
    pub meta: WorldMeta,
    pub avatar: Character,
}

#[derive(Debug, Clone)]
pub struct WorldMeta {
    pub name: String,
    pub seed: String,
}

impl World {
    pub fn new(path: PathBuf, meta: WorldMeta, avatar: Character) -> Self {
        Self { path, meta, avatar }
    }

    pub fn save(&mut self) {
        save(&self.path, self)
            .map_err(|e| panic!("Error on saving world to {:?}: {}", self.path, e))
            .ok();
    }
}
