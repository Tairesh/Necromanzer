use game::bodies::{Body, BodyPartData, Freshness};
use game::human::character::Character;
use game::map::items::{BodyPart, BodyPartType};
use game::map::pos::TilePos;
use std::collections::HashMap;

pub fn human_brain(data: BodyPartData) -> BodyPart {
    BodyPart::new("brain", data, BodyPartType::Brain)
}

pub fn human_eye(data: BodyPartData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left eye" } else { "right eye" },
        data,
        BodyPartType::Eye,
    )
}

pub fn human_nose(data: BodyPartData) -> BodyPart {
    BodyPart::new("nose", data, BodyPartType::Nose)
}

pub fn human_mouth(data: BodyPartData) -> BodyPart {
    BodyPart::new("mouth", data, BodyPartType::Mouth)
}

pub fn human_ear(data: BodyPartData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left ear" } else { "right ear" },
        data,
        BodyPartType::Ear,
    )
}

pub fn human_head(data: BodyPartData) -> BodyPart {
    BodyPart::new("head", data.clone(), BodyPartType::Head)
        .with_inside(match data.freshness {
            Freshness::Fresh | Freshness::Rotten => vec![human_brain(data.clone())],
            Freshness::Skeletal => vec![],
        })
        .with_outside(match data.freshness {
            Freshness::Fresh => vec![
                human_eye(data.clone(), true),
                human_eye(data.clone(), false),
                human_nose(data.clone()),
                human_ear(data.clone(), true),
                human_ear(data.clone(), false),
                human_mouth(data),
            ],
            Freshness::Rotten => vec![
                human_nose(data.clone()),
                human_mouth(data.clone()),
                human_ear(data.clone(), true),
                human_ear(data, false),
            ],
            Freshness::Skeletal => vec![human_mouth(data)],
        })
}

pub fn human_heart(data: BodyPartData) -> BodyPart {
    BodyPart::new("heart", data, BodyPartType::Heart)
}

pub fn human_stomach(data: BodyPartData) -> BodyPart {
    BodyPart::new("stomach", data, BodyPartType::Stomach)
}

pub fn human_liver(data: BodyPartData) -> BodyPart {
    BodyPart::new("liver", data, BodyPartType::Liver)
}

pub fn human_intestines(data: BodyPartData) -> BodyPart {
    BodyPart::new("intestines", data, BodyPartType::Intestines)
}

pub fn human_lung(data: BodyPartData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left lung" } else { "right lung" },
        data,
        BodyPartType::Lung,
    )
}

pub fn human_kidney(data: BodyPartData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left kidney" } else { "right kidney" },
        data,
        BodyPartType::Kidney,
    )
}

pub fn human_torso(data: BodyPartData) -> BodyPart {
    BodyPart::new("torso", data.clone(), BodyPartType::Torso)
        .with_inside(match data.freshness {
            Freshness::Fresh | Freshness::Rotten => vec![
                human_heart(data.clone()),
                human_lung(data.clone(), true),
                human_lung(data.clone(), false),
                human_stomach(data.clone()),
                human_kidney(data.clone(), true),
                human_kidney(data.clone(), false),
                human_liver(data.clone()),
                human_intestines(data.clone()),
            ],
            Freshness::Skeletal => vec![],
        })
        .with_outside(vec![
            human_head(data.clone()),
            human_arm(data.clone(), true),
            human_arm(data.clone(), false),
            human_leg(data.clone(), true),
            human_leg(data, false),
        ])
}

pub fn human_arm(data: BodyPartData, left: bool) -> BodyPart {
    let mut arm = BodyPart::new(
        if left { "left arm" } else { "right arm" },
        data.clone(),
        if left {
            BodyPartType::LeftArm
        } else {
            BodyPartType::RightArm
        },
    );
    arm.outside.push(human_hand(data, left));

    arm
}

pub fn human_hand(data: BodyPartData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left hand" } else { "right hand" },
        data,
        if left {
            BodyPartType::LeftHand
        } else {
            BodyPartType::RightHand
        },
    )
}

pub fn human_leg(data: BodyPartData, left: bool) -> BodyPart {
    let mut leg = BodyPart::new(
        if left { "left leg" } else { "right leg" },
        data.clone(),
        if left {
            BodyPartType::LeftLeg
        } else {
            BodyPartType::RightLeg
        },
    );
    leg.outside.push(human_foot(data, left));

    leg
}

pub fn human_foot(data: BodyPartData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left foot" } else { "right foot" },
        data,
        if left {
            BodyPartType::LeftFoot
        } else {
            BodyPartType::RightFoot
        },
    )
}

pub fn human_body(character: &Character, freshness: Freshness) -> Body {
    let pos = TilePos::new(0, 0);
    let data = BodyPartData::new(character, freshness);
    let parts = HashMap::from([(pos, human_torso(data))]);
    Body::new(parts)
}

#[cfg(test)]
mod tests {
    use super::human_head;
    use game::bodies::helpers::human_torso;
    use game::bodies::{BodyPartData, Freshness};
    use game::human::character::tests::{dead_boy, tester_girl};
    use game::human::gender::Sex;
    use game::map::item::ItemView;
    use game::map::items::{BodyPart, BodyPartType};

    #[test]
    fn test_fresh_head() {
        let character = tester_girl();
        let data = BodyPartData::new(&character, Freshness::Fresh);
        let head = human_head(data);
        assert_eq!("head", head.name);
        assert_eq!(Sex::Female, head.data.sex);
        assert_eq!(character.hair_color, head.data.hair_color);
        assert!(matches!(
            head.inside.iter().next(),
            Some(BodyPart {
                typ: BodyPartType::Brain,
                data: BodyPartData {
                    freshness: Freshness::Fresh,
                    age: 15,
                    sex: Sex::Female,
                    ..
                },
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
                        typ: BodyPartType::Eye,
                        data: BodyPartData {
                            freshness: Freshness::Fresh,
                            age: 15,
                            sex: Sex::Female,
                            ..
                        },
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
                        typ: BodyPartType::Ear,
                        data: BodyPartData {
                            freshness: Freshness::Fresh,
                            age: 15,
                            sex: Sex::Female,
                            ..
                        },
                        ..
                    }
                ))
                .count()
        );
    }

    #[test]
    fn test_rotten_head() {
        let character = dead_boy();
        let data = BodyPartData::new(&character, Freshness::Rotten);
        let head = human_head(data);
        assert_eq!("head", head.name);
        assert_eq!(Sex::Male, head.data.sex);
        assert_eq!(character.hair_color, head.data.hair_color);
        assert!(matches!(
            head.inside.iter().next(),
            Some(BodyPart {
                typ: BodyPartType::Brain,
                data: BodyPartData {
                    freshness: Freshness::Rotten,
                    age: 9,
                    sex: Sex::Male,
                    ..
                },
                ..
            })
        ));
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Eye))
                .count()
        );
        assert_eq!(
            2,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Ear))
                .count()
        );
    }

    #[test]
    fn test_skeletal_head() {
        let character = dead_boy();
        let data = BodyPartData::new(&character, Freshness::Skeletal);
        let head = human_head(data);
        assert_eq!("head", head.name);
        assert_eq!(Sex::Male, head.data.sex);
        assert!(head.inside.is_empty());
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Eye))
                .count()
        );
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Ear))
                .count()
        );
    }

    #[test]
    fn test_fresh_torso() {
        let character = tester_girl();
        let data = BodyPartData::new(&character, Freshness::Fresh);
        let torso = human_torso(data);
        assert_eq!("torso", torso.name);
        assert_eq!("fresh girl torso", torso.name());
        assert_eq!(Sex::Female, torso.data.sex);
        assert_eq!(15, torso.data.age);
        assert_eq!(
            1,
            torso
                .inside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Heart))
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
                .filter(|bp| matches!(bp.typ, BodyPartType::Lung))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Head))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::LeftArm))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::RightArm))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::LeftLeg))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::RightLeg))
                .count()
        );
    }
}
