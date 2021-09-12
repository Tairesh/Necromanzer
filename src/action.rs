use direction::Direction;
use geometry::DIR8;
use map::Passage;
use rand::seq::SliceRandom;
use std::cell::RefMut;
use world::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
    Wielding(Direction),
    Dropping(Direction),
    Digging(Direction),
}

impl ActionType {
    pub fn name(&self, world: &mut RefMut<World>) -> String {
        match self {
            ActionType::SkippingTime => "skip time".to_string(),
            ActionType::Walking(dir) => {
                let pos = world.avatar.pos + dir;
                format!("walk through {}", world.load_tile(pos).terrain.name())
            }
            ActionType::Wielding(dir) => {
                let pos = world.avatar.pos + dir;
                format!(
                    "pick up the {}",
                    world.load_tile(pos).items.last().unwrap().item_type.name()
                )
            }
            ActionType::Dropping(_) => {
                format!(
                    "drop the {}",
                    world.avatar.wield.last().unwrap().item_type.name()
                )
            }
            ActionType::Digging(dir) => {
                let pos = world.avatar.pos + dir;
                format!("dig the {}", world.load_tile(pos).terrain.name())
            }
        }
    }

    pub fn verb(&self) -> &str {
        match self {
            ActionType::SkippingTime => "waiting",
            ActionType::Walking(_) => "walking",
            ActionType::Wielding(_) => "picking up",
            ActionType::Dropping(_) => "dropping",
            ActionType::Digging(_) => "digging",
        }
    }

    pub fn length(&self, world: &mut RefMut<World>) -> u64 {
        match self {
            ActionType::SkippingTime => 1,
            ActionType::Walking(dir) => {
                // TODO: check avatar perks for calculating speed
                let pos = world.avatar.pos + dir;
                match world.load_tile(pos).terrain.pass() {
                    Passage::Passable(length) => length.round() as u64,
                    Passage::Unpassable => 0,
                }
            }
            ActionType::Wielding(dir) => {
                let pos = world.avatar.pos + dir;
                if let Some(item) = world
                    .load_tile(pos)
                    .items
                    .last()
                    .map(|i| i.item_type.clone())
                {
                    item.wield_time(&world.avatar).round() as u64
                } else {
                    0
                }
            }
            ActionType::Dropping(dir) => {
                if let Some(item) = world.avatar.wield.last() {
                    let k = if matches!(dir, Direction::Here) {
                        1.0
                    } else {
                        1.5
                    };
                    (item.item_type.drop_time() * k).round() as u64
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

    pub fn is_possible(&self, world: &mut RefMut<World>) -> bool {
        match self {
            ActionType::SkippingTime => true,
            ActionType::Walking(dir) => {
                let pos = world.avatar.pos + dir;
                world.load_tile(pos).terrain.is_walkable()
            }
            ActionType::Wielding(dir) => {
                let pos = world.avatar.pos + dir;
                !world.load_tile(pos).items.is_empty()
            }
            ActionType::Dropping(dir) => {
                let pos = world.avatar.pos + dir;
                world.load_tile(pos).terrain.is_walkable()
            }
            ActionType::Digging(dir) => {
                let pos = world.avatar.pos + dir;
                world.load_tile(pos).terrain.is_diggable()
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Action {
    pub action: ActionType,
    pub finish: u64,
}

impl Action {
    pub fn new(finish: u64, action: ActionType) -> Self {
        Self { action, finish }
    }

    pub fn act(&self, world: &mut World) {
        // TODO: add log messages
        match self.action {
            ActionType::SkippingTime => {}
            ActionType::Walking(dir) => {
                world.move_avatar(dir);
            }
            ActionType::Wielding(dir) => {
                if let Some(item) = world.load_tile_mut(world.avatar.pos + dir).items.pop() {
                    world.avatar.wield.push(item);
                }
            }
            ActionType::Dropping(dir) => {
                if let Some(item) = world.avatar.wield.pop() {
                    world.load_tile_mut(world.avatar.pos + dir).items.push(item);
                }
            }
            ActionType::Digging(dir) => {
                let pos = world.avatar.pos + dir;
                let items = world.load_tile_mut(pos).dig();
                if !items.is_empty() {
                    let mut rng = rand::thread_rng();
                    let places: Vec<(i32, i32)> = DIR8
                        .iter()
                        .filter(|d| {
                            pos + *d != world.avatar.pos
                                && world.load_tile(pos + *d).terrain.is_walkable()
                        })
                        .copied()
                        .collect();
                    for item in items {
                        let delta = places.choose(&mut rng).unwrap();
                        world.load_tile_mut(pos + delta).items.push(item);
                    }
                }
            }
        }
        world.avatar.action = None;
    }
}
