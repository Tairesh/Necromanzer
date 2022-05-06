use game::actions::ActionType;
use game::ai::brain::Brain;
use game::World;
use geometry::direction::Direction;
use rand::{thread_rng, Rng};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ZombieBrain {}

impl Brain for ZombieBrain {
    fn plan(&mut self, _world: &World) {}

    fn get_action(&self) -> Option<ActionType> {
        let mut rng = thread_rng();

        Some(ActionType::Walking(match rng.gen_range(0..5) {
            0 => Direction::East,
            1 => Direction::West,
            2 => Direction::North,
            3 => Direction::South,
            4 => Direction::Here,
            _ => unreachable!(),
        }))
    }
}
