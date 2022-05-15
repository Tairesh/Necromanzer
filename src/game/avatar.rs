#![allow(dead_code)]

use tetra::graphics::DrawParams;
use tetra::Context;

use assets::tileset::Tileset;
use colors::Colors;
use game::actions::Action;
use game::ai::ZombieAI;
use game::bodies::{Body, Freshness, OrganData};
use game::human::gender::Gender;
use game::human::helpers::human_body;
use game::human::personality::Personality;
use game::map::item::{Item, ItemView};
use game::map::items::{BodyPartType, Cloak, Hat};
use game::map::pos::TilePos;
use geometry::two_dim_direction::TwoDimDirection;
use geometry::Vec2;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Soul {
    Player(Personality),
    Zombie(Personality, ZombieAI),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub body: Body,
    pub pos: TilePos,
    pub action: Option<Action>,
    pub vision: TwoDimDirection, // TODO: rotation of multitile body
    pub wield: Vec<Item>,        // TODO: custom struct with hands counter
    pub stamina: u8,
    pub soul: Soul,
    // TODO: traits
    // TODO: skills
}

impl Avatar {
    pub fn player(personality: Personality, pos: TilePos) -> Self {
        let mut body = human_body(&personality, Freshness::Fresh);
        body.wear.push(Cloak::new().into());
        body.wear.push(Hat::new().into());
        Self::new(body, Soul::Player(personality), pos)
    }

    pub fn zombie(personality: Personality, body: Body, pos: TilePos) -> Self {
        Self::new(body, Soul::Zombie(personality, ZombieAI::default()), pos)
    }

    pub fn new(body: Body, soul: Soul, pos: TilePos) -> Self {
        Avatar {
            body,
            soul,
            pos,
            action: None,
            vision: TwoDimDirection::East,
            wield: Vec::new(),
            stamina: 100,
        }
    }

    pub fn person(&self) -> &Personality {
        match &self.soul {
            Soul::Player(p) | Soul::Zombie(p, ..) => p,
        }
    }

    pub fn name_for_actions(&self) -> String {
        match &self.soul {
            Soul::Player(..) => "You".to_string(),
            Soul::Zombie(person, ..) => format!("Zombie {}", person.mind.name),
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
        match &self.soul {
            Soul::Zombie(person, ..) => {
                let freshness =
                    self.body
                        .parts
                        .get(&TilePos::new(0, 0))
                        .map_or(Freshness::Rotten, |i| {
                            if let BodyPartType::HumanTorso(OrganData { freshness, .. }, ..) = i.typ
                            {
                                freshness
                            } else {
                                panic!("Root bodypart is not torso!")
                            }
                        });
                let (name, color) = match freshness {
                    Freshness::Fresh => (
                        if person.appearance.age > 15 {
                            "raw_zombie"
                        } else {
                            "raw_zombie_child"
                        },
                        person.appearance.skin_tone.into(),
                    ),
                    Freshness::Rotten => (
                        if person.appearance.age > 15 {
                            "zombie"
                        } else {
                            "zombie_child"
                        },
                        Colors::WHITE,
                    ),
                    Freshness::Skeletal => (
                        if person.appearance.age > 15 {
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
            }
            Soul::Player(person) => {
                // TODO: draw wear
                tileset.draw_region(
                    ctx,
                    match person.mind.gender {
                        Gender::Female => "female",
                        Gender::Male => "male",
                        Gender::Custom(_) => "queer",
                    },
                    DrawParams::new()
                        .position(position)
                        .scale(scale)
                        .color(person.appearance.skin_tone.into()),
                );
            }
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
