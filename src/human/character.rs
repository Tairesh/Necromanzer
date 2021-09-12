use assets::Assets;
use human::gender::Gender;
use human::main_hand::MainHand;
use human::skin_tone::SkinTone;
use rand::distributions::Standard;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub gender: Gender,
    pub age: u8,
    pub main_hand: MainHand,
    pub skin_tone: SkinTone,
}

pub fn random_character<R: Rng + ?Sized>(rng: &mut R, assets: Rc<RefCell<Assets>>) -> Character {
    let gender = rng.sample(Standard);
    let assets = assets.borrow();
    Character::new(
        match gender {
            Gender::Male => &assets.male_names,
            Gender::Female => &assets.female_names,
            Gender::Custom(_) => &assets.names,
        }
        .choose(rng)
        .unwrap()
        .to_string(),
        gender,
        rng.gen_range(0..=99),
        rng.sample(Standard),
        rng.sample(Standard),
    )
}

impl Character {
    pub fn new(
        name: String,
        gender: Gender,
        age: u8,
        main_hand: MainHand,
        skin_tone: SkinTone,
    ) -> Self {
        Self {
            name,
            gender,
            age,
            main_hand,
            skin_tone,
        }
    }
}
