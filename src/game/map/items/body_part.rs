use super::super::super::human::body::{BodyPartData, Freshness};
use super::super::item::{ItemInteract, ItemView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BodyPart {
    pub data: BodyPartData,
    pub typ: BodyPartType,
}

impl BodyPart {
    pub fn new(data: BodyPartData, typ: BodyPartType) -> Self {
        Self { data, typ }
    }
}

impl ItemView for BodyPart {
    fn name(&self) -> String {
        let adjective = match self.data.freshness {
            Freshness::Fresh => "fresh",
            Freshness::Rotten => "rotten",
            Freshness::Skeletal => "skeletal",
        };
        let (name, with_gender) = match self.typ {
            BodyPartType::Head => ("head", true),
            BodyPartType::Eye => ("eye", false),
            BodyPartType::Nose => ("nose", true),
            BodyPartType::Mouth => ("mouth", true),
            BodyPartType::Ear => ("ear", true),
            BodyPartType::Brain => ("brain", false),
            BodyPartType::Torso => ("torso", true),
            BodyPartType::Heart => ("heart", false),
            BodyPartType::Stomach => ("stomach", false),
            BodyPartType::Lung => ("lung", false),
            BodyPartType::Kidney => ("kidney", false),
            BodyPartType::Liver => ("liver", false),
            BodyPartType::LeftArm => ("left arm", true),
            BodyPartType::LeftHand => ("left hand", true),
            BodyPartType::RightArm => ("right arm", true),
            BodyPartType::RightHand => ("right hand", true),
            BodyPartType::LeftLeg => ("left leg", true),
            BodyPartType::LeftFoot => ("left foot", true),
            BodyPartType::RightLeg => ("right leg", true),
            BodyPartType::RightFoot => ("right foot", true),
        };
        format!("{} {} {}", adjective, self.data.age_name(with_gender), name)
    }

    fn looks_like(&self) -> &'static str {
        "flesh"
    }
}

impl ItemInteract for BodyPart {
    fn mass(&self) -> u32 {
        match self.typ {
            BodyPartType::Head => 3_000,
            BodyPartType::Eye => 8,
            BodyPartType::Nose => 50,
            BodyPartType::Mouth => 100,
            BodyPartType::Ear => 50,
            BodyPartType::Brain => 1_400,
            BodyPartType::Torso => 25_000,
            BodyPartType::Heart => 250,
            BodyPartType::Stomach => 125,
            BodyPartType::Lung => 500,
            BodyPartType::Kidney => 100,
            BodyPartType::Liver => 1_500,
            BodyPartType::LeftArm => 3_000,
            BodyPartType::LeftHand => 500,
            BodyPartType::RightArm => 3_000,
            BodyPartType::RightHand => 500,
            BodyPartType::LeftLeg => 10_000,
            BodyPartType::LeftFoot => 750,
            BodyPartType::RightLeg => 10_000,
            BodyPartType::RightFoot => 750,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BodyPartType {
    Head,
    Eye,
    Nose,
    Mouth, // TODO: jaws, teeth
    Ear,
    Brain,
    Torso,
    Heart,
    Stomach,
    Lung,
    Kidney,
    Liver,
    LeftArm,
    LeftHand,
    RightArm,
    RightHand, // TODO: fingers
    LeftLeg,
    LeftFoot,
    RightLeg,
    RightFoot,
}
