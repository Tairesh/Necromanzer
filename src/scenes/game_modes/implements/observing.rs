use colors::Colors;
use geometry::Vec2;
use input;
use scenes::game_modes::update_result::UpdateResult;
use scenes::game_modes::{GameModeImpl, SomeResults};
use scenes::implements::Game;
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

#[derive(Debug, Copy, Clone)]
pub struct Observing {
    pub cursor: Vec2,
}

impl GameModeImpl for Observing {
    fn cursors(&self, _game: &Game) -> Vec<(Vec2, Color)> {
        vec![(self.cursor, Colors::LIME)]
    }
    fn update(&mut self, ctx: &mut Context) -> SomeResults {
        if input::is_key_pressed(ctx, Key::Escape) {
            UpdateResult::Pop.into()
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.cursor += dir;
            dbg!(self.cursor);
            None
        } else {
            None
        }
    }
}
