use direction::Direction;
use world::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
}

impl ActionType {
    pub fn length(&self, _world: &World) -> f64 {
        match self {
            ActionType::SkippingTime => 1.0,
            ActionType::Walking(_) => 10.0,
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
    pub fn new(world: &World, action: ActionType) -> Self {
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
