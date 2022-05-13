use geometry::direction::Direction;

use super::super::super::map::item::{ItemInteract, ItemView};
use super::super::super::{Avatar, World};
use super::super::action_impl::ActionImpl;
use super::super::ActionPossibility::{self, No, Yes};
use super::super::{Action, ActionResult};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Wield {
    pub dir: Direction,
}

impl ActionImpl for Wield {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if !actor.wield.is_empty() {
            return No("You already have something in your hands".to_string());
        }
        let pos = actor.pos + self.dir;
        if let Some(item) = world.map().get_tile(pos).items.last() {
            Yes(item.wield_time(actor).round() as u32)
        } else {
            No("There is nothing to pick up".to_string())
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) -> Option<ActionResult> {
        let item = world
            .map()
            .get_tile_mut(action.owner(world).pos + self.dir)
            .items
            .pop();
        if let Some(item) = item {
            let name = item.name();
            action.owner_mut(world).wield.push(item);
            Some(ActionResult::LogMessage(format!(
                "{} wield the {}",
                action.owner(world).name_for_actions(),
                name
            )))
        } else {
            None
        }
    }
}
