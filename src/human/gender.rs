use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Gender {
    Male,
    Female,
    Custom(String),
}

impl Gender {
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

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        if rng.gen_bool(0.51) {
            Gender::Female
        } else {
            Gender::Male
        }
    }
}
