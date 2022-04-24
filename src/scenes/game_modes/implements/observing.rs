use colors::Colors;
use game::World;
use geometry::direction::Direction;
use geometry::point::Point;
use geometry::{Rect, Vec2};
use input;
use scenes::game_modes::update_result::UpdateResult;
use scenes::game_modes::{GameModeImpl, SomeResults};
use scenes::implements::Game;
use sprites::label::Label;
use sprites::meshy::JustMesh;
use sprites::position::Position;
use sprites::sprite::{Draw, Positionate, Stringify};
use std::time::Instant;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{Color, Rectangle};
use tetra::input::{Key, KeyModifier};
use tetra::Context;

struct ObservingSprite {
    pub label: Label,
    pub mesh: JustMesh,
}

fn create_mesh(ctx: &mut Context, rect: Rect, position: Position) -> JustMesh {
    JustMesh::new(
        Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0.0, 0.0, rect.w, rect.h),
        )
        .unwrap(),
        Some(Colors::BLACK.with_alpha(0.7)),
        Vec2::new(rect.w, rect.h),
        position,
    )
}

pub struct Observing {
    last_shift: Instant,
    last_mouse_position: Vec2,
    mouse_moved: bool,
    last_zoom: f32,
    mouse_moved_pos: Point,
    sprite: Option<Box<ObservingSprite>>,
}

impl Observing {
    pub fn new() -> Self {
        Self {
            last_shift: Instant::now(),
            last_mouse_position: Vec2::zero(),
            mouse_moved: false,
            last_zoom: 0.0,
            mouse_moved_pos: Point::zero(),
            sprite: None,
        }
    }

    fn update_mouse(&mut self, ctx: &mut Context, game: &mut Game) {
        let mouse = input::get_mouse_position(ctx);
        let zoom = game.world.game_view.zoom.as_view();
        if mouse != self.last_mouse_position || zoom != self.last_zoom {
            self.last_mouse_position = mouse;
            self.last_zoom = zoom;
            if self.mouse_moved {
                let (w, h) = game.window_size;
                self.mouse_moved_pos = ((mouse - Vec2::new((w / 2) as f32, (h / 2) as f32))
                    / (game.assets.tileset.tile_size as f32 * zoom))
                    .into();
            }
            self.mouse_moved = true;
        }
    }

    fn update_sprite(&mut self, ctx: &mut Context, game: &mut Game) {
        let pos = game.world.player().pos + game.shift_of_view + self.mouse_moved_pos;
        let msg = game.world.this_is(pos, true);
        let tile_size = game.tile_size();
        let position = Vec2::from(self.mouse_moved_pos * tile_size);
        let position_shift = tile_size / 2.0 + 5.0;
        let position = match Direction::from_delta(self.mouse_moved_pos.x, self.mouse_moved_pos.y) {
            Direction::NorthWest | Direction::North | Direction::West | Direction::Here => {
                Position::at_center_by_left_top(
                    position.x + position_shift,
                    position.y + position_shift,
                )
            }
            Direction::East | Direction::NorthEast => Position::at_center_by_right_top(
                position.x - position_shift,
                position.y + position_shift,
            ),
            Direction::South | Direction::SouthWest => Position::at_center_by_left_bottom(
                position.x + position_shift,
                position.y - position_shift,
            ),
            Direction::SouthEast => Position::at_center_by_right_bottom(
                position.x - position_shift,
                position.y - position_shift,
            ),
        };
        let window_size = game.window_size;
        if let Some(sprite) = &mut self.sprite {
            sprite.label.set_value(msg);
            sprite.label.set_position(position);
            sprite.label.positionate(ctx, window_size);
            let rect = sprite.label.rect();
            sprite.mesh = create_mesh(ctx, rect, position);
            sprite.mesh.positionate(ctx, window_size);
        } else {
            let mut label = Label::new(
                msg,
                game.assets.fonts.default.clone(),
                Colors::WHITE_SMOKE,
                position,
            );
            label.positionate(ctx, window_size);
            let rect = label.rect();
            let mut mesh = create_mesh(ctx, rect, position);
            mesh.positionate(ctx, window_size);
            self.sprite = Some(Box::new(ObservingSprite { label, mesh }));
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
        let mut shifted = false;
        if input::is_key_pressed(ctx, Key::Escape) {
            game.shift_of_view = Point::zero();
            return UpdateResult::Pop.into();
        } else if input::is_mouse_scrolled_down(ctx) {
            game.world.game_view.zoom.dec();
            shifted = true;
        } else if input::is_mouse_scrolled_up(ctx) {
            game.world.game_view.zoom.inc();
            shifted = true;
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let now = Instant::now();
            if now.duration_since(self.last_shift).as_millis()
                > game.settings.borrow().repeat_interval
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_shift = now;
                game.shift_of_view += dir;
                shifted = true;
            }
        }
        if self.mouse_moved || shifted {
            self.update_sprite(ctx, game);
        }

        None
    }
    fn draw(&mut self, ctx: &mut Context, _game: &mut Game) {
        if let Some(sprite) = &mut self.sprite {
            sprite.mesh.draw(ctx);
            sprite.label.draw(ctx);
        }
    }
}
