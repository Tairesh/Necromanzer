use colors::Colors;
use direction::Direction;
use map::Passage;
use scenes::game::{GameModeTrait, UpdateResult};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::input::Key;
use tetra::Context;
use world::World;
use {input, Vec2};

pub(crate) struct Dropping {
    cursor: Mesh,
    selected: Option<Direction>,
}

impl Dropping {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            cursor: Mesh::rectangle(
                ctx,
                ShapeStyle::Stroke(1.0),
                Rectangle::new(0.0, 0.0, 10.0, 10.0),
            )
            .unwrap(),
            selected: None,
        }
    }
}

impl GameModeTrait for Dropping {
    fn update(&mut self, ctx: &mut Context, world: &mut World) -> UpdateResult {
        if input::is_key_pressed(ctx, Key::Escape) {
            UpdateResult::ResetGameMode
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let tile = world.load_tile(world.avatar.pos.add(dir));
            if let Passage::Passable(_) = tile.terrain.pass() {
                if self.selected.is_none() {
                    self.selected = Some(dir);
                    UpdateResult::Drop(dir)
                } else {
                    UpdateResult::DoNothing
                }
            } else {
                UpdateResult::DoNothing
            }
        } else if self.selected.is_some() {
            UpdateResult::ResetGameMode
        } else {
            UpdateResult::DoNothing
        }
    }

    fn draw(&mut self, ctx: &mut Context, _world: &mut World, center: Vec2, zoom: f32) {
        if let Some(dir) = self.selected {
            self.cursor.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        center.x + dir.dx() as f32 * 10.0 * zoom,
                        center.y + dir.dy() as f32 * 10.0 * zoom,
                    ))
                    .scale(Vec2::new(zoom, zoom))
                    .color(Colors::LIGHT_YELLOW.with_alpha(0.75)),
            )
        }
    }
}
