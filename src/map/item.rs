use assets::TilesetRegions;
use avatar::Avatar;
use human::character::Character;
use human::main_hand::MainHand;
use tetra::graphics::Rectangle;

// TODO: ItemTypes should be loaded from jsons for modding
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ItemType {
    Shovel,
    Knife,
    Axe,
    Corpse(Character),
    GraveStone(Character),
}

impl ItemType {
    pub fn region(&self, regions: &TilesetRegions) -> Rectangle {
        match self {
            ItemType::Shovel => regions.shovel,
            ItemType::Knife => regions.knife,
            ItemType::Axe => regions.axe,
            ItemType::Corpse(..) => regions.corpse,
            ItemType::GraveStone(..) => regions.grave_stone,
        }
    }

    pub fn name(&self) -> String {
        match self {
            ItemType::Shovel => "shovel".to_string(),
            ItemType::Knife => "knife".to_string(),
            ItemType::Axe => "axe".to_string(),
            ItemType::Corpse(c) => format!("corpse of {}", c.name),
            ItemType::GraveStone(_) => "gravestone".to_string(),
        }
    }

    pub fn wield_time(&self, avatar: &Avatar) -> f64 {
        let k = match avatar.character.main_hand {
            MainHand::Left => 1.1,
            MainHand::Right | MainHand::Ambidexter => 1.0,
        };
        k * match self {
            ItemType::Shovel => 30.0,
            ItemType::Knife => 20.0,
            ItemType::Axe => 25.0,
            ItemType::Corpse(c) => {
                if c.age < 16 {
                    50.0
                } else {
                    100.0
                }
            }
            ItemType::GraveStone(_) => 200.0,
        }
    }

    pub fn drop_time(&self) -> f64 {
        10.0
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Item {
    pub item_type: ItemType,
}

impl Item {
    pub fn new(item_type: ItemType) -> Self {
        Self { item_type }
    }
}
