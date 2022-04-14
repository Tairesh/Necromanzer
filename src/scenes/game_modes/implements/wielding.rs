use colors::Colors;
use game::actions::ActionType;
use geometry::direction::{Direction, DIR9};
use geometry::Vec2;
use input;
use scenes::game_modes::update_result::UpdateResult;
use scenes::game_modes::{GameModeImpl, SomeResults};
use scenes::implements::Game;
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

#[derive(Debug, Copy, Clone)]
pub struct Wielding {
    selected: Option<Direction>,
}

impl Wielding {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Wielding {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Wielding {
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
                        .map(|t| !t.items.is_empty())
                        .unwrap_or(false)
                })
                .map(|d| (Vec2::from(d), Colors::WHITE_SMOKE))
                .collect()
        }
    }

    fn can_push(&self, game: &Game) -> Result<(), String> {
        if game.world.player().wield.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "You already have {} in hands",
                game.world.player().wield.last().unwrap().name()
            ))
        }
    }

    fn update(&mut self, ctx: &mut Context) -> SomeResults {
        if input::is_key_pressed(ctx, Key::Escape) {
            UpdateResult::Pop.into()
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            UpdateResult::TryRotate(dir).into()
        } else {
            self.selected.map(|dir| {
                vec![
                    UpdateResult::TryRotate(dir),
                    UpdateResult::TryStartAction(ActionType::Wielding(dir)),
                    UpdateResult::Pop,
                ]
            })
        }
    }
}
