#![allow(dead_code)]

use assets::tileset::Tileset;
use colors::Colors;
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
pub enum Brain {
    Player,
    Zombie,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub character: Character,
    pub body: Body,
    pub pos: TilePos,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    pub wield: Vec<Item>, // TODO: custom struct with hands counter
    pub stamina: u8,
    pub brain: Brain,
    // TODO: traits
    // TODO: skills
}

impl Avatar {
    pub fn player(character: Character, pos: TilePos) -> Self {
        let mut body = Body::human(&character, Freshness::Fresh);
        body.wear.push(Item::new(ItemType::Cloak));
        body.wear.push(Item::new(ItemType::MagicHat));
        Self::new(character, body, Brain::Player, pos)
    }

    pub fn zombie(character: Character, body: Body, pos: TilePos) -> Self {
        Self::new(character, body, Brain::Zombie, pos)
    }

    pub fn new(character: Character, body: Body, brain: Brain, pos: TilePos) -> Self {
        Avatar {
            body,
            character,
            brain,
            pos,
            action: None,
            vision: TwoDimDirection::East,
            wield: Vec::new(),
            stamina: 100,
        }
    }

    pub fn name_for_actions(&self) -> String {
        match self.brain {
            Brain::Player => "You".to_string(),
            Brain::Zombie => format!("Zombie {}", self.character.name),
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
        // TODO: create canvas
        let scale = if !rotate || matches!(self.vision, TwoDimDirection::East) {
            Vec2::new(zoom, zoom)
        } else {
            position.x += 10.0 * zoom;
            Vec2::new(-zoom, zoom)
        };
        if let Brain::Zombie = self.brain {
            let freshness = self
                .body
                .parts
                .get("torso")
                .map(|i| {
                    i.item_type
                        .body_part()
                        .map(|bp| bp.freshness)
                        .unwrap_or(Freshness::Rotten)
                })
                .unwrap_or(Freshness::Rotten);
            let (region, color) = match freshness {
                Freshness::Fresh => (
                    if self.character.age > 15 {
                        tileset.raw_zombie
                    } else {
                        tileset.raw_zombie_child
                    },
                    self.character.skin_tone.into(),
                ),
                Freshness::Rotten => (
                    if self.character.age > 15 {
                        tileset.zombie
                    } else {
                        tileset.zombie_child
                    },
                    Colors::WHITE,
                ),
                Freshness::Skeletal => (
                    if self.character.age > 15 {
                        tileset.skeleton
                    } else {
                        tileset.skeleton_child
                    },
                    Colors::WARM_IVORY,
                ),
            };
            tileset.texture.draw_region(
                ctx,
                region,
                DrawParams::new()
                    .position(position)
                    .scale(scale)
                    .color(color),
            );
        } else {
            // TODO: draw wear
            tileset.texture.draw_region(
                ctx,
                match self.character.gender {
                    Gender::Female => tileset.female,
                    Gender::Male => tileset.male,
                    Gender::Custom(_) => tileset.queer,
                },
                DrawParams::new()
                    .position(position)
                    .scale(scale)
                    .color(self.character.skin_tone.into()),
            );
        }
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
