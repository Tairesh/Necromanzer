use colors::Colors;
use direction::Direction;
use input;
use scenes::game::{GameModeTrait, UpdateResult};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::input::Key;
use tetra::{Context, TetraVec2};
use world::World;

pub(crate) struct Examining {
    mesh: Mesh,
    selected: Option<Direction>,
}

impl Examining {
    pub fn new(ctx: &mut Context) -> Self {
        let mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Stroke(1.0),
            Rectangle::new(0.0, 0.0, 10.0, 10.0),
        )
        .unwrap();
        Self {
            mesh,
            selected: None,
        }
    }
}

impl GameModeTrait for Examining {
    fn update(&mut self, ctx: &mut Context) -> UpdateResult {
        if input::is_key_pressed(ctx, Key::Escape) {
            UpdateResult::ResetGameMode
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            if self.selected.is_none() {
                self.selected = Some(dir);
                UpdateResult::Examine(dir)
            } else {
                UpdateResult::DoNothing
            }
        } else if self.selected.is_some() {
            UpdateResult::ResetGameMode
        } else {
            UpdateResult::DoNothing
        }
    }

    fn draw(&mut self, ctx: &mut Context, _world: &mut World, center: TetraVec2, zoom: f32) {
        if let Some(dir) = self.selected {
            self.mesh.draw(
                ctx,
                DrawParams::new()
                    .position(TetraVec2::new(
                        center.x + dir.dx() as f32 * 10.0 * zoom,
                        center.y + dir.dy() as f32 * 10.0 * zoom,
                    ))
                    .scale(TetraVec2::new(zoom, zoom))
                    .color(Colors::LIGHT_YELLOW.with_alpha(0.75)),
            )
        }
    }
}
