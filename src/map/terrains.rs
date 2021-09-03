use assets::TilesetRegions;
use human::character::Character;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use tetra::graphics::Rectangle;

#[derive(Debug)]
pub enum Terrain {
    Dirt(DirtVariant),
    Boulder(BoulderVariant),
    Grave(GraveVariant, GraveData),
    Grass(GrassVariant),
    DeadGrass(DeadGrassVariant),
}

impl Terrain {
    pub fn this_is(&self) -> String {
        match self {
            Terrain::Dirt(_) => "This is dirt.".to_string(),
            Terrain::Boulder(var) => match var {
                BoulderVariant::One1 | BoulderVariant::One2 | BoulderVariant::One3 => {
                    "This is a boulder."
                }
                BoulderVariant::Two1 | BoulderVariant::Two2 => "This is a couple of boulders.",
                BoulderVariant::Three1 | BoulderVariant::Three2 => "This is a bunch of boulders.",
            }
            .to_string(),
            Terrain::Grave(_, data) => {
                format!(
                    "This is the grave of {}. {} died in {} at the age of {}.",
                    data.character.name,
                    data.character.gender.pronounce(),
                    data.death_year,
                    data.character.age,
                )
            }
            Terrain::Grass(_) => "This is grass.".to_string(),
            Terrain::DeadGrass(_) => "This is dead grass.".to_string(),
        }
    }

    pub fn region(&self, regions: &TilesetRegions) -> Rectangle {
        match self {
            Terrain::Dirt(variant) => match variant {
                DirtVariant::Dirt1 => regions.dirt1,
                DirtVariant::Dirt2 => regions.dirt2,
                DirtVariant::Dirt3 => regions.dirt3,
                DirtVariant::Dirt4 => regions.dirt4,
                DirtVariant::Dirt5 => regions.dirt5,
            },
            Terrain::Boulder(variant) => match variant {
                BoulderVariant::One1 => regions.boulder1,
                BoulderVariant::One2 => regions.boulder2,
                BoulderVariant::One3 => regions.boulder3,
                BoulderVariant::Two1 => regions.boulders1,
                BoulderVariant::Two2 => regions.boulders2,
                BoulderVariant::Three1 => regions.boulders3,
                BoulderVariant::Three2 => regions.boulders4,
            },
            Terrain::Grave(variant, _) => match variant {
                GraveVariant::Grave1 => regions.grave1,
                GraveVariant::Grave2 => regions.grave2,
                GraveVariant::Grave3 => regions.grave3,
                GraveVariant::Grave4 => regions.grave4,
            },
            Terrain::Grass(variant) => match variant {
                GrassVariant::Grass1 => regions.grass1,
                GrassVariant::Grass2 => regions.grass2,
                GrassVariant::Grass3 => regions.grass3,
                GrassVariant::Grass4 => regions.grass4,
                GrassVariant::Grass5 => regions.grass5,
                GrassVariant::Grass6 => regions.grass6,
                GrassVariant::Grass7 => regions.grass7,
                GrassVariant::Grass8 => regions.grass8,
                GrassVariant::Grass9 => regions.grass9,
                GrassVariant::Grass10 => regions.grass10,
                GrassVariant::Grass11 => regions.grass11,
                GrassVariant::Grass12 => regions.grass12,
                GrassVariant::Grass13 => regions.grass13,
                GrassVariant::Grass14 => regions.grass14,
            },
            Terrain::DeadGrass(variant) => match variant {
                DeadGrassVariant::DeadGrass1 => regions.dead_grass1,
                DeadGrassVariant::DeadGrass2 => regions.dead_grass2,
                DeadGrassVariant::DeadGrass3 => regions.dead_grass3,
                DeadGrassVariant::DeadGrass4 => regions.dead_grass4,
                DeadGrassVariant::DeadGrass5 => regions.dead_grass5,
                DeadGrassVariant::DeadGrass6 => regions.dead_grass6,
                DeadGrassVariant::DeadGrass7 => regions.dead_grass7,
                DeadGrassVariant::DeadGrass8 => regions.dead_grass8,
                DeadGrassVariant::DeadGrass9 => regions.dead_grass9,
                DeadGrassVariant::DeadGrass10 => regions.dead_grass10,
                DeadGrassVariant::DeadGrass11 => regions.dead_grass11,
                DeadGrassVariant::DeadGrass12 => regions.dead_grass12,
                DeadGrassVariant::DeadGrass13 => regions.dead_grass13,
                DeadGrassVariant::DeadGrass14 => regions.dead_grass14,
            },
        }
    }

    pub fn is_walkable(&self) -> bool {
        match self {
            Terrain::Grave(_, _) => false,
            Terrain::Dirt(_) | Terrain::Boulder(_) | Terrain::Grass(_) | Terrain::DeadGrass(_) => {
                true
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DirtVariant {
    Dirt1,
    Dirt2,
    Dirt3,
    Dirt4,
    Dirt5,
}

impl Distribution<DirtVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DirtVariant {
        if rng.gen_bool(0.9) {
            DirtVariant::Dirt3
        } else {
            match rng.gen_range(0..4) {
                0 => DirtVariant::Dirt1,
                1 => DirtVariant::Dirt2,
                2 => DirtVariant::Dirt4,
                3 => DirtVariant::Dirt5,
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BoulderVariant {
    One1,
    One2,
    One3,
    Two1,
    Two2,
    Three1,
    Three2,
}

impl Distribution<BoulderVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BoulderVariant {
        match rng.gen_range(0..7) {
            0 => BoulderVariant::One1,
            1 => BoulderVariant::One2,
            2 => BoulderVariant::One3,
            3 => BoulderVariant::Two1,
            4 => BoulderVariant::Two2,
            5 => BoulderVariant::Three1,
            6 => BoulderVariant::Three2,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum GraveVariant {
    Grave1,
    Grave2,
    Grave3,
    Grave4,
}

impl Distribution<GraveVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GraveVariant {
        match rng.gen_range(0..4) {
            0 => GraveVariant::Grave1,
            1 => GraveVariant::Grave2,
            2 => GraveVariant::Grave3,
            3 => GraveVariant::Grave4,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct GraveData {
    pub character: Character,
    pub death_year: u8,
}

#[derive(Debug)]
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
}

impl Distribution<GrassVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GrassVariant {
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
    }
}

#[derive(Debug)]
pub enum DeadGrassVariant {
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

impl Distribution<DeadGrassVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DeadGrassVariant {
        match rng.gen_range(0..14) {
            0 => DeadGrassVariant::DeadGrass1,
            1 => DeadGrassVariant::DeadGrass2,
            2 => DeadGrassVariant::DeadGrass3,
            3 => DeadGrassVariant::DeadGrass4,
            4 => DeadGrassVariant::DeadGrass5,
            5 => DeadGrassVariant::DeadGrass6,
            6 => DeadGrassVariant::DeadGrass7,
            7 => DeadGrassVariant::DeadGrass8,
            8 => DeadGrassVariant::DeadGrass9,
            9 => DeadGrassVariant::DeadGrass10,
            10 => DeadGrassVariant::DeadGrass11,
            11 => DeadGrassVariant::DeadGrass12,
            12 => DeadGrassVariant::DeadGrass13,
            13 => DeadGrassVariant::DeadGrass14,
            _ => unreachable!(),
        }
    }
}
