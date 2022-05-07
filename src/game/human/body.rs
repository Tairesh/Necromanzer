#![allow(dead_code)]
use super::super::map::item::Item;
use super::super::map::items::{BodyPart, BodyPartType};
use super::character::{age_name, Character};
use super::gender::Gender;
use super::skin_tone::SkinTone;
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
pub struct BodyPartData {
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

impl BodyPartData {
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
    pub parts: HashMap<String, Item>, // TODO: use static str
    // TODO: schema for placing items to different tiles
    pub wear: Vec<Item>, // TODO: some schema for bodyparts to wear
}

impl Body {
    pub fn human(character: &Character, freshness: Freshness) -> Self {
        let data = BodyPartData::new(character, freshness);
        let mut parts = HashMap::new();
        let mut head = data.clone();
        head.outside.insert(
            "left eye".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Eye).into(),
        );
        head.outside.insert(
            "right eye".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Eye).into(),
        );
        head.outside.insert(
            "nose".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Nose).into(),
        );
        head.outside.insert(
            "mouth".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Mouth).into(),
        );
        head.outside.insert(
            "left ear".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Ear).into(),
        );
        head.outside.insert(
            "right ear".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Ear).into(),
        );
        head.inside.insert(
            "brain".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Brain).into(),
        );
        parts.insert(
            "head".to_string(),
            BodyPart::new(head, BodyPartType::Head).into(),
        );
        let mut torso = data.clone();
        torso.inside.insert(
            "heart".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Heart).into(),
        );
        torso.inside.insert(
            "stomach".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Stomach).into(),
        );
        torso.inside.insert(
            "left lung".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Lung).into(),
        );
        torso.inside.insert(
            "right lung".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Lung).into(),
        );
        torso.inside.insert(
            "left kidney".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Kidney).into(),
        );
        torso.inside.insert(
            "right kidney".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Kidney).into(),
        );
        torso.inside.insert(
            "right kidney".to_string(),
            BodyPart::new(data.clone(), BodyPartType::Liver).into(),
        );
        parts.insert(
            "torso".to_string(),
            BodyPart::new(torso, BodyPartType::Torso).into(),
        );
        let mut left_arm = data.clone();
        left_arm.outside.insert(
            "hand".to_string(),
            BodyPart::new(data.clone(), BodyPartType::LeftHand).into(),
        );
        parts.insert(
            "left arm".to_string(),
            BodyPart::new(left_arm, BodyPartType::LeftArm).into(),
        );
        let mut right_arm = data.clone();
        right_arm.outside.insert(
            "hand".to_string(),
            BodyPart::new(data.clone(), BodyPartType::RightHand).into(),
        );
        parts.insert(
            "right arm".to_string(),
            BodyPart::new(right_arm, BodyPartType::LeftArm).into(),
        );
        let mut left_foot = data.clone();
        left_foot.outside.insert(
            "foot".to_string(),
            BodyPart::new(data.clone(), BodyPartType::LeftFoot).into(),
        );
        parts.insert(
            "left leg".to_string(),
            BodyPart::new(left_foot, BodyPartType::LeftLeg).into(),
        );
        let mut right_foot = data.clone();
        right_foot.outside.insert(
            "foot".to_string(),
            BodyPart::new(data, BodyPartType::RightFoot).into(),
        );
        parts.insert(
            "right leg".to_string(),
            BodyPart::new(right_foot, BodyPartType::RightLeg).into(),
        );

        Self {
            parts,
            wear: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::human::character::Character;
    use super::super::super::human::gender::Gender;
    use super::super::super::human::main_hand::MainHand;
    use super::super::super::human::skin_tone::SkinTone;
    use super::super::super::map::item::Item;
    use super::super::super::map::items::{BodyPart, BodyPartType};
    use super::{Body, Freshness};

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

        assert!(matches!(head, Item::BodyPart(..)));
        if let Item::BodyPart(bodypart) = &head {
            assert!(matches!(bodypart.typ, BodyPartType::Head));
            let data = &bodypart.data;
            assert!(matches!(data.freshness, Freshness::Fresh));
            assert!(matches!(data.skin_tone, SkinTone::Amber));
            let brain = data.inside.get("brain").unwrap();
            assert!(matches!(brain, Item::BodyPart(..)));
            if let Item::BodyPart(bodypart) = &brain {
                assert!(matches!(bodypart.typ, BodyPartType::Brain));
                let data = &bodypart.data;
                assert!(matches!(data.gender, Gender::Female));
            }
            assert_eq!(
                data.outside
                    .values()
                    .filter(|item| matches!(
                        item,
                        Item::BodyPart(BodyPart {
                            typ: BodyPartType::Eye,
                            ..
                        })
                    ))
                    .count(),
                2
            );
        }
    }
}
