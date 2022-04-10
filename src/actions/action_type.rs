#![allow(dead_code)]

use geometry::direction::Direction;
use map::Passage;
use world::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
    Wielding(Direction),
    Dropping(usize, Direction),
    Digging(Direction),
}

impl ActionType {
    pub fn name(&self, world: &World) -> String {
        match self {
            ActionType::SkippingTime => "skip time".to_string(),
            ActionType::Walking(dir) => {
                let pos = world.avatar.pos + dir;
                format!(
                    "walk through {}",
                    world.get_tile(pos).unwrap().terrain.name()
                )
            }
            ActionType::Wielding(dir) => {
                let pos = world.avatar.pos + dir;
                format!(
                    "pick up the {}",
                    world
                        .get_tile(pos)
                        .unwrap()
                        .items
                        .last()
                        .unwrap()
                        .item_type
                        .name()
                )
            }
            ActionType::Dropping(i, _) => {
                format!(
                    "drop the {}",
                    world.avatar.wield.get(*i).unwrap().item_type.name()
                )
            }
            ActionType::Digging(dir) => {
                let pos = world.avatar.pos + dir;
                format!("dig the {}", world.get_tile(pos).unwrap().terrain.name())
            }
        }
    }

    pub fn verb(&self) -> &str {
        match self {
            ActionType::SkippingTime => "waiting",
            ActionType::Walking(_) => "walking",
            ActionType::Wielding(_) => "picking up",
            ActionType::Dropping(..) => "dropping",
            ActionType::Digging(_) => "digging",
        }
    }

    pub fn length(&self, world: &World) -> u128 {
        match self {
            ActionType::SkippingTime => 1,
            ActionType::Walking(dir) => {
                // TODO: check avatar perks for calculating speed
                let pos = world.avatar.pos + dir;
                match world.get_tile(pos).unwrap().terrain.pass() {
                    Passage::Passable(length) => length.round() as u128,
                    Passage::Unpassable => 0,
                }
            }
            ActionType::Wielding(dir) => {
                let pos = world.avatar.pos + dir;
                if let Some(item) = world
                    .get_tile(pos)
                    .unwrap()
                    .items
                    .last()
                    .map(|i| i.item_type.clone())
                {
                    item.wield_time(&world.avatar).round() as u128
                } else {
                    0
                }
            }
            ActionType::Dropping(i, dir) => {
                if let Some(item) = world.avatar.wield.get(*i) {
                    let k = if matches!(dir, Direction::Here) {
                        1.0
                    } else {
                        1.5
                    };
                    (item.item_type.drop_time() * k).round() as u128
                } else {
                    0
                }
            }
            ActionType::Digging(_) => {
                // TODO: check tool quality, avatar perks and tile terrain
                1000
            }
        }
    }

    pub fn is_possible(&self, world: &World) -> bool {
        match self {
            ActionType::SkippingTime => true,
            ActionType::Walking(dir) => {
                let pos = world.avatar.pos + dir;
                world.get_tile(pos).unwrap().terrain.is_walkable()
            }
            ActionType::Wielding(dir) => {
                if !world.avatar.wield.is_empty() {
                    return false;
                }
                let pos = world.avatar.pos + dir;
                !world.get_tile(pos).unwrap().items.is_empty()
            }
            ActionType::Dropping(_, dir) => {
                let pos = world.avatar.pos + dir;
                world.get_tile(pos).unwrap().terrain.is_walkable()
            }
            ActionType::Digging(dir) => {
                let pos = world.avatar.pos + dir;
                world.get_tile(pos).unwrap().terrain.is_diggable()
            }
        }
    }
}
