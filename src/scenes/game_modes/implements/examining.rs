use colors::Colors;
use geometry::direction::Direction;
use geometry::Vec2;
use input;
use scenes::game_modes::{GameModeImpl, SomeResults, UpdateResult};
use scenes::implements::Game;
use settings::game::GameSettings;
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

#[derive(Debug, Copy, Clone)]
pub struct Examining {
    selected: Option<Direction>,
}

impl Examining {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Examining {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Examining {
    fn cursors(&self, _game: &Game) -> Vec<(Vec2, Color)> {
        if let Some(selected) = self.selected {
            vec![(selected.into(), Colors::LIME)]
        } else {
            vec![]
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
                    UpdateResult::Examine(dir),
                    UpdateResult::Pop,
                ]
            })
        }
    }
}
