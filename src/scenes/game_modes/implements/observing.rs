use colors::Colors;
use game::World;
use geometry::point::Point;
use geometry::Vec2;
use input;
use scenes::game_modes::update_result::UpdateResult;
use scenes::game_modes::{GameModeImpl, SomeResults};
use scenes::implements::Game;
use std::time::Instant;
use tetra::graphics::Color;
use tetra::input::{Key, KeyModifier};
use tetra::Context;

#[derive(Debug, Copy, Clone)]
pub struct Observing {
    pub pos: Point,
    last_shift: Instant,
    last_mouse_position: Vec2,
    mouse_moved: bool,
    last_zoom: f32,
    mouse_moved_pos: Point,
}

impl Observing {
    pub fn new() -> Self {
        Self {
            pos: Point::zero(),
            last_shift: Instant::now(),
            last_mouse_position: Vec2::zero(),
            mouse_moved: false,
            last_zoom: 0.0,
            mouse_moved_pos: Point::zero(),
        }
    }

    fn update_mouse(&mut self, ctx: &mut Context, game: &mut Game) {
        let mouse = input::get_mouse_position(ctx);
        let zoom = game.world.game_view.zoom.as_view();
        if mouse != self.last_mouse_position || zoom != self.last_zoom {
            self.last_mouse_position = mouse;
            self.last_zoom = zoom;
            if self.mouse_moved {
                let (w, h) = tetra::window::get_size(ctx);
                self.mouse_moved_pos = ((mouse - Vec2::new((w / 2) as f32, (h / 2) as f32))
                    / (game.tileset.tile_size as f32 * zoom))
                    .into();
            }
            self.mouse_moved = true;
        }
    }
}

impl Default for Observing {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Observing {
    fn cursors(&self, _world: &World) -> Vec<(Point, Color)> {
        vec![(self.mouse_moved_pos, Colors::LIME)]
    }
    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> SomeResults {
        self.update_mouse(ctx, game);
        if input::is_key_pressed(ctx, Key::Escape) {
            game.shift_of_view = Point::zero();
            return UpdateResult::Pop.into();
        } else if input::is_mouse_scrolled_down(ctx) {
            game.world.game_view.zoom.dec();
        } else if input::is_mouse_scrolled_up(ctx) {
            game.world.game_view.zoom.inc();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let now = Instant::now();
            if now.duration_since(self.last_shift).as_millis()
                > game.settings.borrow().repeat_interval
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_shift = now;
                game.shift_of_view += dir;
            }
        }

        None
    }
}
