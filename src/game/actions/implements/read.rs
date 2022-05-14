use game::actions::action_impl::ActionImpl;
use game::actions::Action;
use game::actions::ActionPossibility::{self, No, Yes};
use game::log::{LogCategory, LogEvent};
use game::{Avatar, World};
use geometry::direction::Direction;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Read {
    pub dir: Direction,
}

impl ActionImpl for Read {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        let mut map = world.map();
        // TODO: check skill of reading, and probably even another languages
        let tile = map.get_tile(pos);
        if tile.is_readable() {
            Yes(tile.read().len() as u32)
        } else {
            No("There is nothing to read".to_string())
        }
    }
    fn on_finish(&self, action: &Action, world: &mut World) {
        let pos = action.owner(world).pos + self.dir;
        world.log().push(LogEvent::new(
            world.map().get_tile(pos).read(),
            pos,
            LogCategory::Success,
        ));
    }
}
