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
pub struct Appearance {
    #[serde(rename = "a")]
    pub age: u8,
    #[serde(rename = "s")]
    pub skin_tone: SkinTone,
    #[serde(rename = "h")]
    pub hair_color: HairColor,
    #[serde(rename = "z")]
    pub body_size: BodySize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mind {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "g")]
    pub gender: Gender,
    #[serde(rename = "m")]
    pub main_hand: MainHand,
    #[serde(rename = "l")]
    pub alive: bool,
    // TODO: profession
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    #[serde(rename = "a")]
    pub appearance: Appearance,
    #[serde(rename = "m")]
    pub mind: Mind,
}

impl Character {
    pub fn new(appearance: Appearance, mind: Mind) -> Self {
        Self { appearance, mind }
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
            Appearance {
                age: rng.gen_range(0..=99),
                skin_tone: rng.sample(Standard),
                hair_color: rng.sample(Standard),
                body_size: rng.sample(Standard),
            },
            Mind {
                name,
                gender,
                main_hand: rng.sample(Standard),
                alive,
            },
        )
    }

    pub fn age_name(&self) -> &str {
        age_name(self.appearance.age, Some(&self.mind.gender))
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
    use game::human::character::{Appearance, Mind};
    use game::human::gender::Gender;
    use game::human::hair_color::HairColor;
    use game::human::main_hand::MainHand;
    use game::human::skin_tone::SkinTone;

    use super::Character;

    pub fn dead_boy() -> Character {
        Character::new(
            Appearance {
                age: 9,
                skin_tone: SkinTone::Almond,
                hair_color: HairColor::Black,
                body_size: BodySize::Tiny,
            },
            Mind {
                name: "Dead Boy".to_string(),
                gender: Gender::Male,
                main_hand: MainHand::Right,
                alive: false,
            },
        )
    }

    pub fn tester_girl() -> Character {
        Character::new(
            Appearance {
                age: 15,
                skin_tone: SkinTone::WarmIvory,
                hair_color: HairColor::Ginger,
                body_size: BodySize::Small,
            },
            Mind {
                name: "Tester Girl".to_string(),
                gender: Gender::Female,
                main_hand: MainHand::Left,
                alive: true,
            },
        )
    }

    pub fn old_queer() -> Character {
        Character::new(
            Appearance {
                age: 75,
                skin_tone: SkinTone::Almond,
                hair_color: HairColor::Black,
                body_size: BodySize::Large,
            },
            Mind {
                name: "Old Queer".to_string(),
                gender: Gender::Custom("X".to_string()),
                main_hand: MainHand::Ambidexter,
                alive: true,
            },
        )
    }

    #[test]
    fn test_age_name() {
        let character = tester_girl();
        assert_eq!("girl", character.age_name());
    }
}
