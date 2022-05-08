use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, No, Yes};
use game::map::item::ItemInteract;
use game::map::terrain::{TerrainInteract, TerrainView};
use game::{Avatar, World};
use geometry::direction::Direction;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Drop {
    pub item_id: usize,
    pub dir: Direction,
}

impl ActionImpl for Drop {
    fn length(&self, actor: &Avatar, _world: &World) -> u32 {
        if let Some(item) = actor.wield.get(self.item_id) {
            let k = if matches!(self.dir, Direction::Here) {
                1.0
            } else {
                1.5
            };
            return (item.drop_time(actor) * k).round() as u32;
        }

        0
    }

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

        Yes
    }
}
