use map::terrains::{DeadGrassVariant, DirtVariant, GrassVariant, Terrain};
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Tile {
    pub terrain: Terrain,
}

impl Tile {
    pub fn new(terrain: Terrain) -> Self {
        Self { terrain }
    }

    /// Calls when avatar leaves tile
    pub fn off_step(&mut self) {}

    /// Calls when avatar walks on tile
    pub fn on_step(&mut self) {
        // TODO: (for future) footprints
        if rand::thread_rng().gen_bool(0.1) {
            match self.terrain {
                Terrain::Grass(_) | Terrain::DeadGrass(_) => {
                    self.terrain = Terrain::Dirt(rand::random::<DirtVariant>());
                }
                Terrain::Dirt(variant) => match variant {
                    DirtVariant::Dirt3 => {}
                    _ => {
                        self.terrain = Terrain::Dirt(DirtVariant::Dirt3);
                    }
                },
                _ => {}
            }
        }
    }

    pub fn kill_grass(&mut self) {
        if let Terrain::Grass(variant) = &self.terrain {
            self.terrain = Terrain::DeadGrass(match variant {
                GrassVariant::Grass1 => DeadGrassVariant::DeadGrass1,
                GrassVariant::Grass2 => DeadGrassVariant::DeadGrass2,
                GrassVariant::Grass3 => DeadGrassVariant::DeadGrass3,
                GrassVariant::Grass4 => DeadGrassVariant::DeadGrass4,
                GrassVariant::Grass5 => DeadGrassVariant::DeadGrass5,
                GrassVariant::Grass6 => DeadGrassVariant::DeadGrass6,
                GrassVariant::Grass7 => DeadGrassVariant::DeadGrass7,
                GrassVariant::Grass8 => DeadGrassVariant::DeadGrass8,
                GrassVariant::Grass9 => DeadGrassVariant::DeadGrass9,
                GrassVariant::Grass10 => DeadGrassVariant::DeadGrass10,
                GrassVariant::Grass11 => DeadGrassVariant::DeadGrass11,
                GrassVariant::Grass12 => DeadGrassVariant::DeadGrass12,
                GrassVariant::Grass13 => DeadGrassVariant::DeadGrass13,
                GrassVariant::Grass14 => DeadGrassVariant::DeadGrass14,
            });
        }
    }
}
