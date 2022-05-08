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
    fn length(&self, actor: &Avatar, world: &World) -> u32 {
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            return tile
                .items
                .iter()
                .filter(|i| matches!(i, Item::Corpse(..)))
                .map(|i| i.mass() / 10)
                .next()
                .unwrap_or(0);
        }

        0
    }

    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            if tile.items.iter().any(|i| matches!(i, Item::Corpse(..))) {
                return Yes;
            }
        }

        No("There is nothing to rise".to_string())
    }
}
