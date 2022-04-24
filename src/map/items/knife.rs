use assets::tileset::Tileset;
use map::item::{ItemInteract, ItemTag, ItemView};
use std::collections::HashSet;
use tetra::graphics::Rectangle;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Knife {}

impl Knife {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Knife {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemView for Knife {
    fn name(&self) -> String {
        "knife".to_string()
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        tileset.knife
    }
}

impl ItemInteract for Knife {
    fn tags(&self) -> HashSet<ItemTag> {
        HashSet::from([ItemTag::Butch])
    }

    fn mass(&self) -> u32 {
        500
    }
}
