use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

use colors::Colors;
use game::actions::implements::Dig;
use game::map::item::{ItemInteract, ItemTag};
use game::map::terrain::TerrainInteract;
use game::World;
use geometry::direction::{Direction, DIR9};
use geometry::point::Point;
use input;
use scenes::game_modes::GameModeImpl;
use scenes::implements::Game;
use scenes::transition::SomeTransitions;

pub struct Digging {
    selected: Option<Direction>,
}

impl Digging {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Digging {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Digging {
    fn cursors(&self, world: &World) -> Vec<(Point, Color)> {
        if let Some(selected) = self.selected {
            vec![(selected.into(), Colors::LIME)]
        } else {
            DIR9.iter()
                .copied()
                .filter(|d| {
                    let pos = world.player().pos + d;
                    world
                        .get_tile(pos)
                        .map(|t| t.terrain.is_diggable())
                        .unwrap_or(false)
                })
                .map(|d| (d.into(), Colors::LIGHT_YELLOW))
                .collect()
        }
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        if world
            .player()
            .wield
            .iter()
            .any(|i| i.tags().contains(&ItemTag::Dig))
        {
            Ok(())
        } else {
            Err("You can't dig without a shovel".to_string())
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            game.try_start_action(Dig { dir }.into());
            game.modes.pop();
        }
        None
    }
}
