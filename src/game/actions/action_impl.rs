use enum_dispatch::enum_dispatch;

use game::actions::{Action, ActionPossibility, ActionResult};
use game::{Avatar, World};

#[enum_dispatch]
pub trait ActionImpl {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility;
    fn on_start(&self, _action: &Action, _world: &mut World) -> Option<ActionResult> {
        None
    }
    fn on_step(&self, _action: &Action, _world: &mut World) -> Option<ActionResult> {
        None
    }
    fn on_finish(&self, _action: &Action, _world: &mut World) -> Option<ActionResult> {
        None
    }
}
