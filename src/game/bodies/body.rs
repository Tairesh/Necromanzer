use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use game::map::item::Item;
use game::map::items::BodyPart;
use game::map::pos::TilePos;

pub type BodyPartsCollections = HashMap<TilePos, BodyPart>;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body {
    #[serde(rename = "p")]
    #[serde_as(as = "Vec<(_, _)>")]
    pub parts: BodyPartsCollections,
    #[serde(rename = "w")]
    pub wear: Vec<Item>,
}

impl Body {
    pub fn new(parts: BodyPartsCollections) -> Self {
        Self {
            parts,
            wear: Vec::default(),
        }
    }
}
