use direction::Direction;
use map::Passage;
use world::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
}

impl ActionType {
    pub fn name(&self, world: &mut World) -> String {
        match self {
            ActionType::SkippingTime => "skip time".to_string(),
            ActionType::Walking(dir) => {
                let pos = world.avatar.pos.add(*dir);
                let tile = world.load_tile(pos);
                format!("walk through {}", tile.terrain.name())
            }
        }
    }

    pub fn length(&self, world: &mut World) -> f64 {
        match self {
            ActionType::SkippingTime => 1.0,
            ActionType::Walking(dir) => {
                // TODO: check avatar perks for calculating speed
                let pos = world.avatar.pos.add(*dir);
                let tile = world.load_tile(pos);
                match tile.terrain.pass() {
                    Passage::Passable(length) => length as f64,
                    Passage::Unpassable => 0.0,
                }
            }
        }
    }

    pub fn is_possible(&self, world: &mut World) -> bool {
        match self {
            ActionType::SkippingTime => true,
            ActionType::Walking(dir) => {
                let tile = world.avatar.pos.add(*dir);
                world.load_tile(tile).terrain.is_walkable()
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
    pub fn new(world: &mut World, action: ActionType) -> Self {
        let length = action.length(world);
        Self {
            action,
            finish: world.meta.current_tick + length,
        }
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
