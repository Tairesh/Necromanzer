use action::Action;
use assets::Tileset;
use direction::TwoDimDirection;
use human::body::{Body, Freshness};
use human::character::Character;
use human::gender::Gender;
use map::item::{Item, ItemType};
use map::pos::TilePos;
use tetra::graphics::{DrawParams, Rectangle};
use tetra::Context;
use Vec2;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub character: Character,
    pub body: Body,
    pub pos: TilePos,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    pub wield: Vec<Item>, // TODO: custom struct with hands counter
}

impl Avatar {
    pub fn new(character: Character, pos: TilePos) -> Self {
        Avatar {
            body: Body::human(&character, Freshness::Fresh),
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
        tileset: &Tileset,
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
        let torso = self.body.parts.get("torso").unwrap();
        let (gender, skin_tone) = if let ItemType::HumanTorso(part) = &torso.item_type {
            (part.gender.clone(), part.skin_tone)
        } else {
            (self.character.gender.clone(), self.character.skin_tone)
        };
        tileset.texture.draw_region(
            ctx,
            match gender {
                Gender::Female => Rectangle::new(0.0, 0.0, 10.0, 10.0),
                Gender::Male => Rectangle::new(10.0, 0.0, 10.0, 10.0),
                Gender::Custom(_) => Rectangle::new(20.0, 0.0, 10.0, 10.0),
            },
            DrawParams::new()
                .position(position)
                .scale(scale)
                .color(skin_tone.into()),
        );
        if let Some(item) = self.wield.get(0) {
            let offset = if !rotate || matches!(self.vision, TwoDimDirection::East) {
                Vec2::new(15.0 * zoom, 10.0 * zoom)
            } else {
                Vec2::new(-15.0 * zoom, 10.0 * zoom)
            };
            tileset.texture.draw_region(
                ctx,
                item.item_type.region(tileset),
                DrawParams::new()
                    .position(position + offset)
                    .scale(scale * -1.0),
            );
        }
    }
}
