use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, No, Yes};
use game::map::item::{ItemInteract, ItemTag};
use game::map::terrain::{Terrain, TerrainInteract, TerrainView};
use game::{Avatar, World};
use geometry::direction::Direction;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Dig {
    pub dir: Direction,
}

impl ActionImpl for Dig {
    fn length(&self, actor: &Avatar, world: &World) -> u32 {
        // TODO: check tool quality, avatar perks
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            return match tile.terrain {
                Terrain::Grave(..) => 2000,
                _ => 1000,
            };
        }

        0
    }

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

        Yes
    }
}
