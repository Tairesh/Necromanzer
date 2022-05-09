use std::convert::TryFrom;

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Gender {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "x")]
    Custom(String),
}

impl Gender {
    pub fn pronounce(&self) -> (&str, &str, &str) {
        match self {
            Gender::Male => ("He", "him", "his"),
            Gender::Female => ("She", "her", "her"),
            Gender::Custom(_) => ("They", "them", "their"),
        }
    }
}

impl From<String> for Gender {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            _ => Gender::Custom(value),
        }
    }
}

impl From<Gender> for String {
    fn from(gender: Gender) -> Self {
        match gender {
            Gender::Male => "Male".to_string(),
            Gender::Female => "Female".to_string(),
            Gender::Custom(s) => s,
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Sex {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
}

impl Default for Sex {
    fn default() -> Self {
        Self::Female
    }
}

impl TryFrom<&Gender> for Sex {
    type Error = &'static str;

    fn try_from(value: &Gender) -> Result<Self, Self::Error> {
        match value {
            Gender::Male => Ok(Self::Male),
            Gender::Female => Ok(Self::Female),
            Gender::Custom(_) => Err("Can't match custom genders to biological sex"),
        }
    }
}

impl From<Sex> for Gender {
    fn from(value: Sex) -> Self {
        match value {
            Sex::Male => Gender::Male,
            Sex::Female => Gender::Female,
        }
    }
}
