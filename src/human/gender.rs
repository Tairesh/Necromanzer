use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::Serializer;
use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Custom(String),
}

impl Gender {
    // TODO: impl From<String> for Gender
    pub fn from_string(value: String) -> Self {
        match value.as_str() {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            _ => Gender::Custom(value),
        }
    }

    pub fn pronounce(&self) -> &str {
        match self {
            Gender::Male => "He",
            Gender::Female => "She",
            Gender::Custom(_) => "They",
        }
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.collect_str(match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Custom(s) => s,
        })
    }
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        if rng.gen_bool(0.51) {
            Gender::Female
        } else {
            Gender::Male
        }
    }
}
