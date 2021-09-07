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
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Item {
    item_type: ItemType,
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
