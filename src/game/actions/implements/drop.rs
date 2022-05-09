use geometry::direction::Direction;

use super::super::super::map::item::{ItemInteract, ItemView};
use super::super::super::map::terrain::{TerrainInteract, TerrainView};
use super::super::super::{Avatar, World};
use super::super::action_impl::ActionImpl;
use super::super::ActionPossibility::{No, Yes};
use super::super::{Action, ActionPossibility, ActionResult};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Drop {
    pub item_id: usize,
    pub dir: Direction,
}

impl ActionImpl for Drop {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if actor.wield.is_empty() {
            return No("You have nothing to drop".to_string());
        }
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            if !tile.terrain.is_passable() {
                return No(format!("You can't put items on {}", tile.terrain.name()));
            }
        }

        if let Some(item) = actor.wield.get(self.item_id) {
            let k = if matches!(self.dir, Direction::Here) {
                1.0
            } else {
                1.5
            };
            Yes((item.drop_time(actor) * k).round() as u32)
        } else {
            No("Item doesn't exists".to_string())
        }
    }
    fn on_finish(&self, action: &Action, world: &mut World) -> Option<ActionResult> {
        let item = action.owner_mut(world).wield.remove(self.item_id);
        let name = item.name();
        world
            .load_tile_mut(action.owner(world).pos + self.dir)
            .items
            .push(item);
        Some(ActionResult::LogMessage(format!(
            "{} drop the {}",
            action.owner(world).name_for_actions(),
            name
        )))
    }
}
