use assets::TilesetRegions;
use tetra::graphics::Rectangle;

// TODO: ItemTypes should be loaded from jsons for modding
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ItemType {
    Shovel,
    Knife,
    Axe,
}

impl ItemType {
    pub fn region(&self, regions: &TilesetRegions) -> Rectangle {
        match self {
            ItemType::Shovel => regions.shovel,
            ItemType::Knife => regions.knife,
            ItemType::Axe => regions.axe,
        }
    }
}
