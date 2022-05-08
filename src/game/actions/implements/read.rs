use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, No, Yes};
use game::{Avatar, World};
use geometry::direction::Direction;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Read {
    pub dir: Direction,
}

impl ActionImpl for Read {
    fn length(&self, actor: &Avatar, world: &World) -> u32 {
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            if tile.is_readable() {
                return tile.read().len() as u32;
            }
        }

        0
    }

    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        // TODO: check skill of reading, and probably even another languages
        if let Some(tile) = world.get_tile(pos) {
            if tile.is_readable() {
                return Yes;
            }
        }

        No("There is nothing to read".to_string())
    }
}
