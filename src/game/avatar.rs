#![allow(dead_code)]

use assets::tileset::Tileset;
use game::actions::Action;
use geometry::direction::TwoDimDirection;
use geometry::Vec2;
use human::body::{Body, Freshness};
use human::character::Character;
use human::gender::Gender;
use map::item::{Item, ItemType};
use map::pos::TilePos;
use tetra::graphics::DrawParams;
use tetra::Context;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub character: Character,
    pub body: Body,
    pub pos: TilePos,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    pub wield: Vec<Item>, // TODO: custom struct with hands counter
    pub stamina: u8,
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
            stamina: 100,
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
                Gender::Female => tileset.female,
                Gender::Male => tileset.male,
                Gender::Custom(_) => tileset.queer,
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
