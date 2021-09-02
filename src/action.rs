use avatar::Avatar;
use direction::Direction;
use maptile::TileBase;
use world::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
}

impl ActionType {
    pub fn length(&self, _avatar: &Avatar) -> f64 {
        match self {
            ActionType::SkippingTime => 1.0,
            ActionType::Walking(_) => 1.0,
        }
    }

    pub fn is_possible(&self, world: &mut World) -> bool {
        match self {
            ActionType::SkippingTime => true,
            ActionType::Walking(dir) => {
                let tile = world.avatar.pos.add(*dir);
                let tile = world.load_tile(tile);
                matches!(tile, TileBase::Dirt(_))
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
        let length = action.length(&world.avatar);
        Self {
            action,
            finish: world.meta.current_tick + length,
        }
    }

    pub fn act(&self, avatar: &mut Avatar) {
        match self.action {
            ActionType::SkippingTime => {}
            ActionType::Walking(dir) => {
                avatar.move_to(dir, false);
            }
        }
        avatar.action = None;
    }
}
