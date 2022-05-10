use std::collections::HashMap;
use std::convert::TryInto;

use game::bodies::{Body, Freshness, OrganData};
use game::human::character::Character;
use game::human::gender::Sex;
use game::human::hair_color::HairColor;
use game::human::skin_tone::SkinTone;
use game::map::items::{BodyPart, BodyPartType};
use game::map::pos::TilePos;

pub fn human_brain(organ_data: OrganData, character: Character) -> BodyPart {
    BodyPart::new("brain", BodyPartType::Brain(organ_data, character))
}

pub fn human_eye(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left eye" } else { "right eye" },
        BodyPartType::Eye(organ_data),
    )
}

pub fn human_nose(organ_data: OrganData, skin_tone: SkinTone) -> BodyPart {
    BodyPart::new("nose", BodyPartType::Nose(organ_data, skin_tone))
}

pub fn human_mouth(organ_data: OrganData, skin_tone: SkinTone, sex: Sex) -> BodyPart {
    BodyPart::new("mouth", BodyPartType::Mouth(organ_data, skin_tone, sex))
}

pub fn human_ear(organ_data: OrganData, skin_tone: SkinTone, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left ear" } else { "right ear" },
        BodyPartType::Ear(organ_data, skin_tone),
    )
}

pub fn human_head(character: &Character, freshness: Freshness) -> BodyPart {
    let organ_data = OrganData::new(character, freshness);
    let sex = (&character.mind.gender).try_into().unwrap_or_default();
    let hair_color = if character.appearance.age < 50 {
        character.appearance.hair_color
    } else {
        HairColor::Gray
    };
    let skin_tone = character.appearance.skin_tone;
    BodyPart::new(
        "head",
        BodyPartType::Head(organ_data.clone(), hair_color, skin_tone, sex),
    )
    .with_inside(match freshness {
        Freshness::Fresh | Freshness::Rotten => {
            vec![human_brain(organ_data.clone(), character.clone())]
        }
        Freshness::Skeletal => vec![],
    })
    .with_outside(match freshness {
        Freshness::Fresh => vec![
            human_eye(organ_data.clone(), true),
            human_eye(organ_data.clone(), false),
            human_nose(organ_data.clone(), skin_tone),
            human_ear(organ_data.clone(), skin_tone, true),
            human_ear(organ_data.clone(), skin_tone, false),
            human_mouth(organ_data, skin_tone, sex),
        ],
        Freshness::Rotten => vec![
            human_nose(organ_data.clone(), skin_tone),
            human_mouth(organ_data.clone(), skin_tone, sex),
            human_ear(organ_data.clone(), skin_tone, true),
            human_ear(organ_data, skin_tone, false),
        ],
        Freshness::Skeletal => vec![human_mouth(organ_data, skin_tone, sex)],
    })
}

pub fn human_heart(organ_data: OrganData) -> BodyPart {
    BodyPart::new("heart", BodyPartType::Heart(organ_data))
}

pub fn human_stomach(organ_data: OrganData) -> BodyPart {
    BodyPart::new("stomach", BodyPartType::Stomach(organ_data))
}

pub fn human_liver(organ_data: OrganData) -> BodyPart {
    BodyPart::new("liver", BodyPartType::Liver(organ_data))
}

pub fn human_intestines(organ_data: OrganData) -> BodyPart {
    BodyPart::new("intestines", BodyPartType::Intestines(organ_data))
}

pub fn human_lung(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left lung" } else { "right lung" },
        BodyPartType::Lung(organ_data),
    )
}

pub fn human_kidney(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left kidney" } else { "right kidney" },
        BodyPartType::Kidney(organ_data),
    )
}

pub fn human_arm(organ_data: OrganData, skin_tone: SkinTone, sex: Sex, left: bool) -> BodyPart {
    let mut arm = BodyPart::new(
        if left { "left arm" } else { "right arm" },
        if left {
            BodyPartType::LeftArm(organ_data.clone(), skin_tone, sex)
        } else {
            BodyPartType::RightArm(organ_data.clone(), skin_tone, sex)
        },
    );
    arm.outside
        .push(human_hand(organ_data, skin_tone, sex, left));

    arm
}

pub fn human_hand(organ_data: OrganData, skin_tone: SkinTone, sex: Sex, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left hand" } else { "right hand" },
        if left {
            BodyPartType::LeftHand(organ_data, skin_tone, sex)
        } else {
            BodyPartType::RightHand(organ_data, skin_tone, sex)
        },
    )
}

pub fn human_leg(organ_data: OrganData, skin_tone: SkinTone, sex: Sex, left: bool) -> BodyPart {
    let mut leg = BodyPart::new(
        if left { "left leg" } else { "right leg" },
        if left {
            BodyPartType::LeftLeg(organ_data.clone(), skin_tone, sex)
        } else {
            BodyPartType::RightLeg(organ_data.clone(), skin_tone, sex)
        },
    );
    leg.outside
        .push(human_foot(organ_data, skin_tone, sex, left));

    leg
}

pub fn human_foot(organ_data: OrganData, skin_tone: SkinTone, sex: Sex, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left foot" } else { "right foot" },
        if left {
            BodyPartType::LeftFoot(organ_data, skin_tone, sex)
        } else {
            BodyPartType::RightFoot(organ_data, skin_tone, sex)
        },
    )
}

pub fn human_torso(character: &Character, freshness: Freshness) -> BodyPart {
    let organ_data = OrganData::new(character, freshness);
    let skin_tone = character.appearance.skin_tone;
    let hair_color = character.appearance.hair_color;
    let sex = (&character.mind.gender).try_into().unwrap_or_default();
    BodyPart::new(
        "torso",
        BodyPartType::Torso(organ_data.clone(), hair_color, skin_tone, sex),
    )
    .with_inside(match freshness {
        Freshness::Fresh | Freshness::Rotten => vec![
            human_heart(organ_data.clone()),
            human_lung(organ_data.clone(), true),
            human_lung(organ_data.clone(), false),
            human_stomach(organ_data.clone()),
            human_kidney(organ_data.clone(), true),
            human_kidney(organ_data.clone(), false),
            human_liver(organ_data.clone()),
            human_intestines(organ_data.clone()),
        ],
        Freshness::Skeletal => vec![],
    })
    .with_outside(vec![
        human_head(character, freshness),
        human_arm(organ_data.clone(), skin_tone, sex, true),
        human_arm(organ_data.clone(), skin_tone, sex, false),
        human_leg(organ_data.clone(), skin_tone, sex, true),
        human_leg(organ_data, skin_tone, sex, false),
    ])
}

pub fn human_body(character: &Character, freshness: Freshness) -> Body {
    let parts = HashMap::from([(TilePos::new(0, 0), human_torso(character, freshness))]);
    Body::new(parts)
}

#[cfg(test)]
mod tests {
    use game::bodies::helpers::{human_body, human_torso};
    use game::bodies::{BodySize, Freshness, OrganData};
    use game::human::character::tests::{dead_boy, old_queer, tester_girl};
    use game::human::character::{Appearance, Character, Mind};
    use game::human::gender::{Gender, Sex};
    use game::human::hair_color::HairColor;
    use game::human::main_hand::MainHand;
    use game::human::skin_tone::SkinTone;
    use game::map::item::ItemView;
    use game::map::items::{BodyPart, BodyPartType};
    use game::map::pos::TilePos;

    use super::human_head;

    #[test]
    fn test_fresh_head() {
        let character = tester_girl();
        let head = human_head(&character, Freshness::Fresh);
        assert_eq!("head", head.name);
        assert!(matches!(
            head,
            BodyPart {
                typ: BodyPartType::Head(
                    OrganData {
                        age: 15,
                        alive: true,
                        size: BodySize::Small,
                        freshness: Freshness::Fresh,
                    },
                    HairColor::Ginger,
                    SkinTone::WarmIvory,
                    Sex::Female,
                ),
                ..
            }
        ));
        assert!(matches!(
            head.inside.iter().next(),
            Some(BodyPart {
                typ: BodyPartType::Brain(
                    OrganData {
                        freshness: Freshness::Fresh,
                        age: 15,
                        size: BodySize::Small,
                        alive: true,
                    },
                    Character {
                        appearance: Appearance {
                            age: 15,
                            hair_color: HairColor::Ginger,
                            body_size: BodySize::Small,
                            skin_tone: SkinTone::WarmIvory,
                        },
                        mind: Mind {
                            alive: true,
                            gender: Gender::Female,
                            main_hand: MainHand::Left,
                            ..
                        }
                    }
                ),
                ..
            })
        ));
        assert_eq!(
            2,
            head.outside
                .iter()
                .filter(|bp| matches!(
                    bp,
                    BodyPart {
                        typ: BodyPartType::Eye(OrganData {
                            freshness: Freshness::Fresh,
                            age: 15,
                            size: BodySize::Small,
                            alive: true,
                        }),
                        ..
                    }
                ))
                .count()
        );
        assert_eq!(
            2,
            head.outside
                .iter()
                .filter(|bp| matches!(
                    bp,
                    BodyPart {
                        typ: BodyPartType::Ear(
                            OrganData {
                                freshness: Freshness::Fresh,
                                age: 15,
                                size: BodySize::Small,
                                alive: true,
                            },
                            SkinTone::WarmIvory
                        ),
                        ..
                    }
                ))
                .count()
        );
    }

    #[test]
    fn test_rotten_head() {
        let character = dead_boy();
        let head = human_head(&character, Freshness::Rotten);
        assert_eq!("head", head.name);
        assert!(matches!(
            head,
            BodyPart {
                typ: BodyPartType::Head(
                    OrganData {
                        age: 9,
                        alive: false,
                        size: BodySize::Tiny,
                        freshness: Freshness::Rotten,
                    },
                    HairColor::Black,
                    SkinTone::Almond,
                    Sex::Male,
                ),
                ..
            }
        ));
        assert!(matches!(
            head.inside.iter().next(),
            Some(BodyPart {
                typ: BodyPartType::Brain(
                    OrganData {
                        freshness: Freshness::Rotten,
                        age: 9,
                        ..
                    },
                    Character {
                        mind: Mind {
                            gender: Gender::Male,
                            ..
                        },
                        ..
                    }
                ),
                ..
            })
        ));
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Eye(..)))
                .count()
        );
        assert_eq!(
            2,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Ear(..)))
                .count()
        );
    }

    #[test]
    fn test_skeletal_head() {
        let character = dead_boy();
        let head = human_head(&character, Freshness::Skeletal);
        assert_eq!("head", head.name);
        assert!(matches!(
            head,
            BodyPart {
                typ: BodyPartType::Head(
                    OrganData {
                        age: 9,
                        alive: false,
                        size: BodySize::Tiny,
                        freshness: Freshness::Skeletal,
                    },
                    HairColor::Black,
                    SkinTone::Almond,
                    Sex::Male,
                ),
                ..
            }
        ));
        assert!(head.inside.is_empty());
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Eye(..)))
                .count()
        );
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Ear(..)))
                .count()
        );
    }

    #[test]
    fn test_fresh_torso() {
        let character = tester_girl();
        let torso = human_torso(&character, Freshness::Fresh);
        assert_eq!("torso", torso.name);
        assert_eq!("fresh girl torso", torso.name());
        assert!(matches!(
            torso,
            BodyPart {
                typ: BodyPartType::Torso(
                    OrganData {
                        age: 15,
                        alive: true,
                        freshness: Freshness::Fresh,
                        size: BodySize::Small,
                    },
                    HairColor::Ginger,
                    SkinTone::WarmIvory,
                    Sex::Female,
                ),
                ..
            }
        ));
        assert_eq!(
            1,
            torso
                .inside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Heart(..)))
                .count()
        );
        assert_eq!(
            // А в каждом человеке есть два танцора: правое и левое.
            // Один танцор - правое, другой - левое.
            // Два легких танцора. Два легких. Правое легкое и левое.
            // В каждом человеке два танцора - его правое и левое легкое.
            // Легкие танцуют, и человек получает кислород.
            2,
            torso
                .inside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Lung(..)))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Head(..)))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::LeftArm(..)))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::RightArm(..)))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::LeftLeg(..)))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::RightLeg(..)))
                .count()
        );
    }

    #[test]
    fn test_old_man_body() {
        let character = old_queer();
        let body = human_body(&character, Freshness::Fresh);
        let torso = body.parts.get(&TilePos::new(0, 0)).unwrap();
        let head = torso.outside.first().unwrap();
        assert!(matches!(
            head.typ,
            BodyPartType::Head(
                OrganData {
                    freshness: Freshness::Fresh,
                    age: 75,
                    size: BodySize::Large,
                    alive: true,
                },
                HairColor::Gray,
                SkinTone::Almond,
                Sex::Female
            )
        ));
    }
}
