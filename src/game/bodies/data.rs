use game::human::character::{age_name, Character};
use game::human::gender::Sex;
use game::human::hair_color::HairColor;
use game::human::skin_tone::SkinTone;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::convert::TryInto;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum Freshness {
    #[serde(rename = "f")]
    Fresh,
    #[serde(rename = "r")]
    Rotten,
    #[serde(rename = "s")]
    Skeletal,
}

impl Freshness {
    pub fn adjective(&self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Rotten => "rotten",
            Self::Skeletal => "skeletal",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum BodySize {
    #[serde(rename = "1")]
    Tiny,
    #[serde(rename = "2")]
    Small,
    #[serde(rename = "3")]
    Normal,
    #[serde(rename = "4")]
    Large,
    #[serde(rename = "5")]
    Huge,
}

impl Distribution<BodySize> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BodySize {
        match rng.gen_range(0..5) {
            // TODO: normal distribution
            0 => BodySize::Tiny,
            1 => BodySize::Small,
            2 => BodySize::Normal,
            3 => BodySize::Large,
            4 => BodySize::Huge,
            _ => unreachable!(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BodyPartData {
    #[serde(rename = "f")]
    pub freshness: Freshness,
    #[serde(rename = "a")]
    pub age: u8,
    #[serde(rename = "g")]
    pub sex: Sex,
    #[serde(rename = "s")]
    pub skin_tone: SkinTone,
    #[serde(rename = "z")]
    pub size: BodySize,
    #[serde(rename = "l")]
    pub alive: bool,
    #[serde(rename = "h")]
    pub hair_color: HairColor,
    // TODO: scars/tattoo/etc.
}

impl BodyPartData {
    pub fn new(character: &Character, freshness: Freshness) -> Self {
        Self {
            freshness,
            age: character.age,
            skin_tone: character.skin_tone,
            sex: (&character.gender).try_into().unwrap_or_default(),
            hair_color: if character.age < 50 {
                character.hair_color
            } else {
                HairColor::Gray
            },
            size: character.body_size,
            alive: character.alive,
        }
    }

    pub fn age_name(&self, with_gender: bool) -> &str {
        let gender = self.sex.into();
        age_name(self.age, if with_gender { Some(&gender) } else { None })
    }
}
