use game::bodies::{Freshness, OrganData};
use game::human::character::{age_name, Character};
use game::human::gender::{Gender, Sex};
use game::human::hair_color::HairColor;
use game::human::skin_tone::SkinTone;

use super::super::item::{ItemInteract, ItemView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BodyPart {
    #[serde(rename = "n")]
    pub name: String, // like "second head", TODO: probably rename to ID or "key"
    #[serde(rename = "t")]
    pub typ: BodyPartType,
    #[serde(rename = "o")]
    pub outside: Vec<BodyPart>,
    #[serde(rename = "i")]
    pub inside: Vec<BodyPart>,
}

impl BodyPart {
    pub fn new<S: Into<String>>(name: S, typ: BodyPartType) -> Self {
        Self {
            name: name.into(),
            typ,
            outside: Vec::default(),
            inside: Vec::default(),
        }
    }

    pub fn with_inside(mut self, inside: Vec<BodyPart>) -> Self {
        self.inside = inside;
        self
    }

    pub fn with_outside(mut self, outside: Vec<BodyPart>) -> Self {
        self.outside = outside;
        self
    }

    pub fn organ_data(&self) -> &OrganData {
        match &self.typ {
            BodyPartType::Head(data, ..)
            | BodyPartType::Eye(data)
            | BodyPartType::Nose(data, ..)
            | BodyPartType::Mouth(data, ..)
            | BodyPartType::Ear(data, ..)
            | BodyPartType::Brain(data, ..)
            | BodyPartType::Torso(data, ..)
            | BodyPartType::Heart(data, ..)
            | BodyPartType::Stomach(data, ..)
            | BodyPartType::Lung(data, ..)
            | BodyPartType::Kidney(data, ..)
            | BodyPartType::Liver(data, ..)
            | BodyPartType::Intestines(data, ..)
            | BodyPartType::LeftArm(data, ..)
            | BodyPartType::LeftHand(data, ..)
            | BodyPartType::RightArm(data, ..)
            | BodyPartType::RightHand(data, ..)
            | BodyPartType::LeftLeg(data, ..)
            | BodyPartType::LeftFoot(data, ..)
            | BodyPartType::RightLeg(data, ..)
            | BodyPartType::RightFoot(data, ..) => data,
        }
    }

    pub fn freshness(&self) -> Freshness {
        self.organ_data().freshness
    }

    pub fn sex(&self) -> Option<Sex> {
        match &self.typ {
            BodyPartType::Head(.., sex)
            | BodyPartType::Mouth(.., sex)
            | BodyPartType::Torso(.., sex)
            | BodyPartType::LeftArm(.., sex)
            | BodyPartType::LeftHand(.., sex)
            | BodyPartType::RightArm(.., sex)
            | BodyPartType::RightHand(.., sex)
            | BodyPartType::LeftLeg(.., sex)
            | BodyPartType::LeftFoot(.., sex)
            | BodyPartType::RightLeg(.., sex)
            | BodyPartType::RightFoot(.., sex) => Some(*sex),
            _ => None,
        }
    }

    pub fn age_name(&self) -> &str {
        let gender = self.sex().map(Gender::from);
        age_name(
            self.organ_data().age,
            if let Some(gender) = &gender {
                Some(gender)
            } else {
                None
            },
        )
    }
}

impl ItemView for BodyPart {
    fn name(&self) -> String {
        let age_name = self.age_name();
        if matches!(
            self.typ,
            BodyPartType::Head(
                OrganData {
                    freshness: Freshness::Skeletal,
                    ..
                },
                ..
            )
        ) {
            return format!("{} skull", age_name);
        }
        let adjective = self.freshness().adjective();
        let name = match self.typ {
            BodyPartType::Head(_, _, _, _) => "head",
            BodyPartType::Eye(_) => "eye",
            BodyPartType::Nose(_, _) => "nose",
            BodyPartType::Mouth(_, _, _) => "mouth",
            BodyPartType::Ear(_, _) => "ear",
            BodyPartType::Brain(_, _) => "brain",
            BodyPartType::Torso(_, _, _, _) => "torso",
            BodyPartType::Heart(_) => "heart",
            BodyPartType::Stomach(_) => "stomach",
            BodyPartType::Lung(_) => "lung",
            BodyPartType::Kidney(_) => "kidney",
            BodyPartType::Liver(_) => "liver",
            BodyPartType::Intestines(_) => "intestines",
            BodyPartType::LeftArm(_, _, _) => "left arm",
            BodyPartType::LeftHand(_, _, _) => "left hand",
            BodyPartType::RightArm(_, _, _) => "right arm",
            BodyPartType::RightHand(_, _, _) => "right hand",
            BodyPartType::LeftLeg(_, _, _) => "left leg",
            BodyPartType::LeftFoot(_, _, _) => "left foot",
            BodyPartType::RightLeg(_, _, _) => "right leg",
            BodyPartType::RightFoot(_, _, _) => "right foot",
        };
        format!("{} {} {}", adjective, age_name, name)
    }

    fn looks_like(&self) -> &'static str {
        "flesh"
    }
}

impl ItemInteract for BodyPart {
    fn mass(&self) -> u32 {
        match self.typ {
            // TODO: use Sex and BodySize
            BodyPartType::Head(..) => 3_500,
            BodyPartType::Eye(..) => 8,
            BodyPartType::Nose(..) => 60,
            BodyPartType::Mouth(..) => 200,
            BodyPartType::Ear(..) => 50,
            BodyPartType::Brain(..) => 1_400,
            BodyPartType::Torso(..) => 25_000,
            BodyPartType::Heart(..) => 250,
            BodyPartType::Stomach(..) => 125,
            BodyPartType::Lung(..) => 600,
            BodyPartType::Kidney(..) => 100,
            BodyPartType::Liver(..) => 1_500,
            BodyPartType::Intestines(..) => 2_000,
            BodyPartType::LeftArm(..) | BodyPartType::RightArm(..) => 3_000,
            BodyPartType::LeftHand(..) | BodyPartType::RightHand(..) => 500,
            BodyPartType::LeftLeg(..) | BodyPartType::RightLeg(..) => 10_000,
            BodyPartType::LeftFoot(..) | BodyPartType::RightFoot(..) => 750,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BodyPartType {
    Head(OrganData, HairColor, SkinTone, Sex),
    Eye(OrganData), // TODO: eye color
    Nose(OrganData, SkinTone),
    Mouth(OrganData, SkinTone, Sex), // TODO: jaws, teeth, beard
    Ear(OrganData, SkinTone),
    Brain(OrganData, Character),
    Torso(OrganData, HairColor, SkinTone, Sex),
    Heart(OrganData),
    Stomach(OrganData),
    Lung(OrganData),
    Kidney(OrganData),
    Liver(OrganData),
    Intestines(OrganData),
    LeftArm(OrganData, SkinTone, Sex),  // TODO: shoulders
    LeftHand(OrganData, SkinTone, Sex), // TODO: fingers
    RightArm(OrganData, SkinTone, Sex),
    RightHand(OrganData, SkinTone, Sex),
    LeftLeg(OrganData, SkinTone, Sex),
    LeftFoot(OrganData, SkinTone, Sex),
    RightLeg(OrganData, SkinTone, Sex),
    RightFoot(OrganData, SkinTone, Sex),
}
