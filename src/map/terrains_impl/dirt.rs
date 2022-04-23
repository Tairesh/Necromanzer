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
        Self::new(DirtVariant::Dirt3)
    }
}

impl TerrainView for Dirt {
    fn name(&self) -> &str {
        match self.variant {
            DirtVariant::Dirt3 => "flat dirt",
            _ => "dirt",
        }
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        match self.variant {
            DirtVariant::Dirt1 => tileset.dirt1,
            DirtVariant::Dirt2 => tileset.dirt2,
            DirtVariant::Dirt3 => tileset.dirt3,
            DirtVariant::Dirt4 => tileset.dirt4,
            DirtVariant::Dirt5 => tileset.dirt5,
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
    Dirt1,
    Dirt2,
    Dirt3, // TODO: rename Dirt3 to FlatDirt
    Dirt4,
    Dirt5,
}

impl Distribution<DirtVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DirtVariant {
        if rng.gen_bool(0.9) {
            DirtVariant::Dirt3
        } else {
            match rng.gen_range(0..4) {
                0 => DirtVariant::Dirt1,
                1 => DirtVariant::Dirt2,
                2 => DirtVariant::Dirt4,
                3 => DirtVariant::Dirt5,
                _ => unreachable!(),
            }
        }
    }
}
