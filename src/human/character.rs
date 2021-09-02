use human::main_hand::MainHand;
use human::skin_tone::SkinTone;
use rand::distributions::Standard;
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Character {
    pub name: String,
    pub gender: String,
    pub age: u8,
    pub main_hand: MainHand,
    pub skin_tone: SkinTone,
}

pub fn random_character<R: Rng + ?Sized>(rng: &mut R) -> Character {
    let gender = rng.gen_bool(0.51);
    let name = "Ashley";
    Character::new(
        name.to_string(),
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
