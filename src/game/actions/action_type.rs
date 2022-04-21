#![allow(dead_code)]

use game::World;
use geometry::direction::Direction;
use map::item::ItemType;
use map::Passage;

pub enum ActionPossibility {
    Yes,
    No(String),
}

use self::ActionPossibility::{No, Yes};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
    Wielding(Direction),
    Dropping(usize, Direction),
    Digging(Direction),
    Reading(Direction),
    Animate(Direction),
}

// TODO: get rid of all these unwraps
impl ActionType {
    pub fn length(&self, world: &World) -> u32 {
        match self {
            ActionType::SkippingTime => 1,
            ActionType::Walking(dir) => {
                // TODO: check avatar perks for calculating speed
                let pos = world.player().pos + dir;
                match world.get_tile(pos).unwrap().terrain.pass() {
                    Passage::Passable(length) => length.round() as u32,
                    Passage::Unpassable => 0,
                }
            }
            ActionType::Wielding(dir) => {
                let pos = world.player().pos + dir;
                if let Some(item) = world
                    .get_tile(pos)
                    .unwrap()
                    .items
                    .last()
                    .map(|i| i.item_type.clone())
                {
                    item.wield_time(world.player()).round() as u32
                } else {
                    0
                }
            }
            ActionType::Dropping(i, dir) => {
                if let Some(item) = world.player().wield.get(*i) {
                    let k = if matches!(dir, Direction::Here) {
                        1.0
                    } else {
                        1.5
                    };
                    (item.item_type.drop_time() * k).round() as u32
                } else {
                    0
                }
            }
            ActionType::Digging(_) => {
                // TODO: check tool quality, avatar perks and tile terrain
                1000
            }
            ActionType::Reading(dir) => {
                let pos = world.player().pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    if tile.is_readable() {
                        return tile.read().len() as u32;
                    }
                }

                0
            }
            ActionType::Animate(dir) => {
                let pos = world.player().pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    if tile
                        .items
                        .iter()
                        .any(|i| matches!(i.item_type, ItemType::Corpse(..)))
                    {
                        return 5000; // TODO: use coprse mass
                    }
                }

                0
            }
        }
    }

    pub fn is_possible(&self, world: &World) -> ActionPossibility {
        match self {
            ActionType::SkippingTime => Yes,
            ActionType::Walking(dir) => {
                let pos = world.player().pos + dir;
                let tile = world.get_tile(pos).unwrap();
                if !tile.terrain.is_walkable() {
                    return No(format!("You can't walk to the {}", tile.terrain.name()));
                }
                if !tile.units.is_empty() {
                    let i = tile.units.iter().copied().next().unwrap();
                    return No(format!(
                        "{} is on the way",
                        world.units.get(i).unwrap().character.name
                    ));
                }
                Yes
            }
            ActionType::Wielding(dir) => {
                if !world.player().wield.is_empty() {
                    return ActionPossibility::No(
                        "You already have something in your hands".to_string(),
                    );
                }
                let pos = world.player().pos + dir;
                if world.get_tile(pos).unwrap().items.is_empty() {
                    return ActionPossibility::No("There is nothing to pick up".to_string());
                }
                ActionPossibility::Yes
            }
            ActionType::Dropping(_, dir) => {
                if world.player().wield.is_empty() {
                    return ActionPossibility::No("You have nothing to drop".to_string());
                }
                let pos = world.player().pos + dir;
                let terrain = &world.get_tile(pos).unwrap().terrain;
                if !terrain.is_walkable() {
                    return ActionPossibility::No(format!(
                        "You can't put items on {}",
                        terrain.name()
                    ));
                }
                ActionPossibility::Yes
            }
            ActionType::Digging(dir) => {
                let pos = world.player().pos + dir;
                let terrain = &world.get_tile(pos).unwrap().terrain;
                if !terrain.is_diggable() {
                    return ActionPossibility::No(format!("You can't dig the {}", terrain.name()));
                }
                ActionPossibility::Yes
            }
            ActionType::Reading(dir) => {
                let pos = world.player().pos + dir;
                // TODO: check skill of reading, and probably even another languages
                if let Some(tile) = world.get_tile(pos) {
                    if tile.is_readable() {
                        return ActionPossibility::Yes;
                    }
                }

                ActionPossibility::No("There is nothing to read".to_string())
            }
            ActionType::Animate(dir) => {
                let pos = world.player().pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    if tile
                        .items
                        .iter()
                        .any(|i| matches!(i.item_type, ItemType::Corpse(..)))
                    {
                        return Yes;
                    }
                }

                No("There is nothing to rise".to_string())
            }
        }
    }
}
