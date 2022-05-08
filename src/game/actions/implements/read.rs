use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, No, Yes};
use game::actions::{Action, ActionResult};
use game::{Avatar, World};
use geometry::direction::Direction;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Read {
    pub dir: Direction,
}

impl ActionImpl for Read {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        // TODO: check skill of reading, and probably even another languages
        if let Some(tile) = world.get_tile(pos) {
            if tile.is_readable() {
                return Yes(tile.read().len() as u32);
            }
        }

        No("There is nothing to read".to_string())
    }
    fn on_finish(&self, action: &Action, world: &mut World) -> Option<ActionResult> {
        let pos = action.owner(world).pos + self.dir;
        Some(ActionResult::LogMessage(world.load_tile(pos).read()))
    }
}
