use super::super::super::human::body::Body;
use super::super::super::human::character::Character;
use super::super::item::{Item, ItemInteract, ItemView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Corpse {
    pub character: Character,
    pub body: Body,
}

impl Corpse {
    pub fn new(character: Character, body: Body) -> Self {
        Self { character, body }
    }
}

impl ItemView for Corpse {
    fn name(&self) -> String {
        let mut adjectives = Vec::new();
        if self.body.wear.is_empty() {
            adjectives.push("naked");
        }
        let age_name = if let Some(torso) = self.body.parts.get("torso") {
            if let Item::BodyPart(bp) = torso {
                adjectives.push(bp.data.freshness.adjective());
                bp.data.age_name(true)
            } else {
                "dismembered"
            }
        } else {
            self.character.age_name()
        };
        format!("{} {} corpse", adjectives.join(" "), age_name)
    }

    fn looks_like(&self) -> &'static str {
        "corpse"
    }
}

impl ItemInteract for Corpse {
    fn mass(&self) -> u32 {
        // TODO: return body mass
        60_000
    }
}
