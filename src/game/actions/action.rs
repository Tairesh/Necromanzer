use super::{ActionImpl, ActionResult, ActionType};
use colors::Colors;
use game::actions::implements::{Dig, Drop, Raise, Read, Walk, Wield};
use game::actions::ActionPossibility;
use game::map::item::{Item, ItemView};
use game::map::terrain::TerrainInteract;
use game::{Avatar, World};
use geometry::direction::{Direction, DIR8};
use rand::seq::SliceRandom;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Action {
    pub owner: usize,
    pub typ: ActionType,
    length: u32,
    pub finish: u128,
}

impl Action {
    pub fn new(owner: usize, typ: ActionType, world: &World) -> Result<Self, String> {
        match typ.is_possible(world.get_unit(owner), world) {
            ActionPossibility::Yes => {
                let length = typ.length(world.get_unit(owner), world);
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
        world.get_unit(self.owner)
    }

    fn owner_mut<'a>(&self, world: &'a mut World) -> &'a mut Avatar {
        world.get_unit_mut(self.owner)
    }

    /// called every tick
    pub fn act(&self, world: &mut World) -> Option<ActionResult> {
        if let ActionPossibility::No(reason) = self.typ.is_possible(self.owner(world), world) {
            return Some(ActionResult::CancelAction(reason));
        }

        let steps = (self.finish - world.meta.current_tick) as u32;
        if steps == self.length {
            match self.typ {
                ActionType::Dig(..) => Some(ActionResult::LogMessage(format!(
                    "{} start digging",
                    self.owner(world).name_for_actions()
                ))),
                _ => None,
            }
        } else if steps == 0 {
            // finish
            match self.typ {
                ActionType::Skip(..) => None,
                ActionType::Walk(Walk { dir }) => {
                    world.move_avatar(self.owner, dir);
                    None
                }
                ActionType::Wield(Wield { dir }) => {
                    if let Some(item) = world.load_tile_mut(self.owner(world).pos + dir).items.pop()
                    {
                        let name = item.name();
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
                ActionType::Drop(Drop { item_id, dir }) => {
                    let item = self.owner_mut(world).wield.remove(item_id);
                    let name = item.name();
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
                ActionType::Dig(Dig { dir }) => {
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
                    world.calc_fov();
                    Some(ActionResult::LogMessage(format!(
                        "{} dig a hole",
                        self.owner(world).name_for_actions()
                    )))
                }
                ActionType::Read(Read { dir }) => {
                    let pos = self.owner(world).pos + dir;
                    Some(ActionResult::LogMessage(world.load_tile(pos).read()))
                }
                ActionType::Raise(Raise { dir }) => {
                    let pos = self.owner(world).pos + dir;
                    if let Some(i) = world
                        .load_tile(pos)
                        .items
                        .iter()
                        .position(|i| matches!(i, Item::Corpse(..)))
                    {
                        let body = world.load_tile_mut(pos).items.remove(i);
                        if let Item::Corpse(corpse) = body {
                            let name = corpse.character.age_name().to_owned();
                            let zombie = Avatar::zombie(corpse.character, corpse.body, pos);
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
