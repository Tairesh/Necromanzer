use map::terrains::{DeadGrassVariant, DirtVariant, Terrain};
use rand::Rng;

#[derive(Debug)]
pub struct Tile {
    pub terrain: Terrain,
}

impl Tile {
    pub fn new(terrain: Terrain) -> Self {
        Self { terrain }
    }

    /// Calls when avatar leaves tile
    pub fn off_step(&mut self) {
        if let Terrain::Grass(_) = self.terrain {
            if rand::thread_rng().gen_bool(0.3) {
                self.terrain = Terrain::DeadGrass(rand::random::<DeadGrassVariant>());
            }
        }
    }

    /// Calls when avatar walks on tile
    pub fn on_step(&mut self) {
        if let Terrain::Grass(_) | Terrain::DeadGrass(_) = self.terrain {
            if rand::thread_rng().gen_bool(0.1) {
                self.terrain = Terrain::Dirt(rand::random::<DirtVariant>());
            }
        }
    }
}
