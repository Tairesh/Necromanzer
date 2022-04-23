use assets::tileset::Tileset;
use map::item::{ItemInteract, ItemView};
use tetra::graphics::Rectangle;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Cloak {}

impl Cloak {
    pub fn new() -> Self {
        Self {}
    }
}

impl ItemView for Cloak {
    fn name(&self) -> String {
        "cloak".to_string()
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        tileset.cloak
    }
}

impl ItemInteract for Cloak {
    fn mass(&self) -> u32 {
        300
    }

    fn is_wearable(&self) -> bool {
        true
    }
}
