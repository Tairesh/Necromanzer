use num_enum::{IntoPrimitive, TryFromPrimitive};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use variant_count::VariantCount;

use cycle_enum::{CycleEnum, VariantCount};

#[derive(
    Serialize,
    Deserialize,
    IntoPrimitive,
    TryFromPrimitive,
    VariantCount,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
)]
#[repr(u8)]
pub enum MainHand {
    #[serde(rename = "l")]
    Left,
    #[serde(rename = "r")]
    Right,
    #[serde(rename = "a")]
    Ambidexter,
}

impl MainHand {
    pub fn name(self) -> &'static str {
        self.into()
    }
}

impl VariantCount for MainHand {
    fn variant_count() -> usize {
        Self::VARIANT_COUNT
    }
}

impl CycleEnum for MainHand {}

impl From<MainHand> for &str {
    fn from(s: MainHand) -> Self {
        match s {
            MainHand::Left => "Left",
            MainHand::Right => "Right",
            MainHand::Ambidexter => "Ambidexter",
        }
    }
}

impl Distribution<MainHand> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MainHand {
        if rng.gen_bool(0.01) {
            MainHand::Ambidexter
        } else if rng.gen_bool(0.16) {
            MainHand::Left
        } else {
            MainHand::Right
        }
    }
}
