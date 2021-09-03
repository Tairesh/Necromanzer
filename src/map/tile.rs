use map::terrains::Terrain;

#[derive(Debug)]
pub struct Tile {
    pub terrain: Terrain,
}

impl Tile {
    pub fn new(terrain: Terrain) -> Self {
        Self { terrain }
    }
}
