use colors::Colors;
use geometry::point::Point;
use geometry::Vec2;
use input;
use scenes::game_modes::update_result::UpdateResult;
use scenes::game_modes::{GameModeImpl, SomeResults};
use scenes::implements::Game;
use settings::game::GameSettings;
use std::time::Instant;
use tetra::graphics::Color;
use tetra::input::{Key, KeyModifier};
use tetra::Context;

#[derive(Debug, Copy, Clone)]
pub struct Observing {
    pub pos: Point,
    last_shift: Instant,
}

impl Observing {
    pub fn new() -> Self {
        Self {
            pos: Point::zero(),
            last_shift: Instant::now(),
        }
    }
}

impl Default for Observing {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Observing {
    fn cursors(&self, _game: &Game) -> Vec<(Vec2, Color)> {
        vec![(Vec2::zero(), Colors::LIME)]
    }
    fn update(&mut self, ctx: &mut Context, settings: &GameSettings) -> SomeResults {
        if input::is_key_pressed(ctx, Key::Escape) {
            Some(vec![UpdateResult::SetViewFollow, UpdateResult::Pop])
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let now = Instant::now();
            if now.duration_since(self.last_shift).as_millis() > settings.repeat_interval
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_shift = now;
                self.pos += dir;
                UpdateResult::SetViewShift(self.pos).into()
            } else {
                None
            }
        } else {
            None
        }
    }
}
