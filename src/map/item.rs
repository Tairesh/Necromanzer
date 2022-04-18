#![allow(dead_code)]
use assets::tileset::Tileset;
use game::Avatar;
use human::body::{Body, BodyPart};
use human::character::Character;
use human::main_hand::MainHand;
use map::terrains::GraveData;
use tetra::graphics::Rectangle;

// TODO: ItemTypes should be loaded from jsons for modding
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ItemType {
    Shovel,
    Knife,
    Axe,
    Corpse(Character, Body),
    GraveStone(GraveData),

    HumanHead(BodyPart),
    HumanEye(BodyPart),
    HumanNose(BodyPart),
    HumanMouth(BodyPart),
    HumanBrain(BodyPart),
    HumanTorso(BodyPart),
    HumanHeart(BodyPart),
    HumanStomach(BodyPart),
    HumanLung(BodyPart),
    HumanKidney(BodyPart),
    HumanLeftArm(BodyPart),
    HumanRightArm(BodyPart),
    HumanLeftHand(BodyPart),
    HumanRightHand(BodyPart),
    HumanLeftLeg(BodyPart),
    HumanRightLeg(BodyPart),
    HumanLeftFoot(BodyPart),
    HumanRightFoot(BodyPart),

    MagicHat,
    Hat,
    Cloak,
}

impl ItemType {
    pub fn region(&self, tileset: &Tileset) -> Rectangle {
        match self {
            ItemType::Shovel => tileset.shovel,
            ItemType::Knife => tileset.knife,
            ItemType::Axe => tileset.axe,
            ItemType::Hat | ItemType::MagicHat => tileset.hat,
            ItemType::Cloak => tileset.cloak,
            ItemType::Corpse(_, _) => tileset.corpse,
            ItemType::GraveStone(_) => tileset.grave_stone,
            ItemType::HumanHead(_)
            | ItemType::HumanEye(_)
            | ItemType::HumanNose(_)
            | ItemType::HumanMouth(_)
            | ItemType::HumanBrain(_)
            | ItemType::HumanTorso(_)
            | ItemType::HumanHeart(_)
            | ItemType::HumanStomach(_)
            | ItemType::HumanKidney(_)
            | ItemType::HumanLung(_)
            | ItemType::HumanLeftArm(_)
            | ItemType::HumanRightArm(_)
            | ItemType::HumanLeftHand(_)
            | ItemType::HumanRightHand(_)
            | ItemType::HumanLeftLeg(_)
            | ItemType::HumanRightLeg(_)
            | ItemType::HumanLeftFoot(_)
            | ItemType::HumanRightFoot(_) => tileset.flesh,
        }
    }

    pub fn name(&self) -> String {
        match self {
            ItemType::Shovel => "shovel".to_string(),
            ItemType::Knife => "knife".to_string(),
            ItemType::Axe => "axe".to_string(),
            ItemType::MagicHat => "magic hat".to_string(),
            ItemType::Hat => "hat".to_string(),
            ItemType::Cloak => "cloak".to_string(),
            ItemType::Corpse(c, _) => format!("corpse of {}", c.gender_age_name()),
            ItemType::GraveStone(_) => "gravestone".to_string(),
            ItemType::HumanHead(data) => format!("{} head", data.age_name(true)),
            ItemType::HumanEye(data) => format!("{} eye", data.age_name(false)),
            ItemType::HumanNose(data) => format!("{} nose", data.age_name(true)),
            ItemType::HumanMouth(data) => format!("{} mouth", data.age_name(true)),
            ItemType::HumanBrain(data) => format!("{} brain", data.age_name(false)),
            ItemType::HumanTorso(data) => format!("{} torso", data.age_name(true)),
            ItemType::HumanHeart(data) => format!("{} heart", data.age_name(false)),
            ItemType::HumanStomach(data) => format!("{} stomach", data.age_name(false)),
            ItemType::HumanLung(data) => format!("{} lung", data.age_name(false)),
            ItemType::HumanKidney(data) => format!("{} kidney", data.age_name(false)),
            ItemType::HumanLeftArm(data) => format!("{} left arm", data.age_name(true)),
            ItemType::HumanRightArm(data) => format!("{} right arm", data.age_name(true)),
            ItemType::HumanLeftHand(data) => format!("{} left hand", data.age_name(true)),
            ItemType::HumanRightHand(data) => format!("{} right hand", data.age_name(true)),
            ItemType::HumanLeftLeg(data) => format!("{} left leg", data.age_name(true)),
            ItemType::HumanRightLeg(data) => format!("{} right leg", data.age_name(true)),
            ItemType::HumanLeftFoot(data) => format!("{} left foot", data.age_name(true)),
            ItemType::HumanRightFoot(data) => format!("{} right foot", data.age_name(true)),
        }
    }

    pub fn wield_time(&self, avatar: &Avatar) -> f64 {
        let k = match avatar.character.main_hand {
            MainHand::Left => 1.1,
            MainHand::Right | MainHand::Ambidexter => 1.0,
        };
        k * match self {
            ItemType::Shovel => 30.0,
            ItemType::Knife => 20.0,
            ItemType::Axe => 25.0,
            ItemType::Hat | ItemType::MagicHat => 10.0,
            ItemType::Cloak => 15.0,
            ItemType::Corpse(c, _) => {
                if c.age < 16 {
                    50.0
                } else {
                    100.0
                }
            }
            ItemType::GraveStone(_) => 200.0,
            ItemType::HumanEye(_)
            | ItemType::HumanNose(_)
            | ItemType::HumanMouth(_)
            | ItemType::HumanBrain(_)
            | ItemType::HumanHeart(_)
            | ItemType::HumanStomach(_)
            | ItemType::HumanLung(_)
            | ItemType::HumanKidney(_)
            | ItemType::HumanLeftHand(_)
            | ItemType::HumanRightHand(_)
            | ItemType::HumanLeftFoot(_)
            | ItemType::HumanRightFoot(_) => 5.0,
            ItemType::HumanHead(_) => 10.0,
            ItemType::HumanTorso(_) => 20.0,
            ItemType::HumanLeftArm(_) | ItemType::HumanRightArm(_) => 12.0,
            ItemType::HumanLeftLeg(_) | ItemType::HumanRightLeg(_) => 15.0,
        }
    }

    pub fn drop_time(&self) -> f64 {
        10.0
    }

    pub fn body_part(&self) -> Option<&BodyPart> {
        match self {
            ItemType::Shovel
            | ItemType::Knife
            | ItemType::Axe
            | ItemType::Hat
            | ItemType::MagicHat
            | ItemType::Cloak
            | ItemType::Corpse(_, _)
            | ItemType::GraveStone(_) => None,
            ItemType::HumanHead(b)
            | ItemType::HumanEye(b)
            | ItemType::HumanNose(b)
            | ItemType::HumanMouth(b)
            | ItemType::HumanBrain(b)
            | ItemType::HumanTorso(b)
            | ItemType::HumanHeart(b)
            | ItemType::HumanStomach(b)
            | ItemType::HumanLung(b)
            | ItemType::HumanKidney(b)
            | ItemType::HumanLeftArm(b)
            | ItemType::HumanRightArm(b)
            | ItemType::HumanLeftHand(b)
            | ItemType::HumanRightHand(b)
            | ItemType::HumanLeftLeg(b)
            | ItemType::HumanRightLeg(b)
            | ItemType::HumanLeftFoot(b)
            | ItemType::HumanRightFoot(b) => Some(b),
        }
    }

    pub fn is_readable(&self) -> bool {
        matches!(self, ItemType::GraveStone(..))
    }

    pub fn read(&self) -> String {
        match self {
            ItemType::GraveStone(data) => data.read(),
            _ => unreachable!(),
        }
    }

    pub fn is_wearable(&self) -> bool {
        matches!(self, ItemType::Hat | ItemType::MagicHat | ItemType::Cloak)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Item {
    pub item_type: ItemType,
}

impl Item {
    pub fn new(item_type: ItemType) -> Self {
        Self { item_type }
    }

    pub fn name(&self) -> String {
        self.item_type.name()
    }

    pub fn is_readable(&self) -> bool {
        self.item_type.is_readable()
    }

    pub fn read(&self) -> String {
        self.item_type.read()
    }
}
