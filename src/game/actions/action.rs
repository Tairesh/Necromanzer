use game::actions::ActionPossibility;
use game::log::{LogCategory, LogEvent};
use game::{Avatar, World};

use super::{ActionImpl, ActionType};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Action {
    pub owner: usize,
    pub typ: ActionType,
    pub length: u32,
    pub finish: u128,
}

impl Action {
    pub fn new(owner: usize, typ: ActionType, world: &World) -> Result<Self, String> {
        match typ.is_possible(world.get_unit(owner), world) {
            ActionPossibility::Yes(length) => {
                let finish = world.meta.current_tick + length as u128;
                Ok(Self {
                    owner,
                    typ,
                    length,
                    finish,
                })
            }
            ActionPossibility::No(s) => Err(s),
        }
    }

    pub(crate) fn owner<'a>(&self, world: &'a World) -> &'a Avatar {
        world.get_unit(self.owner)
    }

    pub(crate) fn owner_mut<'a>(&self, world: &'a mut World) -> &'a mut Avatar {
        world.get_unit_mut(self.owner)
    }

    fn cancel_action(&self, world: &mut World, reason: String) {
        self.owner_mut(world).action = None;
        if self.owner == 0 {
            world.log().push(LogEvent::new(
                reason,
                self.owner(world).pos,
                LogCategory::Warning,
            ));
        }
    }

    /// called every tick
    pub fn act(&self, world: &mut World) {
        if let ActionPossibility::No(reason) = self.typ.is_possible(self.owner(world), world) {
            self.cancel_action(world, reason);
        }
        // TODO: draw stamina

        let steps = (self.finish - world.meta.current_tick) as u32;
        if steps == self.length {
            self.typ.on_start(self, world);
        } else if steps == 0 {
            self.typ.on_finish(self, world);
        } else {
            self.typ.on_step(self, world);
        }
    }
}
