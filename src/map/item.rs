use avatar::Avatar;
use human::main_hand::MainHand;
use tetra::graphics::Rectangle;

// TODO: ItemTypes should be loaded from jsons for modding
#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ItemType {
    Shovel,
    Knife,
    Axe,
}

impl ItemType {
    pub fn region(&self) -> Rectangle {
        // TODO: should be a better way to define it
        match self {
            ItemType::Shovel => Rectangle::new(0.0, 70.0, 10.0, 10.0),
            ItemType::Knife => Rectangle::new(10.0, 70.0, 10.0, 10.0),
            ItemType::Axe => Rectangle::new(20.0, 70.0, 10.0, 10.0),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ItemType::Shovel => "shovel",
            ItemType::Knife => "knife",
            ItemType::Axe => "axe",
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

    pub fn region(&self) -> Rectangle {
        self.item_type.region()
    }

    pub fn name(&self) -> &str {
        self.item_type.name()
    }
}
