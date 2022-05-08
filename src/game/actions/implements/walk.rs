use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, No, Yes};
use game::avatar::Soul;
use game::map::passage::Passage::Passable;
use game::map::terrain::{TerrainInteract, TerrainView};
use game::{Avatar, World};
use geometry::direction::Direction;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Walk {
    pub dir: Direction,
}

impl ActionImpl for Walk {
    fn length(&self, actor: &Avatar, world: &World) -> u32 {
        // TODO: check avatar perks for calculating speed
        // TODO: add sqrt(2) for diagonal movement
        let koeff = match actor.soul {
            Soul::Zombie(..) => 1.5,
            _ => 1.0,
        };
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            if let Passable(length) = tile.terrain.passage() {
                return (length * koeff).round() as u32;
            }
        }

        0
    }

    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        if let Some(tile) = world.get_tile(pos) {
            if !tile.terrain.is_passable() {
                return No(format!("You can't walk to the {}", tile.terrain.name()));
            }
            let unit_on_tile = tile.units.iter().next();
            if let Some(unit_id) = unit_on_tile {
                if let Some(unit) = world.units.get(*unit_id) {
                    return No(format!("{} is on the way", unit.character.name));
                }
            }

            Yes
        } else {
            No("Tile isn't loaded yet".to_string())
        }
    }
}
