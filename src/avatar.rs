use action::Action;
use direction::TwoDimDirection;
use human::character::Character;
use human::gender::Gender;
use map::item::Item;
use map::pos::TilePos;
use tetra::graphics::{DrawParams, Rectangle, Texture};
use tetra::Context;
use Vec2;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub character: Character,
    pub pos: TilePos,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    pub wield: Vec<Item>, // TODO: custom struct with hands counter
}

impl Avatar {
    pub fn new(character: Character, pos: TilePos) -> Self {
        Avatar {
            character,
            pos,
            action: None,
            vision: TwoDimDirection::East,
            wield: Vec::new(),
        }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        tileset: &Texture,
        mut position: Vec2,
        zoom: f32,
        rotate: bool,
    ) {
        let scale = if !rotate || matches!(self.vision, TwoDimDirection::East) {
            Vec2::new(zoom, zoom)
        } else {
            position.x += 10.0 * zoom;
            Vec2::new(-zoom, zoom)
        };
        tileset.draw_region(
            ctx,
            match self.character.gender {
                Gender::Female => Rectangle::new(0.0, 0.0, 10.0, 10.0),
                Gender::Male => Rectangle::new(10.0, 0.0, 10.0, 10.0),
                Gender::Custom(_) => Rectangle::new(20.0, 0.0, 10.0, 10.0),
            },
            DrawParams::new()
                .position(position)
                .scale(scale)
                .color(self.character.skin_tone.color()),
        );
        if let Some(item) = self.wield.get(0) {
            let offset = if !rotate || matches!(self.vision, TwoDimDirection::East) {
                Vec2::new(15.0 * zoom, 10.0 * zoom)
            } else {
                Vec2::new(-15.0 * zoom, 10.0 * zoom)
            };
            tileset.draw_region(
                ctx,
                item.region(),
                DrawParams::new()
                    .position(position + offset)
                    .scale(scale * -1.0),
            );
        }
    }
}
