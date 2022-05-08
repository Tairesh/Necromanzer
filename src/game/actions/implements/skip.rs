use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, Yes};
use game::{Avatar, World};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Skip {}

impl ActionImpl for Skip {
    fn is_possible(&self, _actor: &Avatar, _world: &World) -> ActionPossibility {
        Yes(1)
    }
}
