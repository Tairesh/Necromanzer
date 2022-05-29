use crate::geometry::Direction;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        map::item::{Item, ItemInteract},
        Avatar, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Raise {
    pub dir: Direction,
}

impl ActionImpl for Raise {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        if let Some(item) = world
            .map()
            .get_tile(pos)
            .items
            .iter()
            .find(|i| matches!(i, Item::Corpse(..)))
        {
            return Yes(item.mass() / 10);
        }

        No("There is nothing to rise".to_string())
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let pos = action.owner(world).pos + self.dir;
        let corpse_index = world
            .map()
            .get_tile(pos)
            .items
            .iter()
            .position(|i| matches!(i, Item::Corpse(..)));
        if let Some(i) = corpse_index {
            let body = world.map().get_tile_mut(pos).items.remove(i);
            if let Item::Corpse(corpse) = body {
                let name = corpse.character.age_name().to_owned();
                let zombie = Avatar::zombie(corpse.character, corpse.body, pos);
                world.add_unit(zombie);
                world.log().push(LogEvent::new(
                    format!("Zombie {} stands up!", name),
                    pos,
                    LogCategory::Success,
                ));
            }
        }
    }
}
