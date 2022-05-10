#![allow(dead_code)]

use tetra::graphics::DrawParams;
use tetra::Context;

use assets::tileset::Tileset;
use colors::Colors;
use game::actions::Action;
use game::ai::ZombieBrain;
use game::bodies::helpers::human_body;
use game::bodies::{Body, Freshness, OrganData};
use game::human::character::Character;
use game::human::gender::Gender;
use game::map::item::{Item, ItemView};
use game::map::items::{BodyPartType, Cloak, Hat};
use game::map::pos::TilePos;
use geometry::two_dim_direction::TwoDimDirection;
use geometry::Vec2;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Soul {
    Player,
    Zombie(ZombieBrain),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub character: Character, // TODO: move this into Soul struct
    pub body: Body,
    pub pos: TilePos,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    pub wield: Vec<Item>, // TODO: custom struct with hands counter
    pub stamina: u8,
    pub soul: Soul,
    // TODO: traits
    // TODO: skills
}

impl Avatar {
    pub fn player(character: Character, pos: TilePos) -> Self {
        let mut body = human_body(&character, Freshness::Fresh);
        body.wear.push(Cloak::new().into());
        body.wear.push(Hat::new().into());
        Self::new(character, body, Soul::Player, pos)
    }

    pub fn zombie(character: Character, body: Body, pos: TilePos) -> Self {
        Self::new(character, body, Soul::Zombie(ZombieBrain::default()), pos)
    }

    pub fn new(character: Character, body: Body, soul: Soul, pos: TilePos) -> Self {
        Avatar {
            body,
            character,
            soul,
            pos,
            action: None,
            vision: TwoDimDirection::East,
            wield: Vec::new(),
            stamina: 100,
        }
    }

    pub fn name_for_actions(&self) -> String {
        match self.soul {
            Soul::Player => "You".to_string(),
            Soul::Zombie(..) => format!("Zombie {}", self.character.mind.name),
        }
    }

    // TODO: instead of draw, just return some sort of Glyph struct that doesnt reference Context
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
        if let Soul::Zombie(..) = self.soul {
            let freshness =
                self.body
                    .parts
                    .get(&TilePos::new(0, 0))
                    .map_or(Freshness::Rotten, |i| {
                        if let BodyPartType::Torso(OrganData { freshness, .. }, ..) = i.typ {
                            freshness
                        } else {
                            panic!("Root bodypart is not torso!")
                        }
                    });
            let (name, color) = match freshness {
                Freshness::Fresh => (
                    if self.character.appearance.age > 15 {
                        "raw_zombie"
                    } else {
                        "raw_zombie_child"
                    },
                    self.character.appearance.skin_tone.into(),
                ),
                Freshness::Rotten => (
                    if self.character.appearance.age > 15 {
                        "zombie"
                    } else {
                        "zombie_child"
                    },
                    Colors::WHITE,
                ),
                Freshness::Skeletal => (
                    if self.character.appearance.age > 15 {
                        "skeleton"
                    } else {
                        "skeleton_child"
                    },
                    Colors::WARM_IVORY,
                ),
            };
            tileset.draw_region(
                ctx,
                name,
                DrawParams::new()
                    .position(position)
                    .scale(scale)
                    .color(color),
            );
        } else {
            // TODO: draw wear
            tileset.draw_region(
                ctx,
                match self.character.mind.gender {
                    Gender::Female => "female",
                    Gender::Male => "male",
                    Gender::Custom(_) => "queer",
                },
                DrawParams::new()
                    .position(position)
                    .scale(scale)
                    .color(self.character.appearance.skin_tone.into()),
            );
        }
        if let Some(item) = self.wield.get(0) {
            let offset = if !rotate || matches!(self.vision, TwoDimDirection::East) {
                Vec2::new(15.0 * zoom, 10.0 * zoom)
            } else {
                Vec2::new(-15.0 * zoom, 10.0 * zoom)
            };
            tileset.draw_region(
                ctx,
                item.looks_like(),
                DrawParams::new()
                    .position(position + offset)
                    .scale(scale * -1.0),
            );
        }
    }
}
