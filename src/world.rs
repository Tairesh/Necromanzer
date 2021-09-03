use assets::Assets;
use avatar::Avatar;
use direction::Direction;
use map::chunk::Chunk;
use map::pos::{ChunkPos, TilePos};
use map::tile::Tile;
use savefile::save;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct WorldMeta {
    pub name: String,
    pub seed: u64,
    pub current_tick: f64,
}

pub struct World {
    assets: Rc<RefCell<Assets>>,
    path: PathBuf,
    pub meta: WorldMeta,
    pub avatar: Avatar,
    chunks: HashMap<ChunkPos, Chunk>,
}

impl World {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        path: PathBuf,
        meta: WorldMeta,
        avatar: Avatar,
    ) -> Self {
        Self {
            assets,
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
        let assets = self.assets.clone();
        self.chunks
            .entry(pos)
            .or_insert_with_key(|pos| Chunk::generate(seed, assets, *pos))
    }

    pub fn load_chunk_mut(&mut self, pos: ChunkPos) -> &mut Chunk {
        let seed = self.meta.seed;
        let assets = self.assets.clone();
        self.chunks
            .entry(pos)
            .or_insert_with_key(|pos| Chunk::generate(seed, assets, *pos))
    }

    // TODO: load bunch of tiles with minimum load_chunk() calls for rendering
    pub fn load_tile(&mut self, pos: TilePos) -> &Tile {
        let (chunk, pos) = pos.chunk_and_pos();
        let chunk = self.load_chunk(chunk);
        &chunk.tiles[pos]
    }

    pub fn load_tile_mut(&mut self, pos: TilePos) -> &mut Tile {
        let (chunk, pos) = pos.chunk_and_pos();
        let chunk = self.load_chunk_mut(chunk);
        &mut chunk.tiles[pos]
    }

    pub fn move_avatar(&mut self, dir: Direction) {
        let pos = self.avatar.pos;
        self.load_tile_mut(pos).off_step();
        let pos = pos.add(dir);
        self.avatar.pos = pos;
        if let Some(dir) = dir.as_two_dimensional() {
            self.avatar.vision = dir;
        }
        self.load_tile_mut(pos).on_step();
    }

    pub fn tick(&mut self) {
        // doing actions that should be done
        if let Some(action) = self.avatar.action {
            if action.finish <= self.meta.current_tick {
                action.act(self);
            }
        }
        // TODO: should skip time
        if let Some(action) = &self.avatar.action {
            let mut amount = action.finish - self.meta.current_tick;
            if amount > 1.0 {
                amount = 1.0;
            }
            self.meta.current_tick += amount;
        }
        // println!("{}", self.meta.current_tick);
    }
}
