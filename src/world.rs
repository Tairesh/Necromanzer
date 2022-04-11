#![allow(dead_code)]

use actions::ActionResult;
use assets::game_data::GameData;
use avatar::Avatar;
use geometry::direction::{Direction, TwoDimDirection};
use map::chunk::Chunk;
use map::pos::{ChunkPos, TilePos};
use map::tile::Tile;
use rand::Rng;
use savefile::{GameView, Meta};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::rc::Rc;
use {geometry, savefile};

#[derive(Debug)]
pub struct World {
    pub meta: Meta,
    pub game_view: GameView,
    pub avatar: Avatar,
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub changed: HashSet<ChunkPos>,
    game_data: Rc<GameData>,
}

impl World {
    pub fn new(
        meta: Meta,
        game_view: GameView,
        avatar: Avatar,
        chunks: HashMap<ChunkPos, Chunk>,
        game_data: Rc<GameData>,
    ) -> Self {
        let changed = chunks.keys().copied().collect();
        Self {
            meta,
            game_view,
            avatar,
            chunks,
            changed,
            game_data,
        }
    }

    pub fn init(mut self) -> Self {
        self.kill_grass(self.avatar.pos, 13, 0.8);
        self
    }

    pub fn save(&mut self) {
        savefile::save(self)
            .map_err(|e| panic!("Error on saving world to {:?}: {:?}", self.meta.path, e))
            .ok();
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }

    pub fn load_chunk(&mut self, pos: ChunkPos) -> &Chunk {
        let seed = self.meta.seed.clone();
        let game_data = self.game_data.clone();
        self.chunks
            .entry(pos)
            .or_insert_with_key(|pos| Chunk::generate(seed, &game_data, *pos))
    }

    pub fn load_chunk_mut(&mut self, pos: ChunkPos) -> &mut Chunk {
        let seed = self.meta.seed.clone();
        let game_data = self.game_data.clone();
        self.changed.insert(pos);
        self.chunks
            .entry(pos)
            .or_insert_with_key(|pos| Chunk::generate(seed, &game_data, *pos))
    }

    pub fn get_tile(&self, pos: TilePos) -> Option<&Tile> {
        let (chunk, pos) = pos.chunk_and_pos();
        let chunk = self.get_chunk(chunk)?;
        Some(&chunk.tiles[pos])
    }

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

    pub fn tiles_between(
        &mut self,
        left_top: TilePos,
        right_bottom: TilePos,
    ) -> Vec<(TilePos, &Tile)> {
        let (ChunkPos { x: lt_x, y: lt_y }, _) = left_top.chunk_and_pos();
        let (ChunkPos { x: rb_x, y: rb_y }, _) = right_bottom.chunk_and_pos();

        // TODO: Find a way to get rid of this shit
        for x in lt_x..=rb_x {
            for y in lt_y..=rb_y {
                let pos = ChunkPos::new(x, y);
                self.load_chunk(pos);
            }
        }

        let mut tiles =
            Vec::with_capacity(((rb_x - lt_x + 1) * (rb_y - lt_y + 1)) as usize * Chunk::USIZE);
        for x in lt_x..=rb_x {
            for y in lt_y..=rb_y {
                let pos = ChunkPos::new(x, y);
                let chunk = self.chunks.get(&pos).unwrap();
                for (i, tile) in chunk.tiles.iter().enumerate() {
                    tiles.push((TilePos::from(pos, i), tile));
                }
            }
        }
        tiles
    }

    pub fn move_avatar(&mut self, dir: Direction) {
        let pos = self.avatar.pos;
        self.load_tile_mut(pos).off_step();
        self.avatar.pos = pos + dir;
        if let Ok(dir) = TwoDimDirection::try_from(dir) {
            self.avatar.vision = dir;
        }
        self.load_tile_mut(self.avatar.pos).on_step();
    }

    pub fn kill_grass(&mut self, around: TilePos, diameter: u8, probability: f64) {
        for (dx, dy) in match diameter {
            7 => geometry::CIRCLE7.iter().copied(),
            9 => geometry::CIRCLE9.iter().copied(),
            11 => geometry::CIRCLE11.iter().copied(),
            13 => geometry::CIRCLE13.iter().copied(),
            _ => unimplemented!(),
        } {
            let k = (1.0 - (dx as f64).hypot(dy as f64) / ((diameter - 1) as f64 / 2.0))
                .min(1.0)
                .max(0.0);
            if rand::thread_rng().gen_bool(probability * k) {
                let pos = around + (dx, dy);
                self.load_tile_mut(pos).kill_grass();
            }
        }
    }

    /// Doing actions that should be done
    fn act(&mut self) {
        if let Some(action) = self.avatar.action {
            if action.finish >= self.meta.current_tick {
                if let Some(result) = action.act(self) {
                    match result {
                        ActionResult::LogMessage(msg) => {
                            println!("{}", msg);
                        }
                    }
                }
            }
            if self.meta.current_tick == action.finish {
                self.avatar.action = None;
            }
        }
    }

    fn every_tick(&mut self) {
        self.kill_grass(self.avatar.pos, 13, 0.01);
        self.act();
    }

    pub const SPEND_LIMIT: u32 = 100;

    pub fn tick(&mut self) {
        self.act();
        let mut spend = 0;
        while self.avatar.action.is_some() && spend < World::SPEND_LIMIT {
            self.meta.current_tick += 1;
            spend += 1;
            self.every_tick();
        }
    }
}
