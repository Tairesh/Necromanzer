use game::actions::ActionPossibility;
use game::{Avatar, World};

use super::{ActionImpl, ActionResult, ActionType};

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
                    finish,
                    length,
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

    /// called every tick
    pub fn act(&self, world: &mut World) -> Option<ActionResult> {
        if let ActionPossibility::No(reason) = self.typ.is_possible(self.owner(world), world) {
            return Some(ActionResult::CancelAction(reason));
        }

        let steps = (self.finish - world.meta.current_tick) as u32;
        if steps == self.length {
            self.typ.on_start(self, world)
        } else if steps == 0 {
            self.typ.on_finish(self, world)
        } else {
            self.typ.on_step(self, world)
        }
    }
}
