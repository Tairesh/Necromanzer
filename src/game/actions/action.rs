use super::{ActionResult, ActionType};
use colors::Colors;
use game::actions::action_type::ActionPossibility;
use game::{Avatar, World};
use geometry::direction::{Direction, DIR8};
use map::item::ItemType;
use rand::seq::SliceRandom;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Action {
    pub typ: ActionType,
    pub finish: u128,
}

impl Action {
    pub fn new(typ: ActionType, world: &World) -> Result<Self, String> {
        match typ.is_possible(world) {
            ActionPossibility::Yes => {
                let finish = world.meta.current_tick + typ.length(world) as u128;
                Ok(Self { typ, finish })
            }
            ActionPossibility::No(s) => Err(s),
        }
    }

    /// called every tick
    pub fn act(&self, world: &mut World) -> Option<ActionResult> {
        let steps = (self.finish - world.meta.current_tick) as u32;
        if steps == self.typ.length(world) {
            match self.typ {
                ActionType::Digging(..) => {
                    Some(ActionResult::LogMessage("You start digging".to_string()))
                }
                _ => None,
            }
        } else if steps == 0 {
            // finish
            match self.typ {
                ActionType::SkippingTime => None,
                ActionType::Walking(dir) => {
                    // TODO: move other units
                    world.move_avatar(0, dir);
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
                ActionType::Reading(dir) => {
                    let pos = world.player().pos + dir;
                    Some(ActionResult::LogMessage(world.load_tile(pos).read()))
                }
                ActionType::Animate(dir) => {
                    let pos = world.player().pos + dir;
                    if let Some(i) = world
                        .load_tile(pos)
                        .items
                        .iter()
                        .position(|i| matches!(i.item_type, ItemType::Corpse(..)))
                    {
                        let body = world.load_tile_mut(pos).items.remove(i);
                        if let ItemType::Corpse(character, body) = body.item_type {
                            let name = character.age_name().to_owned();
                            let zombie = Avatar::zombie(character, body, pos);
                            world.add_unit(zombie);
                            return Some(ActionResult::ColoredLogMessage(
                                format!("Zombie {} stands up!", name),
                                Colors::LIGHT_PINK,
                            ));
                        }
                    }
                    None
                }
            }
        } else {
            None
        }
    }
}
