use colors::Colors;
use game::actions::action_impl::ActionImpl;
use game::actions::ActionPossibility::{self, No, Yes};
use game::actions::{Action, ActionResult};
use game::avatar::Soul;
use game::map::passage::Passage::Passable;
use game::map::terrain::{TerrainInteract, TerrainView};
use game::map::tile::Tile;
use game::{Avatar, World};
use geometry::direction::Direction;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Walk {
    pub dir: Direction,
}

impl Walk {
    fn length(actor: &Avatar, tile: &Tile) -> u32 {
        // TODO: check avatar perks for calculating speed
        // TODO: add sqrt(2) for diagonal movement
        let koeff = match actor.soul {
            Soul::Zombie(..) => 1.5,
            Soul::Player => 1.0,
        } * match actor.character.appearance.age {
            0 => 100.0,
            1..=3 => 10.0,
            4..=10 => 3.0,
            11.. => 1.0,
        };
        if let Passable(length) = tile.terrain.passage() {
            (length * koeff).round() as u32
        } else {
            0
        }
    }
}

impl ActionImpl for Walk {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);
        if !tile.terrain.is_passable() {
            return No(format!("You can't walk to the {}", tile.terrain.name()));
        }
        let unit_on_tile = tile.units.iter().next();
        if let Some(unit_id) = unit_on_tile {
            if let Some(unit) = world.units.get(*unit_id) {
                return No(format!("{} is on the way", unit.character.mind.name));
            }
        }

        Yes(Self::length(actor, tile))
    }

    fn on_finish(&self, action: &Action, world: &mut World) -> Option<ActionResult> {
        world.move_avatar(action.owner, self.dir);
        if action.length > 15 {
            Some(ActionResult::ColoredLogMessage(
                format!(
                    "It takes a long time to walk through the {}",
                    world
                        .map()
                        .get_tile(world.get_unit(action.owner).pos)
                        .terrain
                        .name()
                ),
                Colors::GRAY,
            ))
        } else {
            None
        }
    }
}
