use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, No, Yes};
use game::map::item::ItemInteract;
use game::{Avatar, World};
use geometry::direction::Direction;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Wield {
    pub dir: Direction,
}

impl ActionImpl for Wield {
    fn length(&self, actor: &Avatar, world: &World) -> u32 {
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            if let Some(item) = tile.items.last() {
                return item.wield_time(actor).round() as u32;
            }
        }

        0
    }

    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if !actor.wield.is_empty() {
            return No("You already have something in your hands".to_string());
        }
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            if tile.items.is_empty() {
                return No("There is nothing to pick up".to_string());
            }
        }
        Yes
    }
}
