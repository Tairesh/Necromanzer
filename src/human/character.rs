#![allow(dead_code)]

use assets::game_data::GameData;
use human::gender::Gender;
use human::main_hand::MainHand;
use human::skin_tone::SkinTone;
use rand::distributions::Standard;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub gender: Gender,
    pub age: u8,
    pub main_hand: MainHand,
    pub skin_tone: SkinTone,
    // TODO: profession
}

impl Character {
    pub fn new<S: Into<String>>(
        name: S,
        gender: Gender,
        age: u8,
        main_hand: MainHand,
        skin_tone: SkinTone,
    ) -> Self {
        Self {
            name: name.into(),
            gender,
            age,
            main_hand,
            skin_tone,
        }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R, game_data: &GameData) -> Character {
        let gender = rng.sample(Standard);
        let name = format!(
            "{} {}",
            match gender {
                Gender::Male => &game_data.names.male_names,
                Gender::Female => &game_data.names.female_names,
                Gender::Custom(_) => &game_data.names.names,
            }
            .choose(rng)
            .unwrap(),
            game_data.names.male_names.choose(rng).unwrap()
        );
        Character::new(
            name,
            gender,
            rng.gen_range(0..=99),
            rng.sample(Standard),
            rng.sample(Standard),
        )
    }

    pub fn gender_age_name(&self) -> &str {
        if self.age < 3 {
            "baby"
        } else if self.age < 16 {
            match self.gender {
                Gender::Male => "boy",
                Gender::Female => "girl",
                Gender::Custom(_) => "child",
            }
        } else {
            match self.gender {
                Gender::Male => "man",
                Gender::Female => "woman",
                Gender::Custom(_) => "human",
            }
        }
    }
}
