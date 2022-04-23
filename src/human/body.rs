#![allow(dead_code)]
use human::character::{age_name, Character};
use human::gender::Gender;
use human::skin_tone::SkinTone;
use map::item::{Item, ItemType};
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum Freshness {
    #[serde(rename = "f")]
    Fresh,
    #[serde(rename = "r")]
    Rotten,
    #[serde(rename = "s")]
    Skeletal,
}

impl Freshness {
    pub fn adjective(&self) -> &str {
        match self {
            Freshness::Fresh => "fresh",
            Freshness::Rotten => "rotten",
            Freshness::Skeletal => "skeletal",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BodyPart {
    #[serde(rename = "f")]
    pub freshness: Freshness,
    #[serde(rename = "a")]
    pub age: u8,
    #[serde(rename = "g")]
    pub gender: Gender,
    #[serde(rename = "s")]
    pub skin_tone: SkinTone,
    #[serde(rename = "o")]
    pub outside: HashMap<String, Item>,
    #[serde(rename = "i")]
    pub inside: HashMap<String, Item>,
}

impl BodyPart {
    pub fn new(character: &Character, freshness: Freshness) -> Self {
        Self {
            freshness,
            age: character.age,
            gender: character.gender.clone(),
            skin_tone: character.skin_tone,
            outside: HashMap::new(),
            inside: HashMap::new(),
        }
    }

    pub fn age_name(&self, with_gender: bool) -> &str {
        age_name(
            self.age,
            if with_gender {
                Some(&self.gender)
            } else {
                None
            },
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Body {
    pub parts: HashMap<String, Item>,
    // TODO: schema for placing items to different tiles
    pub wear: Vec<Item>, // TODO: some schema for bodyparts to wear
}

impl Body {
    pub fn human(character: &Character, freshness: Freshness) -> Self {
        let body_part = BodyPart::new(character, freshness);
        let mut parts = HashMap::new();
        let mut head = body_part.clone();
        head.outside.insert(
            "left eye".to_string(),
            Item::new(ItemType::HumanEye(body_part.clone())),
        );
        head.outside.insert(
            "right eye".to_string(),
            Item::new(ItemType::HumanEye(body_part.clone())),
        );
        head.outside.insert(
            "nose".to_string(),
            Item::new(ItemType::HumanNose(body_part.clone())),
        );
        head.outside.insert(
            "mouth".to_string(),
            Item::new(ItemType::HumanMouth(body_part.clone())),
        );
        head.inside.insert(
            "brain".to_string(),
            Item::new(ItemType::HumanBrain(body_part.clone())),
        );
        parts.insert("head".to_string(), Item::new(ItemType::HumanHead(head)));
        let mut torso = body_part.clone();
        torso.inside.insert(
            "heart".to_string(),
            Item::new(ItemType::HumanHeart(body_part.clone())),
        );
        torso.inside.insert(
            "stomach".to_string(),
            Item::new(ItemType::HumanStomach(body_part.clone())),
        );
        torso.inside.insert(
            "left lung".to_string(),
            Item::new(ItemType::HumanLung(body_part.clone())),
        );
        torso.inside.insert(
            "right lung".to_string(),
            Item::new(ItemType::HumanLung(body_part.clone())),
        );
        torso.inside.insert(
            "left kidney".to_string(),
            Item::new(ItemType::HumanKidney(body_part.clone())),
        );
        torso.inside.insert(
            "right kidney".to_string(),
            Item::new(ItemType::HumanKidney(body_part.clone())),
        );
        parts.insert("torso".to_string(), Item::new(ItemType::HumanTorso(torso)));
        let mut left_arm = body_part.clone();
        left_arm.outside.insert(
            "hand".to_string(),
            Item::new(ItemType::HumanLeftHand(body_part.clone())),
        );
        parts.insert(
            "left arm".to_string(),
            Item::new(ItemType::HumanLeftArm(left_arm)),
        );
        let mut right_arm = body_part.clone();
        right_arm.outside.insert(
            "hand".to_string(),
            Item::new(ItemType::HumanRightHand(body_part.clone())),
        );
        parts.insert(
            "right arm".to_string(),
            Item::new(ItemType::HumanRightArm(right_arm)),
        );
        let mut left_foot = body_part.clone();
        left_foot.outside.insert(
            "foot".to_string(),
            Item::new(ItemType::HumanLeftFoot(body_part.clone())),
        );
        parts.insert(
            "left leg".to_string(),
            Item::new(ItemType::HumanLeftLeg(left_foot)),
        );
        let mut right_foot = body_part.clone();
        right_foot.outside.insert(
            "foot".to_string(),
            Item::new(ItemType::HumanRightFoot(body_part)),
        );
        parts.insert(
            "right leg".to_string(),
            Item::new(ItemType::HumanRightLeg(right_foot)),
        );

        Self {
            parts,
            wear: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use human::body::{Body, Freshness};
    use human::character::Character;
    use human::gender::Gender;
    use human::main_hand::MainHand;
    use human::skin_tone::SkinTone;
    use map::item::ItemType;

    #[test]
    fn test_human_creating() {
        let character = Character::new(
            "Ashley",
            Gender::Female,
            16,
            MainHand::Right,
            SkinTone::Amber,
        );
        let body = Body::human(&character, Freshness::Fresh);
        let head = body.parts.get("head").unwrap();
        assert!(matches!(head.item_type, ItemType::HumanHead(..)));
        if let ItemType::HumanHead(data) = &head.item_type {
            assert!(matches!(data.freshness, Freshness::Fresh));
            assert!(matches!(data.skin_tone, SkinTone::Amber));
            let brain = data.inside.get("brain").unwrap();
            assert!(matches!(brain.item_type, ItemType::HumanBrain(..)));
            if let ItemType::HumanBrain(data) = &brain.item_type {
                assert!(matches!(data.gender, Gender::Female));
            }
            assert_eq!(
                data.outside
                    .iter()
                    .filter(|(_, item)| matches!(item.item_type, ItemType::HumanEye(..)))
                    .count(),
                2
            );
        }
    }
}
