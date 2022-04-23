use assets::tileset::Tileset;
use map::item::{ItemInteract, ItemTag, ItemView};
use std::collections::HashSet;
use tetra::graphics::Rectangle;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Shovel {}

impl Shovel {
    pub fn new() -> Self {
        Self {}
    }
}

impl ItemView for Shovel {
    fn name(&self) -> String {
        "shovel".to_string()
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        tileset.shovel
    }
}

impl ItemInteract for Shovel {
    fn tags(&self) -> HashSet<ItemTag> {
        HashSet::from([ItemTag::Dig])
    }

    fn mass(&self) -> u32 {
        2_000 // 2 kg (probably it's a very small shovel)
    }
}
