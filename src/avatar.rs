use human::character::Character;
use maptile::TilePos;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub character: Character,
    pub pos: TilePos,
}

impl Avatar {
    pub fn new(character: Character, pos: TilePos) -> Self {
        Avatar { character, pos }
    }
}
