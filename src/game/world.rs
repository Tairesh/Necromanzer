#![allow(dead_code)]

use assets::game_data::GameData;
use fov::{field_of_view_set, FovMap};
use game::actions::{owner_mut, Action, ActionResult};
use game::ai::brain::Brain;
use game::avatar::Soul;
use game::Avatar;
use geometry::direction::{Direction, TwoDimDirection};
use geometry::point::Point;
use map::chunk::Chunk;
use map::item::ItemView;
use map::pos::{ChunkPos, TilePos};
use map::terrain::TerrainView;
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
    pub units: Vec<Avatar>,
    pub loaded_units: HashSet<usize>,
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub changed: HashSet<ChunkPos>,
    game_data: Rc<GameData>,
    pub fov: HashSet<Point>,
    pub fov_dirty: bool,
}

impl World {
    pub fn new(
        meta: Meta,
        game_view: GameView,
        units: Vec<Avatar>,
        chunks: HashMap<ChunkPos, Chunk>,
        game_data: Rc<GameData>,
    ) -> Self {
        let changed = chunks.keys().copied().collect();
        let loaded_units = HashSet::from([0]);
        let mut world = Self {
            meta,
            game_view,
            units,
            loaded_units,
            chunks,
            changed,
            game_data,
            fov: HashSet::default(),
            fov_dirty: true,
        };
        world.load_units();
        world.calc_fov();
        world
    }

    pub fn init(mut self) -> Self {
        self.kill_grass(self.player().pos, 13, 0.8);
        self
    }

    fn calc_fov(&mut self) {
        self.fov = field_of_view_set(self.player().pos.into(), 64, self);
        self.fov_dirty = true;
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

    pub fn get_unit(&self, unit_id: usize) -> &Avatar {
        self.units.get(unit_id).unwrap()
    }

    pub fn player(&self) -> &Avatar {
        self.get_unit(0)
    }

    pub fn player_mut(&mut self) -> &mut Avatar {
        self.units.get_mut(0).unwrap()
    }

    pub fn move_avatar(&mut self, unit_id: usize, dir: Direction) {
        let mut pos = self.units.get(unit_id).unwrap().pos;
        let (old_chunk, _) = pos.chunk_and_pos();
        self.load_tile_mut(pos).off_step(unit_id);
        pos += dir;
        if let Some(unit) = self.units.get_mut(unit_id) {
            unit.pos = pos;
            if let Ok(dir) = TwoDimDirection::try_from(dir) {
                unit.vision = dir;
            }
        }
        self.load_tile_mut(pos).on_step(unit_id);
        if unit_id == 0 && old_chunk != pos.chunk_and_pos().0 {
            self.load_units();
        }
        if unit_id == 0 {
            self.calc_fov();
        }
    }

    // TODO: move this somewhere else
    pub fn this_is(&self, pos: TilePos, multiline: bool) -> String {
        if let Some(tile) = self.get_tile(pos) {
            let mut this_is = format!("This is the {}", tile.terrain.name());
            if multiline {
                this_is = this_is.replace(". ", ".\n");
            }

            if !tile.items.is_empty() || !tile.units.is_empty() {
                this_is.push(if multiline { '\n' } else { ' ' });
                this_is.push_str("Here you see: ");
                if multiline {
                    this_is.push('\n');
                }
            }

            let mut items: Vec<String> = Vec::with_capacity(tile.items.len() + tile.units.len());
            if !tile.items.is_empty() {
                items.append(
                    &mut tile
                        .items
                        .iter()
                        .map(|item| {
                            (if multiline { " - " } else { "" }).to_string() + item.name().as_str()
                        })
                        .collect(),
                );
            }
            if !tile.units.is_empty() {
                items.append(
                    &mut tile
                        .units
                        .iter()
                        .copied()
                        .map(|i| {
                            let unit = self.units.get(i).unwrap();
                            (if multiline { " - " } else { "" }).to_string()
                                + unit.character.name.as_str()
                        })
                        .collect(),
                );
            }
            this_is += items.join(if multiline { "\n" } else { ", " }).as_str();

            this_is
        } else {
            String::new()
        }
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
    fn act(&mut self) -> Option<ActionResult> {
        let mut result = None;

        let actions: Vec<Action> = self
            .units
            .iter()
            .filter(|u| u.action.is_some())
            .map(|u| u.action.as_ref().unwrap().clone())
            .collect();
        for action in actions {
            if action.finish >= self.meta.current_tick {
                let res = action.act(self);
                if action.owner == 0 {
                    result = res;
                }
            }
            if self.meta.current_tick == action.finish {
                owner_mut(action.owner, self).action = None;
            }
        }

        result
    }

    pub fn add_unit(&mut self, unit: Avatar) {
        let pos = unit.pos;
        self.units.push(unit);
        self.load_units();
        let new_id = self.units.len() - 1;
        self.load_tile_mut(pos).units.insert(new_id);
    }

    fn load_units(&mut self) {
        self.loaded_units.clear();
        let center = self.player().pos;
        for (i, unit) in self.units.iter().enumerate() {
            let pos = unit.pos;
            let dist = pos.square_dist_to(center);
            if dist <= Self::BUBBLE_SQUARE_RADIUS {
                self.loaded_units.insert(i);
            } else {
                self.loaded_units.remove(&i);
            }
        }
    }

    pub const BUBBLE_SQUARE_RADIUS: u32 = 128 * 128;
    pub const SPEND_LIMIT: u32 = 100;

    pub fn tick(&mut self) -> Vec<ActionResult> {
        let mut actions = vec![];

        // make zero ticks actions
        if let Some(action) = self.act() {
            actions.push(action);
        }

        let mut spend = 0;
        while self.player().action.is_some() && spend < Self::SPEND_LIMIT {
            self.meta.current_tick += 1;
            spend += 1;
            if let Some(action) = self.act() {
                actions.push(action);
            }

            let mut unit_wants_actions = HashMap::new();
            for (unit_id, unit) in self.units.iter_mut().skip(1).enumerate() {
                if unit.action.is_none() {
                    if let Soul::Npc(brain) = &unit.soul {
                        if let Some(action_type) = brain.get_action() {
                            // +1 is because we skipped first one in enumeration
                            unit_wants_actions.insert(unit_id + 1, action_type);
                        }
                    }
                }
            }
            for (unit_id, typ) in unit_wants_actions.into_iter() {
                self.units.get_mut(unit_id).unwrap().action = Action::new(unit_id, typ, self).ok();
            }
            // self.kill_grass(self.player().pos, 13, 0.01);
        }

        actions
    }
}

impl FovMap for World {
    fn is_transparent(&self, point: Point) -> bool {
        self.get_tile(point.into())
            .map(|t| t.terrain.is_transparent())
            .unwrap_or(false)
    }
}

#[cfg(test)]
pub mod tests {
    use super::World;
    use assets::game_data::GameData;
    use game::actions::{Action, ActionType};
    use game::Avatar;
    use geometry::direction::Direction;
    use geometry::point::Point;
    use human::body::{Body, Freshness};
    use human::character::Character;
    use human::gender::Gender;
    use human::main_hand::MainHand;
    use human::skin_tone::SkinTone;
    use map::pos::TilePos;
    use map::terrain::TerrainView;
    use map::terrains::{Boulder, BoulderSize, Dirt};
    use savefile::{GameView, Meta};
    use std::collections::HashMap;
    use std::rc::Rc;

    pub fn prepare_world() -> World {
        let mut world = World::new(
            Meta::new("test", "test"),
            GameView::default(),
            vec![Avatar::player(
                Character::new(
                    "player",
                    Gender::Female,
                    16,
                    MainHand::Left,
                    SkinTone::Espresso,
                ),
                TilePos::new(0, 0),
            )],
            HashMap::new(),
            Rc::new(GameData::load().unwrap()),
        );
        world.load_tile(TilePos::new(0, 0));

        world
    }

    #[test]
    pub fn test_moving_other_unit() {
        let mut world = prepare_world();
        let character = Character::new(
            "zombie",
            Gender::Female,
            16,
            MainHand::Left,
            SkinTone::Espresso,
        );
        let body = Body::human(&character, Freshness::Rotten);
        let zombie = Avatar::zombie(character, body, TilePos::new(1, 0));
        world.load_tile(TilePos::new(1, 0));
        world.load_tile(TilePos::new(1, -1)); // TODO: autoload tiles when trying to move via AI system
        world.add_unit(zombie);

        assert_eq!(2, world.units.len());
        world.load_tile_mut(TilePos::new(2, 0)).terrain = Dirt::default().into();
        let typ = ActionType::Walking(Direction::East);
        let length = typ.length(1, &world);
        let action = Action::new(1, typ, &world).unwrap();
        if let Some(zombie) = world.units.get_mut(1) {
            zombie.action = Some(action);
        }
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert_eq!(TilePos::new(1, 0), world.units.get(1).unwrap().pos);
        for _ in 0..length {
            world.player_mut().action =
                Some(Action::new(0, ActionType::SkippingTime, &world).unwrap());
            world.tick();
        }
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert_eq!(TilePos::new(2, 0), world.units.get(1).unwrap().pos)
    }

    #[test]
    pub fn test_fov() {
        let mut world = prepare_world();
        assert!(world.fov.contains(&world.player().pos.into()));

        world.load_tile_mut(TilePos::new(1, 0)).terrain = Dirt::default().into();
        world.load_tile_mut(TilePos::new(2, 0)).terrain = Boulder::new(BoulderSize::Huge).into();
        assert!(!world.load_tile(TilePos::new(2, 0)).terrain.is_transparent());
        world.load_tile_mut(TilePos::new(3, 0));

        world.move_avatar(0, Direction::East);
        assert!(world.fov.contains(&Point::new(1, 0)));
        assert!(world.fov.contains(&Point::new(2, 0)));
        assert!(!world.fov.contains(&Point::new(3, 0)));
    }
}
