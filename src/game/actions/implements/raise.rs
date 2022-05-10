use colors::Colors;
use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, No, Yes};
use game::actions::{Action, ActionResult};
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

    fn on_finish(&self, action: &Action, world: &mut World) -> Option<ActionResult> {
        let pos = action.owner(world).pos + self.dir;
        if let Some(i) = world
            .load_tile(pos)
            .items
            .iter()
            .position(|i| matches!(i, Item::Corpse(..)))
        {
            let body = world.load_tile_mut(pos).items.remove(i);
            if let Item::Corpse(corpse) = body {
                let name = corpse.character.age_name().to_owned();
                let zombie = Avatar::zombie(corpse.character, corpse.body, pos);
                world.add_unit(zombie);
                return Some(ActionResult::ColoredLogMessage(
                    format!("Zombie {} stands up!", name),
                    Colors::LIGHT_PINK,
                ));
            }
        }
        None
    }
}