use assets::tileset::Tileset;
use map::passage::Passage;
use map::terrain::{TerrainInteract, TerrainView};
use tetra::graphics::Rectangle;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Pit {}

impl Pit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Pit {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainView for Pit {
    fn name(&self) -> &str {
        "pit"
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        tileset.pit
    }

    fn is_transparent(&self) -> bool {
        true
    }
}

impl TerrainInteract for Pit {
    fn passage(&self) -> Passage {
        Passage::Unpassable
    }
}
