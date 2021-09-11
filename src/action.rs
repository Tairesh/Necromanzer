use direction::Direction;
use map::Passage;
use std::cell::RefMut;
use world::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
    Wielding(Direction),
    Dropping,
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
                    world.load_tile(pos).items.first().unwrap().name()
                )
            }
            ActionType::Dropping => {
                format!("drop the {}", world.avatar.wield.first().unwrap().name())
            }
        }
    }

    pub fn length(&self, world: &mut RefMut<World>) -> f64 {
        match self {
            ActionType::SkippingTime => 1.0,
            ActionType::Walking(dir) => {
                // TODO: check avatar perks for calculating speed
                let pos = world.avatar.pos + dir;
                match world.load_tile(pos).terrain.pass() {
                    Passage::Passable(length) => length as f64,
                    Passage::Unpassable => 0.0,
                }
            }
            ActionType::Wielding(dir) => {
                let pos = world.avatar.pos + dir;
                if let Some(item) = world.load_tile(pos).items.last().map(|i| i.item_type) {
                    item.wield_time(&world.avatar)
                } else {
                    0.0
                }
            }
            ActionType::Dropping => {
                if let Some(item) = world.avatar.wield.last() {
                    item.item_type.drop_time()
                } else {
                    0.0
                }
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
            ActionType::Dropping => {
                let pos = world.avatar.pos;
                world.load_tile(pos).terrain.is_walkable()
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Action {
    pub action: ActionType,
    pub finish: f64,
}

impl Action {
    pub fn new(finish: f64, action: ActionType) -> Self {
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
            ActionType::Dropping => {
                if let Some(item) = world.avatar.wield.pop() {
                    world.load_tile_mut(world.avatar.pos).items.push(item);
                }
            }
        }
        world.avatar.action = None;
    }
}
