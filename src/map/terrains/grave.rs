use assets::tileset::Tileset;
use human::body::{Body, Freshness};
use human::character::Character;
use map::item::Item;
use map::items::{Corpse, Gravestone};
use map::passage::Passage;
use map::terrain::{Terrain, TerrainInteract, TerrainView};
use map::terrains::pit::Pit;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use tetra::graphics::Rectangle;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Grave {
    #[serde(rename = "v")]
    variant: GraveVariant,
    #[serde(rename = "d")]
    data: GraveData,
}

impl Grave {
    pub fn new(variant: GraveVariant, data: GraveData) -> Self {
        Self { variant, data }
    }
}

impl TerrainView for Grave {
    fn name(&self) -> &str {
        match self.variant {
            GraveVariant::New => "grave",
            GraveVariant::Old => "old grave",
        }
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        match self.variant {
            GraveVariant::New => tileset.grave_new,
            GraveVariant::Old => tileset.grave_old,
        }
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

impl TerrainInteract for Grave {
    fn passage(&self) -> Passage {
        Passage::Unpassable
    }

    fn is_diggable(&self) -> bool {
        true
    }

    fn dig_result(&self) -> (Terrain, Vec<Item>) {
        (
            Pit::new().into(),
            vec![
                Gravestone::new(self.data.clone()).into(),
                Corpse::new(
                    self.data.character.clone(),
                    Body::human(
                        &self.data.character,
                        match self.data.death_year {
                            253..=255 => Freshness::Rotten,
                            _ => Freshness::Skeletal,
                        },
                    ),
                )
                .into(),
            ],
        )
    }

    fn is_readable(&self) -> bool {
        true
    }

    fn read(&self) -> String {
        self.data.read()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum GraveVariant {
    New,
    Old,
}

impl Distribution<GraveVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GraveVariant {
        if rng.gen_bool(0.9) {
            GraveVariant::Old
        } else {
            GraveVariant::New
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GraveData {
    #[serde(rename = "c")]
    pub character: Character,
    #[serde(rename = "d")]
    pub death_year: u8,
}

impl GraveData {
    pub fn read(&self) -> String {
        format!(
            "You read on gravestone: {}. {} — {}",
            self.character.name, // TODO: random mottos, professions, etc.
            self.death_year as i32 - self.character.age as i32,
            self.death_year
        )
    }
}
