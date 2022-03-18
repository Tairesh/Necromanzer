use assets::Tileset;
use human::character::Character;
use map::Passage;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use tetra::graphics::Rectangle;

// TODO: Create structs TerrainType and load them from jsons
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Terrain {
    Dirt(DirtVariant),
    Boulder(BoulderVariant),
    Grave(GraveVariant, GraveData),
    Grass(GrassVariant),
    DeadGrass(DeadGrassVariant),
    Pit,
}

impl Terrain {
    pub fn name(&self) -> &str {
        match self {
            Terrain::Dirt(_) => "dirt",
            Terrain::Boulder(var) => match var {
                BoulderVariant::Huge => "huge boulder",
                BoulderVariant::Middle => "boulder",
                BoulderVariant::Small => "small boulder",
            },
            Terrain::Grave(_, _) => "grave",
            Terrain::Grass(_) => "grass",
            Terrain::DeadGrass(_) => "dead grass",
            Terrain::Pit => "pit",
        }
    }

    pub fn this_is(&self) -> String {
        match self {
            Terrain::Grave(_, data) => {
                format!(
                    "This is the grave of {}. {} died in {} at the age of {}.",
                    data.character.name,
                    data.character.gender.pronounce().0,
                    data.death_year,
                    data.character.age,
                )
            }
            _ => format!("That is a {}.", self.name()),
        }
    }

    pub fn region(&self, tileset: &Tileset) -> Rectangle {
        match self {
            Terrain::Dirt(variant) => match variant {
                DirtVariant::Dirt1 => tileset.dirt1,
                DirtVariant::Dirt2 => tileset.dirt2,
                DirtVariant::Dirt3 => tileset.dirt3,
                DirtVariant::Dirt4 => tileset.dirt4,
                DirtVariant::Dirt5 => tileset.dirt5,
            },
            Terrain::Boulder(variant) => match variant {
                BoulderVariant::Huge => tileset.boulder_huge,
                BoulderVariant::Middle => tileset.boulder_middle,
                BoulderVariant::Small => tileset.boulder_small,
            },
            Terrain::Grave(variant, _) => match variant {
                GraveVariant::New => tileset.grave_new,
                GraveVariant::Old => tileset.grave_old,
            },
            Terrain::Grass(variant) => match variant {
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
            },
            Terrain::DeadGrass(variant) => match variant {
                DeadGrassVariant::DeadGrass1 => tileset.dead_grass1,
                DeadGrassVariant::DeadGrass2 => tileset.dead_grass2,
                DeadGrassVariant::DeadGrass3 => tileset.dead_grass3,
                DeadGrassVariant::DeadGrass4 => tileset.dead_grass4,
                DeadGrassVariant::DeadGrass5 => tileset.dead_grass5,
                DeadGrassVariant::DeadGrass6 => tileset.dead_grass6,
                DeadGrassVariant::DeadGrass7 => tileset.dead_grass7,
                DeadGrassVariant::DeadGrass8 => tileset.dead_grass8,
                DeadGrassVariant::DeadGrass9 => tileset.dead_grass9,
                DeadGrassVariant::DeadGrass10 => tileset.dead_grass10,
                DeadGrassVariant::DeadGrass11 => tileset.dead_grass11,
                DeadGrassVariant::DeadGrass12 => tileset.dead_grass12,
                DeadGrassVariant::DeadGrass13 => tileset.dead_grass13,
                DeadGrassVariant::DeadGrass14 => tileset.dead_grass14,
            },
            Terrain::Pit => tileset.pit,
        }
    }

    pub fn is_walkable(&self) -> bool {
        match self.pass() {
            Passage::Passable(_) => true,
            Passage::Unpassable => false,
        }
    }

    pub fn pass(&self) -> Passage {
        match self {
            Terrain::Grave(_, _) | Terrain::Pit => Passage::Unpassable,
            Terrain::Boulder(variant) => match variant {
                BoulderVariant::Small => Passage::Passable(30.0),
                BoulderVariant::Middle | BoulderVariant::Huge => Passage::Unpassable,
            },
            Terrain::Dirt(_) => Passage::Passable(10.0),
            Terrain::Grass(_) | Terrain::DeadGrass(_) => Passage::Passable(11.0),
        }
    }

    pub fn is_diggable(&self) -> bool {
        match self {
            Terrain::Dirt(_) | Terrain::Grave(_, _) | Terrain::Grass(_) | Terrain::DeadGrass(_) => {
                true
            }
            _ => false,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum BoulderVariant {
    Huge,
    Middle,
    Small,
}

impl Distribution<BoulderVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BoulderVariant {
        match rng.gen_range(0..3) {
            0 => BoulderVariant::Huge,
            1 => BoulderVariant::Middle,
            2 => BoulderVariant::Small,
            _ => unreachable!(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum GraveVariant {
    New,
    Old,
}

impl Distribution<GraveVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GraveVariant {
        if rng.gen_bool(0.9) {
            GraveVariant::Old
        } else {
            GraveVariant::New
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GraveData {
    pub character: Character,
    pub death_year: u8,
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
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
