use colors::Colors;
use game::actions::ActionType;
use geometry::direction::{Direction, DIR9};
use geometry::Vec2;
use input;
use scenes::game_modes::update_result::UpdateResult;
use scenes::game_modes::{GameModeImpl, SomeResults};
use scenes::implements::Game;
use settings::game::GameSettings;
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

#[derive(Debug, Copy, Clone)]
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
    fn cursors(&self, game: &Game) -> Vec<(Vec2, Color)> {
        if let Some(selected) = self.selected {
            vec![(selected.into(), Colors::LIME)]
        } else {
            DIR9.iter()
                .copied()
                .filter(|d| {
                    let pos = game.world.player().pos + d;
                    game.world
                        .get_tile(pos)
                        .map(|t| t.terrain.is_walkable())
                        .unwrap_or(false)
                })
                .map(|d| (d.into(), Colors::WHITE_SMOKE))
                .collect()
        }
    }

    fn can_push(&self, game: &Game) -> Result<(), String> {
        if game.world.player().wield.is_empty() {
            Err("You have nothing to drop!".to_string())
        } else {
            Ok(())
        }
    }

    fn update(&mut self, ctx: &mut Context, _settings: &GameSettings) -> SomeResults {
        if input::is_key_pressed(ctx, Key::Escape) {
            UpdateResult::Pop.into()
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            UpdateResult::TryRotate(dir).into()
        } else {
            self.selected.map(|dir| {
                vec![
                    UpdateResult::TryRotate(dir),
                    UpdateResult::TryStartAction(ActionType::Dropping(0, dir)),
                    UpdateResult::Pop,
                ]
            })
        }
    }
}
