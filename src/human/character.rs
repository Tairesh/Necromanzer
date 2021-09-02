use assets::Assets;
use human::main_hand::MainHand;
use human::skin_tone::SkinTone;
use rand::distributions::Standard;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Character {
    pub name: String,
    pub gender: String,
    pub age: u8,
    pub main_hand: MainHand,
    pub skin_tone: SkinTone,
}

pub fn random_character<R: Rng + ?Sized>(rng: &mut R, assets: Rc<RefCell<Assets>>) -> Character {
    let gender = rng.gen_bool(0.51);
    let assets = assets.borrow();
    Character::new(
        (if gender {
            &assets.female_names
        } else {
            &assets.male_names
        })
        .choose(rng)
        .unwrap()
        .to_string(),
        (if gender { "Female" } else { "Male" }).to_string(),
        rng.gen_range(0..=99),
        rng.sample(Standard),
        rng.sample(Standard),
    )
}

impl Character {
    pub fn new(
        name: String,
        gender: String,
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
