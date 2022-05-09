#![allow(dead_code)]

use rand::distributions::Standard;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

use assets::game_data::GameData;
use game::bodies::BodySize;
use game::human::hair_color::HairColor;

use super::gender::Gender;
use super::main_hand::MainHand;
use super::skin_tone::SkinTone;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "g")]
    pub gender: Gender,
    #[serde(rename = "a")]
    pub age: u8,
    #[serde(rename = "m")]
    pub main_hand: MainHand,
    #[serde(rename = "s")]
    pub skin_tone: SkinTone,
    #[serde(rename = "h")]
    pub hair_color: HairColor,
    #[serde(rename = "z")]
    pub body_size: BodySize,
    #[serde(rename = "l")]
    pub alive: bool,
    // TODO: profession
    // TODO: probably move appearance and mind attributes to separate structs
}

impl Character {
    pub fn new<S: Into<String>>(
        name: S,
        gender: Gender,
        age: u8,
        main_hand: MainHand,
        skin_tone: SkinTone,
        hair_color: HairColor,
        body_size: BodySize,
        alive: bool,
    ) -> Self {
        Self {
            name: name.into(),
            gender,
            age,
            main_hand,
            skin_tone,
            hair_color,
            body_size,
            alive,
        }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R, game_data: &GameData, alive: bool) -> Character {
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
            rng.sample(Standard),
            rng.sample(Standard),
            alive,
        )
    }

    pub fn age_name(&self) -> &str {
        age_name(self.age, Some(&self.gender))
    }
}

pub fn age_name(age: u8, gender: Option<&Gender>) -> &'static str {
    match age {
        0..=3 => "baby",
        4..=15 => {
            if let Some(gender) = gender {
                match gender {
                    Gender::Male => "boy",
                    Gender::Female => "girl",
                    Gender::Custom(_) => "child",
                }
            } else {
                "child"
            }
        }
        16.. => {
            if let Some(gender) = gender {
                match gender {
                    Gender::Male => "man",
                    Gender::Female => "woman",
                    Gender::Custom(_) => "human",
                }
            } else {
                "human"
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use game::bodies::BodySize;
    use game::human::gender::Gender;
    use game::human::hair_color::HairColor;
    use game::human::main_hand::MainHand;
    use game::human::skin_tone::SkinTone;

    use super::Character;

    pub fn dead_boy() -> Character {
        Character::new(
            "Dead Boy",
            Gender::Male,
            9,
            MainHand::Right,
            SkinTone::Almond,
            HairColor::Black,
            BodySize::Tiny,
            false,
        )
    }

    pub fn tester_girl() -> Character {
        Character::new(
            "Tester Girl",
            Gender::Female,
            15, // Fifteen is the best age
            MainHand::Left,
            SkinTone::WarmIvory,
            HairColor::Ginger,
            BodySize::Small,
            true,
        )
    }
}
