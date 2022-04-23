use assets::tileset::Tileset;
use map::passage::Passage;
use map::terrain::{TerrainInteract, TerrainView};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use tetra::graphics::Rectangle;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Boulder {
    #[serde(rename = "s")]
    size: BoulderSize,
    // TODO: hp, stone type
}

impl Boulder {
    pub fn new(size: BoulderSize) -> Self {
        Self { size }
    }
}

impl TerrainView for Boulder {
    fn name(&self) -> &str {
        match self.size {
            BoulderSize::Huge => "huge boulder",
            BoulderSize::Middle => "boulder",
            BoulderSize::Small => "small boulder",
        }
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        match self.size {
            BoulderSize::Huge => tileset.boulder_huge,
            BoulderSize::Middle => tileset.boulder_middle,
            BoulderSize::Small => tileset.boulder_small,
        }
    }

    fn is_transparent(&self) -> bool {
        !matches!(self.size, BoulderSize::Huge)
    }
}

impl TerrainInteract for Boulder {
    fn passage(&self) -> Passage {
        match self.size {
            BoulderSize::Huge | BoulderSize::Middle => Passage::Unpassable,
            BoulderSize::Small => Passage::Passable(50.0),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum BoulderSize {
    Huge,
    Middle,
    Small,
}

impl Distribution<BoulderSize> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BoulderSize {
        match rng.gen_range(0..3) {
            0 => BoulderSize::Huge,
            1 => BoulderSize::Middle,
            2 => BoulderSize::Small,
            _ => unreachable!(),
        }
    }
}
