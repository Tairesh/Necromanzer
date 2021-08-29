use human::main_hand::MainHand;
use human::skin_tone::SkinTone;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub gender: String,
    pub age: u8,
    pub main_hand: MainHand,
    pub skin_tone: SkinTone,
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
