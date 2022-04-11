use actions::action_result::ActionResult;
use actions::action_type::ActionType;
use geometry::direction::{Direction, DIR8};
use rand::seq::SliceRandom;
use world::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Action {
    pub action: ActionType,
    pub finish: u128,
}

impl Action {
    pub fn new(finish: u128, action: ActionType) -> Self {
        Self { action, finish }
    }

    /// called every tick
    pub fn act(&self, world: &mut World) -> Option<ActionResult> {
        let steps = self.finish - world.meta.current_tick;
        if steps == self.action.length(world) {
            match self.action {
                ActionType::Digging(..) => {
                    Some(ActionResult::LogMessage("You start digging".to_string()))
                }
                _ => None,
            }
        } else if steps == 0 {
            // finish
            match self.action {
                ActionType::SkippingTime => None,
                ActionType::Walking(dir) => {
                    // TODO: move other units
                    world.move_avatar(dir);
                    None
                }
                ActionType::Wielding(dir) => {
                    // TODO: other units
                    if let Some(item) = world.load_tile_mut(world.player().pos + dir).items.pop() {
                        world.player_mut().wield.push(item.clone());
                        Some(ActionResult::LogMessage(format!(
                            "You wield the {}",
                            item.item_type.name()
                        )))
                    } else {
                        None
                    }
                }
                ActionType::Dropping(i, dir) => {
                    let item = world.player().wield.get(i).unwrap().clone();
                    world
                        .load_tile_mut(world.player().pos + dir)
                        .items
                        .push(item.clone());
                    world.player_mut().wield.remove(i);
                    Some(ActionResult::LogMessage(format!(
                        "You drop the {}",
                        item.item_type.name()
                    )))
                }
                ActionType::Digging(dir) => {
                    let pos = world.player().pos + dir;
                    let items = world.load_tile_mut(pos).dig();
                    if !items.is_empty() {
                        let mut rng = rand::thread_rng();
                        let places: Vec<Direction> = DIR8
                            .iter()
                            .filter(|d| {
                                pos + *d != world.player().pos
                                    && world.load_tile(pos + *d).terrain.is_walkable()
                            })
                            .copied()
                            .collect();
                        for item in items {
                            let delta = places.choose(&mut rng).unwrap();
                            world.load_tile_mut(pos + delta).items.push(item);
                        }
                    }
                    Some(ActionResult::LogMessage("You dig a hole".to_string()))
                }
            }
        } else {
            None
        }
    }
}
