use rand::seq::SliceRandom;

use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, No, Yes};
use game::actions::{Action, ActionResult};
use game::map::item::{ItemInteract, ItemTag};
use game::map::terrain::{Terrain, TerrainInteract, TerrainView};
use game::{Avatar, World};
use geometry::direction::{Direction, DIR8};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Dig {
    pub dir: Direction,
}

impl ActionImpl for Dig {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            if !tile.terrain.is_diggable() {
                return No(format!("You can't dig the {}", tile.terrain.name()));
            }
        }
        if !actor.wield.iter().any(|i| i.tags().contains(&ItemTag::Dig)) {
            return No("You need a shovel to dig!".to_string());
        }

        if let Some(tile) = world.get_tile(pos) {
            Yes(match tile.terrain {
                Terrain::Grave(..) => 2000,
                _ => 1000,
            })
        } else {
            No("TIle isn't loaded yet".to_string())
        }
    }

    fn on_start(&self, action: &Action, world: &mut World) -> Option<ActionResult> {
        Some(ActionResult::LogMessage(format!(
            "{} start digging",
            action.owner(world).name_for_actions()
        )))
    }

    fn on_finish(&self, action: &Action, world: &mut World) -> Option<ActionResult> {
        let pos = action.owner(world).pos + self.dir;
        let items = world.load_tile_mut(pos).dig();
        if !items.is_empty() {
            let mut rng = rand::thread_rng();
            let places: Vec<Direction> = DIR8
                .iter()
                .filter(|d| {
                    (pos + *d != action.owner(world).pos)
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
            action.owner(world).name_for_actions()
        )))
    }
}
