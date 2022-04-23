use super::{ActionResult, ActionType};
use colors::Colors;
use game::actions::action_type::ActionPossibility;
use game::{Avatar, World};
use geometry::direction::{Direction, DIR8};
use map::item::ItemType;
use map::terrain::TerrainInteract;
use rand::seq::SliceRandom;

pub fn owner(owner: usize, world: &World) -> &Avatar {
    world.units.get(owner).unwrap()
}

pub fn owner_mut(owner: usize, world: &mut World) -> &mut Avatar {
    world.units.get_mut(owner).unwrap()
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Action {
    pub owner: usize,
    pub typ: ActionType,
    length: u32,
    pub finish: u128,
}

impl Action {
    pub fn new(owner: usize, typ: ActionType, world: &World) -> Result<Self, String> {
        match typ.is_possible(owner, world) {
            ActionPossibility::Yes => {
                let length = typ.length(owner, world);
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

    fn owner<'a>(&self, world: &'a World) -> &'a Avatar {
        owner(self.owner, world)
    }

    fn owner_mut<'a>(&self, world: &'a mut World) -> &'a mut Avatar {
        owner_mut(self.owner, world)
    }

    /// called every tick
    pub fn act(&self, world: &mut World) -> Option<ActionResult> {
        let steps = (self.finish - world.meta.current_tick) as u32;
        if steps == self.length {
            match self.typ {
                ActionType::Digging(..) => Some(ActionResult::LogMessage(format!(
                    "{} start digging",
                    self.owner(world).name_for_actions()
                ))),
                _ => None,
            }
        } else if steps == 0 {
            // finish
            match self.typ {
                ActionType::SkippingTime => None,
                ActionType::Walking(dir) => {
                    world.move_avatar(self.owner, dir);
                    None
                }
                ActionType::Wielding(dir) => {
                    if let Some(item) = world.load_tile_mut(self.owner(world).pos + dir).items.pop()
                    {
                        let name = item.item_type.name();
                        self.owner_mut(world).wield.push(item);
                        Some(ActionResult::LogMessage(format!(
                            "{} wield the {}",
                            self.owner(world).name_for_actions(),
                            name
                        )))
                    } else {
                        None
                    }
                }
                ActionType::Dropping(i, dir) => {
                    let item = self.owner_mut(world).wield.remove(i);
                    let name = item.item_type.name();
                    world
                        .load_tile_mut(self.owner(world).pos + dir)
                        .items
                        .push(item);
                    Some(ActionResult::LogMessage(format!(
                        "{} drop the {}",
                        self.owner(world).name_for_actions(),
                        name
                    )))
                }
                ActionType::Digging(dir) => {
                    let pos = self.owner(world).pos + dir;
                    let items = world.load_tile_mut(pos).dig();
                    if !items.is_empty() {
                        let mut rng = rand::thread_rng();
                        let places: Vec<Direction> = DIR8
                            .iter()
                            .filter(|d| {
                                (pos + *d != self.owner(world).pos)
                                    && world.load_tile(pos + *d).terrain.is_passable()
                            })
                            .copied()
                            .collect();
                        for item in items {
                            let delta = places.choose(&mut rng).unwrap();
                            world.load_tile_mut(pos + delta).items.push(item);
                        }
                    }
                    Some(ActionResult::LogMessage(format!(
                        "{} dig a hole",
                        self.owner(world).name_for_actions()
                    )))
                }
                ActionType::Reading(dir) => {
                    let pos = self.owner(world).pos + dir;
                    Some(ActionResult::LogMessage(world.load_tile(pos).read()))
                }
                ActionType::Animate(dir) => {
                    let pos = self.owner(world).pos + dir;
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
