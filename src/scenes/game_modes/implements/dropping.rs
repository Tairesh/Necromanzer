use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

use colors::Colors;
use game::actions::implements::Drop;
use game::map::terrain::TerrainInteract;
use game::World;
use geometry::direction::{Direction, DIR9};
use geometry::point::Point;
use input;
use scenes::game_modes::GameModeImpl;
use scenes::implements::GameScene;
use scenes::transition::SomeTransitions;

pub struct Dropping {
    selected: Option<Direction>,
}

impl Dropping {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Dropping {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Dropping {
    fn cursors(&self, world: &World) -> Vec<(Point, Color)> {
        if let Some(selected) = self.selected {
            vec![(selected.into(), Colors::LIME)]
        } else {
            DIR9.iter()
                .copied()
                .filter(|d| {
                    let pos = world.player().pos + d;
                    world.map().get_tile(pos).terrain.is_passable()
                })
                .map(|d| (d.into(), Colors::WHITE_SMOKE))
                .collect()
        }
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        if world.player().wield.is_empty() {
            Err("You have nothing to drop!".to_string())
        } else {
            Ok(())
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            game.try_start_action(Drop { item_id: 0, dir }.into());
            game.modes.pop();
        }
        None
    }
}
