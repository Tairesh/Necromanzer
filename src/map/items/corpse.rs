use assets::tileset::Tileset;
use human::body::Body;
use human::character::Character;
use map::item::{Item, ItemInteract, ItemView};
use tetra::graphics::Rectangle;

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

    fn region(&self, tileset: &Tileset) -> Rectangle {
        tileset.corpse
    }
}

impl ItemInteract for Corpse {
    fn mass(&self) -> u32 {
        // TODO: return body mass
        60_000
    }
}
