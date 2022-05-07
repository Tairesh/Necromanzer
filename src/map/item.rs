use super::items::*;
use enum_dispatch::enum_dispatch;
use game::Avatar;
use human::main_hand::MainHand;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[enum_dispatch]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Item {
    Shovel,
    Axe,
    Knife,
    Corpse,
    Gravestone,
    BodyPart,
    Hat,
    Cloak,
    Rags,
}

#[enum_dispatch(Item)]
pub trait ItemView {
    fn name(&self) -> String;
    fn looks_like(&self) -> &'static str;
}

#[enum_dispatch(Item)]
pub trait ItemInteract {
    fn tags(&self) -> HashSet<ItemTag> {
        HashSet::new()
    }
    fn mass(&self) -> u32; // in grams
    fn wield_time(&self, avatar: &Avatar) -> f64 {
        let k = match avatar.character.main_hand {
            MainHand::Left => 1.1,
            MainHand::Right | MainHand::Ambidexter => 1.0,
        };

        // 100 grams per tick
        k * self.mass() as f64 / 100.0
    }
    fn drop_time(&self, _avatar: &Avatar) -> f64 {
        10.0
    }
    // TODO: same as TerrainInteract
    fn is_readable(&self) -> bool {
        false
    }
    fn read(&self) -> String {
        unreachable!()
    }
    fn is_wearable(&self) -> bool {
        false
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ItemTag {
    Dig,
    Butch,
}

#[cfg(test)]
mod tests {
    use super::{Item, ItemInteract, ItemTag, ItemView};
    use human::body::{Body, BodyPartData, Freshness};
    use human::character::Character;
    use human::gender::Gender;
    use human::main_hand::MainHand;
    use human::skin_tone::SkinTone;
    use map::items::{Axe, BodyPart, BodyPartType, Cloak, Corpse, Gravestone, Hat, Knife, Shovel};
    use map::terrains::GraveData;

    #[test]
    fn test_shovel() {
        let shovel: Item = Shovel::new().into();
        assert_eq!("shovel", shovel.name());
        assert!(shovel.tags().contains(&ItemTag::Dig));
    }

    #[test]
    fn test_axe() {
        let axe: Item = Axe::new().into();
        assert_eq!("axe", axe.name());
        assert!(axe.tags().contains(&ItemTag::Butch));
        assert!(!axe.tags().contains(&ItemTag::Dig));
    }

    #[test]
    fn test_knife() {
        let knife: Item = Knife::new().into();
        assert_eq!("knife", knife.name());
        assert!(knife.tags().contains(&ItemTag::Butch));
        assert!(!knife.tags().contains(&ItemTag::Dig));
    }

    #[test]
    fn test_corpse() {
        let character = Character::new("test", Gender::Male, 15, MainHand::Right, SkinTone::Almond);
        let body = Body::human(&character, Freshness::Rotten);
        let corpse: Item = Corpse::new(character, body).into();
        assert_eq!("naked rotten boy corpse", corpse.name());
    }

    #[test]
    fn test_bodypart() {
        let character = Character::new("test", Gender::Male, 15, MainHand::Right, SkinTone::Almond);
        let brain: Item = BodyPart::new(
            BodyPartData::new(&character, Freshness::Fresh),
            BodyPartType::Brain,
        )
        .into();
        assert_eq!("fresh child brain", brain.name());
        let head: Item = BodyPart::new(
            BodyPartData::new(&character, Freshness::Skeletal),
            BodyPartType::Head,
        )
        .into();
        assert_eq!("skeletal boy head", head.name());
    }

    #[test]
    fn test_gravestone() {
        let character = Character::new("test", Gender::Male, 15, MainHand::Right, SkinTone::Almond);
        let gravestone: Item = Gravestone::new(GraveData {
            character,
            death_year: 255,
        })
        .into();
        assert_eq!("gravestone", gravestone.name());
        assert!(gravestone.is_readable());
        assert!(gravestone.read().contains("test"));
    }

    #[test]
    fn test_hat() {
        let hat: Item = Hat::new().into();
        assert_eq!("hat", hat.name());
        assert!(hat.is_wearable());
    }

    #[test]
    fn test_cloak() {
        let cloak: Item = Cloak::new().into();
        assert_eq!("cloak", cloak.name());
        assert!(cloak.is_wearable());
    }
}
