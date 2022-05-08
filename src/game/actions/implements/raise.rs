use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, No, Yes};
use game::map::item::{Item, ItemInteract};
use game::{Avatar, World};
use geometry::direction::Direction;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Raise {
    pub dir: Direction,
}

impl ActionImpl for Raise {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            if let Some(item) = tile.items.iter().find(|i| matches!(i, Item::Corpse(..))) {
                return Yes(item.mass() / 10);
            }
        }

        No("There is nothing to rise".to_string())
    }
}
