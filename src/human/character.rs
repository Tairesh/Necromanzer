use assets::Names;
use human::gender::Gender;
use human::main_hand::MainHand;
use human::skin_tone::SkinTone;
use rand::distributions::Standard;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub gender: Gender,
    pub age: u8,
    pub main_hand: MainHand,
    pub skin_tone: SkinTone,
}

pub fn random_character<R: Rng + ?Sized>(rng: &mut R, names: &Names) -> Character {
    let gender = rng.sample(Standard);
    Character::new(
        format!(
            "{} {}",
            match gender {
                Gender::Male => &names.male_names,
                Gender::Female => &names.female_names,
                Gender::Custom(_) => &names.names,
            }
            .choose(rng)
            .unwrap(),
            names.names.choose(rng).unwrap()
        ),
        gender,
        rng.gen_range(0..=99),
        rng.sample(Standard),
        rng.sample(Standard),
    )
}

impl Character {
    pub fn new<S: Into<String>>(
        name: S,
        gender: Gender,
        age: u8,
        main_hand: MainHand,
        skin_tone: SkinTone,
    ) -> Self {
        Self {
            name: name.into(),
            gender,
            age,
            main_hand,
            skin_tone,
        }
    }
}
