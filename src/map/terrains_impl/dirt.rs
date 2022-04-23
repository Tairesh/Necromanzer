use assets::tileset::Tileset;
use map::item::Item;
use map::passage::Passage;
use map::terrain::{Terrain, TerrainInteract, TerrainView};
use map::terrains_impl::pit::Pit;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use tetra::graphics::Rectangle;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Dirt {
    #[serde(rename = "v")]
    variant: DirtVariant,
}

impl Dirt {
    pub fn new(variant: DirtVariant) -> Self {
        Self { variant }
    }
}

impl Default for Dirt {
    fn default() -> Self {
        Self::new(DirtVariant::Flat)
    }
}

impl TerrainView for Dirt {
    fn name(&self) -> &str {
        match self.variant {
            DirtVariant::Flat => "flat dirt",
            _ => "dirt",
        }
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        match self.variant {
            DirtVariant::LotOfChunks => tileset.dirt1,
            DirtVariant::SomeChunks => tileset.dirt2,
            DirtVariant::Flat => tileset.dirt3,
            DirtVariant::LittleChunks => tileset.dirt4,
            DirtVariant::AlmostNoChunks => tileset.dirt5,
        }
    }

    fn is_transparent(&self) -> bool {
        true
    }
}

impl TerrainInteract for Dirt {
    fn passage(&self) -> Passage {
        Passage::Passable(10.0)
    }

    fn is_diggable(&self) -> bool {
        true
    }

    fn dig_result(&self) -> (Terrain, Vec<Item>) {
        (Pit::new().into(), vec![])
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum DirtVariant {
    #[serde(rename = "1")]
    Flat,
    #[serde(rename = "2")]
    LotOfChunks,
    #[serde(rename = "3")]
    SomeChunks,
    #[serde(rename = "4")]
    LittleChunks,
    #[serde(rename = "5")]
    AlmostNoChunks,
}

impl Distribution<DirtVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DirtVariant {
        if rng.gen_bool(0.9) {
            DirtVariant::Flat
        } else {
            match rng.gen_range(0..4) {
                0 => DirtVariant::LotOfChunks,
                1 => DirtVariant::SomeChunks,
                2 => DirtVariant::LittleChunks,
                3 => DirtVariant::AlmostNoChunks,
                _ => unreachable!(),
            }
        }
    }
}
