#![allow(dead_code)]

use map::item::Item;
use map::terrains::{DeadGrassVariant, DirtVariant, GrassVariant, Terrain};
use rand::Rng;
use std::collections::HashSet;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Tile {
    #[serde(rename = "t")]
    pub terrain: Terrain,
    #[serde(rename = "i")]
    pub items: Vec<Item>,
    #[serde(default)]
    #[serde(rename = "u")]
    pub units: HashSet<usize>,
}

impl Tile {
    pub fn new(terrain: Terrain) -> Self {
        Self {
            terrain,
            items: Vec::new(),
            units: HashSet::new(),
        }
    }

    /// Calls when avatar leaves tile
    pub fn off_step(&mut self, unit_id: usize) {
        self.units.remove(&unit_id);
    }

    /// Calls when avatar walks on tile
    pub fn on_step(&mut self, unit_id: usize) {
        self.units.insert(unit_id);
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

    pub fn top_item(&self) -> Option<&Item> {
        self.items.last()
    }

    pub fn dig(&mut self) -> Vec<Item> {
        self.terrain.dig()
    }

    pub fn is_readable(&self) -> bool {
        if self.terrain.is_readable() {
            return true;
        }

        self.items.iter().any(|i| i.is_readable())
    }

    pub fn read(&self) -> String {
        // TODO: probably we shouldn't read only first occurency
        if self.terrain.is_readable() {
            return self.terrain.read();
        }

        self.items
            .iter()
            .rev()
            .filter(|i| i.is_readable())
            .map(|i| i.read())
            .next()
            .unwrap_or_else(|| "You can't find anything to read here.".to_string())
    }
}
