use map::item::{ItemInteract, ItemTag, ItemView};
use std::collections::HashSet;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Axe {}

impl Axe {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Axe {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemView for Axe {
    fn name(&self) -> String {
        "axe".to_string()
    }

    fn looks_like(&self) -> &'static str {
        "axe"
    }
}

impl ItemInteract for Axe {
    fn tags(&self) -> HashSet<ItemTag> {
        HashSet::from([ItemTag::Butch])
    }

    fn mass(&self) -> u32 {
        1_000 // 1kg axe
    }
}
