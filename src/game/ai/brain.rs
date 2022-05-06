use game::actions::ActionType;
use game::World;

pub trait Brain {
    fn plan(&mut self, world: &World);
    fn get_action(&self) -> Option<ActionType>;
}
