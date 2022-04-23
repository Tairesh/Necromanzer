use assets::tileset::Tileset;
use map::item::{ItemInteract, ItemView};
use tetra::graphics::Rectangle;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Hat {}

impl Hat {
    pub fn new() -> Self {
        Self {}
    }
}

impl ItemView for Hat {
    fn name(&self) -> String {
        "hat".to_string()
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        tileset.hat
    }
}

impl ItemInteract for Hat {
    fn mass(&self) -> u32 {
        100
    }

    fn is_wearable(&self) -> bool {
        true
    }
}
