use assets::tileset::Tileset;
use map::item::{ItemInteract, ItemView};
use map::terrains::GraveData;
use tetra::graphics::Rectangle;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Gravestone {
    pub data: GraveData,
}

impl Gravestone {
    pub fn new(data: GraveData) -> Self {
        Self { data }
    }
}

impl ItemView for Gravestone {
    fn name(&self) -> String {
        "gravestone".to_string()
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        tileset.grave_stone
    }
}

impl ItemInteract for Gravestone {
    fn mass(&self) -> u32 {
        200_000
    }

    fn is_readable(&self) -> bool {
        true
    }

    fn read(&self) -> String {
        self.data.read()
    }
}
