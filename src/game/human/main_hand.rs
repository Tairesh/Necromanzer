use num_enum::{IntoPrimitive, TryFromPrimitive};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::enums_iter;

#[derive(
    Serialize, Deserialize, IntoPrimitive, TryFromPrimitive, Debug, Copy, Clone, Eq, PartialEq,
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

    pub fn next(self) -> Self {
        enums_iter::next(self)
    }

    pub fn prev(self) -> Self {
        enums_iter::prev(self, 2)
    }
}

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
