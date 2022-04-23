use assets::tileset::Tileset;
use map::item::Item;
use map::passage::Passage;
use map::terrain::{Terrain, TerrainInteract, TerrainView};
use map::terrains_impl::pit::Pit;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use tetra::graphics::Rectangle;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Grass {
    #[serde(rename = "v")]
    variant: GrassVariant,
}

impl Grass {
    pub fn new(variant: GrassVariant) -> Self {
        Self { variant }
    }

    pub fn dead(&self) -> bool {
        matches!(
            self.variant,
            GrassVariant::DeadGrass1
                | GrassVariant::DeadGrass2
                | GrassVariant::DeadGrass3
                | GrassVariant::DeadGrass4
                | GrassVariant::DeadGrass5
                | GrassVariant::DeadGrass6
                | GrassVariant::DeadGrass7
                | GrassVariant::DeadGrass8
                | GrassVariant::DeadGrass9
                | GrassVariant::DeadGrass10
                | GrassVariant::DeadGrass11
                | GrassVariant::DeadGrass12
                | GrassVariant::DeadGrass13
                | GrassVariant::DeadGrass14
        )
    }

    pub fn die(&mut self) {
        self.variant = match self.variant {
            GrassVariant::Grass1 => GrassVariant::DeadGrass1,
            GrassVariant::Grass2 => GrassVariant::DeadGrass2,
            GrassVariant::Grass3 => GrassVariant::DeadGrass3,
            GrassVariant::Grass4 => GrassVariant::DeadGrass4,
            GrassVariant::Grass5 => GrassVariant::DeadGrass5,
            GrassVariant::Grass6 => GrassVariant::DeadGrass6,
            GrassVariant::Grass7 => GrassVariant::DeadGrass7,
            GrassVariant::Grass8 => GrassVariant::DeadGrass8,
            GrassVariant::Grass9 => GrassVariant::DeadGrass9,
            GrassVariant::Grass10 => GrassVariant::DeadGrass10,
            GrassVariant::Grass11 => GrassVariant::DeadGrass11,
            GrassVariant::Grass12 => GrassVariant::DeadGrass12,
            GrassVariant::Grass13 => GrassVariant::DeadGrass13,
            GrassVariant::Grass14 => GrassVariant::DeadGrass14,
            _ => self.variant,
        }
    }
}

impl TerrainView for Grass {
    fn name(&self) -> &str {
        if self.dead() {
            "dead grass"
        } else {
            "grass"
        }
    }

    fn region(&self, tileset: &Tileset) -> Rectangle {
        match self.variant {
            GrassVariant::Grass1 => tileset.grass1,
            GrassVariant::Grass2 => tileset.grass2,
            GrassVariant::Grass3 => tileset.grass3,
            GrassVariant::Grass4 => tileset.grass4,
            GrassVariant::Grass5 => tileset.grass5,
            GrassVariant::Grass6 => tileset.grass6,
            GrassVariant::Grass7 => tileset.grass7,
            GrassVariant::Grass8 => tileset.grass8,
            GrassVariant::Grass9 => tileset.grass9,
            GrassVariant::Grass10 => tileset.grass10,
            GrassVariant::Grass11 => tileset.grass11,
            GrassVariant::Grass12 => tileset.grass12,
            GrassVariant::Grass13 => tileset.grass13,
            GrassVariant::Grass14 => tileset.grass14,
            GrassVariant::DeadGrass1 => tileset.dead_grass1,
            GrassVariant::DeadGrass2 => tileset.dead_grass2,
            GrassVariant::DeadGrass3 => tileset.dead_grass3,
            GrassVariant::DeadGrass4 => tileset.dead_grass4,
            GrassVariant::DeadGrass5 => tileset.dead_grass5,
            GrassVariant::DeadGrass6 => tileset.dead_grass6,
            GrassVariant::DeadGrass7 => tileset.dead_grass7,
            GrassVariant::DeadGrass8 => tileset.dead_grass8,
            GrassVariant::DeadGrass9 => tileset.dead_grass9,
            GrassVariant::DeadGrass10 => tileset.dead_grass10,
            GrassVariant::DeadGrass11 => tileset.dead_grass11,
            GrassVariant::DeadGrass12 => tileset.dead_grass12,
            GrassVariant::DeadGrass13 => tileset.dead_grass13,
            GrassVariant::DeadGrass14 => tileset.dead_grass14,
        }
    }

    fn is_transparent(&self) -> bool {
        true
    }
}

impl TerrainInteract for Grass {
    fn passage(&self) -> Passage {
        Passage::Passable(11.0)
    }

    fn is_diggable(&self) -> bool {
        true
    }

    fn dig_result(&self) -> (Terrain, Vec<Item>) {
        (Pit::new().into(), vec![])
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum GrassVariant {
    Grass1,
    Grass2,
    Grass3,
    Grass4,
    Grass5,
    Grass6,
    Grass7,
    Grass8,
    Grass9,
    Grass10,
    Grass11,
    Grass12,
    Grass13,
    Grass14,
    DeadGrass1,
    DeadGrass2,
    DeadGrass3,
    DeadGrass4,
    DeadGrass5,
    DeadGrass6,
    DeadGrass7,
    DeadGrass8,
    DeadGrass9,
    DeadGrass10,
    DeadGrass11,
    DeadGrass12,
    DeadGrass13,
    DeadGrass14,
}

impl Distribution<GrassVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GrassVariant {
        if rng.gen_bool(0.9) {
            match rng.gen_range(0..14) {
                0 => GrassVariant::Grass1,
                1 => GrassVariant::Grass2,
                2 => GrassVariant::Grass3,
                3 => GrassVariant::Grass4,
                4 => GrassVariant::Grass5,
                5 => GrassVariant::Grass6,
                6 => GrassVariant::Grass7,
                7 => GrassVariant::Grass8,
                8 => GrassVariant::Grass9,
                9 => GrassVariant::Grass10,
                10 => GrassVariant::Grass11,
                11 => GrassVariant::Grass12,
                12 => GrassVariant::Grass13,
                13 => GrassVariant::Grass14,
                _ => unreachable!(),
            }
        } else {
            match rng.gen_range(0..14) {
                0 => GrassVariant::DeadGrass1,
                1 => GrassVariant::DeadGrass2,
                2 => GrassVariant::DeadGrass3,
                3 => GrassVariant::DeadGrass4,
                4 => GrassVariant::DeadGrass5,
                5 => GrassVariant::DeadGrass6,
                6 => GrassVariant::DeadGrass7,
                7 => GrassVariant::DeadGrass8,
                8 => GrassVariant::DeadGrass9,
                9 => GrassVariant::DeadGrass10,
                10 => GrassVariant::DeadGrass11,
                11 => GrassVariant::DeadGrass12,
                12 => GrassVariant::DeadGrass13,
                13 => GrassVariant::DeadGrass14,
                _ => unreachable!(),
            }
        }
    }
}
