use enum_dispatch::enum_dispatch;
use game::actions::ActionPossibility;
use game::{Avatar, World};

#[enum_dispatch]
pub trait ActionImpl {
    fn length(&self, actor: &Avatar, world: &World) -> u32;
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility;
}
