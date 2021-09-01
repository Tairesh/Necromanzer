use avatar::Avatar;
use maptile::TilePos;
use world::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(TilePos),
}

impl ActionType {
    pub fn length(&self, _avatar: &Avatar) -> f64 {
        match self {
            ActionType::SkippingTime => 1.0,
            ActionType::Walking(_) => 1.0,
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
        let length = action.length(&world.avatar);
        Self {
            action,
            finish: world.meta.current_tick + length,
        }
    }

    pub fn act(&self, avatar: &mut Avatar) {
        match self.action {
            ActionType::SkippingTime => {}
            ActionType::Walking(pos) => {
                avatar.pos = pos;
            }
        }
        avatar.action = None;
    }
}
