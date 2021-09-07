use colors::Colors;
use direction::Direction;
use geometry::DIR9;
use itertools::Itertools;
use scenes::game::{GameModeTrait, UpdateResult};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::input::Key;
use tetra::Context;
use world::World;
use {input, Vec2};

pub(crate) struct Examining {
    cursor: Mesh,
    selected: Option<Direction>,
}

impl Examining {
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

impl GameModeTrait for Examining {
    fn update(&mut self, ctx: &mut Context, world: &mut World) -> Vec<UpdateResult> {
        if input::is_key_pressed(ctx, Key::Escape) {
            vec![UpdateResult::ResetGameMode]
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            if self.selected.is_none() {
                self.selected = Some(dir);
                if let Some(dir) = dir.as_two_dimensional() {
                    world.avatar.vision = dir;
                }
                let pos = world.avatar.pos.add(dir);
                let tile = world.load_tile(pos);
                let mut this_is = tile.terrain.this_is();
                if !tile.items.is_empty() {
                    // TODO: use the std version when stable (see https://github.com/rust-lang/rust/issues/79524)
                    let items: String =
                        Itertools::intersperse(tile.items.iter().map(|item| item.name()), ", ")
                            .collect();
                    this_is += " Here you see: ";
                    this_is += items.as_str();
                }
                vec![UpdateResult::AddLogMessage(this_is)]
            } else {
                vec![]
            }
        } else if self.selected.is_some() {
            vec![UpdateResult::ResetGameMode]
        } else {
            vec![]
        }
    }

    fn draw(&mut self, ctx: &mut Context, world: &mut World, center: Vec2, zoom: f32) {
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
        } else {
            for (dx, dy) in DIR9 {
                let pos = world.avatar.pos.add_delta(dx, dy);
                let tile = world.load_tile(pos);
                if !tile.items.is_empty() {
                    self.cursor.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(
                                center.x + dx as f32 * 10.0 * zoom,
                                center.y + dy as f32 * 10.0 * zoom,
                            ))
                            .scale(Vec2::new(zoom, zoom))
                            .color(Colors::LIGHT_YELLOW.with_alpha(0.75)),
                    );
                }
            }
        }
    }
}
