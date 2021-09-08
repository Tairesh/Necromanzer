use direction::Direction;
use map::Passage;
use std::cell::RefCell;
use std::rc::Rc;
use world::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
}

impl ActionType {
    pub fn name(&self, world: Rc<RefCell<World>>) -> String {
        match self {
            ActionType::SkippingTime => "skip time".to_string(),
            ActionType::Walking(dir) => {
                let pos = world.borrow().avatar.pos.add(*dir);
                format!(
                    "walk through {}",
                    world.borrow_mut().load_tile(pos).terrain.name()
                )
            }
        }
    }

    pub fn length(&self, world: Rc<RefCell<World>>) -> f64 {
        match self {
            ActionType::SkippingTime => 1.0,
            ActionType::Walking(dir) => {
                // TODO: check avatar perks for calculating speed
                let pos = world.borrow().avatar.pos.add(*dir);
                match world.borrow_mut().load_tile(pos).terrain.pass() {
                    Passage::Passable(length) => length as f64,
                    Passage::Unpassable => 0.0,
                }
            }
        }
    }

    pub fn is_possible(&self, world: Rc<RefCell<World>>) -> bool {
        match self {
            ActionType::SkippingTime => true,
            ActionType::Walking(dir) => {
                let tile = world.borrow().avatar.pos.add(*dir);
                world.borrow_mut().load_tile(tile).terrain.is_walkable()
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
        match self.action {
            ActionType::SkippingTime => {}
            ActionType::Walking(dir) => {
                world.move_avatar(dir);
            }
        }
        world.avatar.action = None;
    }
}
