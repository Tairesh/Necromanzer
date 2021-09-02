use chunk::{Chunk, ChunkPos};
use direction::Direction;
use human::character::Character;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

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
pub enum Terrain {
    Dirt(DirtVariant),
    Boulder(BoulderVariant),
    Grave(GraveVariant, GraveData),
}

#[derive(Debug)]
pub struct Tile {
    pub terrain: Terrain,
}

impl Tile {
    pub fn new(terrain: Terrain) -> Self {
        Self { terrain }
    }

    pub fn this_is(&self) -> &str {
        match self.terrain {
            Terrain::Dirt(_) => "dirt",
            Terrain::Boulder(_) => "boulder",
            Terrain::Grave(_, _) => "grave",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TilePos {
    x: i32,
    y: i32,
}

impl TilePos {
    pub fn new(x: i32, y: i32) -> Self {
        TilePos { x, y }
    }

    pub fn chunk_and_pos(&self) -> (ChunkPos, usize) {
        let chunk = ChunkPos::new(
            (self.x as f32 / Chunk::SIZE as f32).floor() as i32,
            (self.y as f32 / Chunk::SIZE as f32).floor() as i32,
        );
        let topleft = (chunk.x() * Chunk::SIZE, chunk.y() * Chunk::SIZE);
        let pos = ((self.x - topleft.0) * Chunk::SIZE + self.y - topleft.1) as usize;
        (chunk, pos)
    }

    pub fn add(&self, dir: Direction) -> Self {
        Self::new(self.x + dir.dx() as i32, self.y + dir.dy() as i32)
    }

    pub fn add_delta(&self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}

#[cfg(test)]
mod tests {
    use chunk::ChunkPos;
    use maptile::TilePos;

    #[test]
    fn test_pos_to_chunk() {
        let (chunk, pos) = TilePos::new(0, 0).chunk_and_pos();
        assert_eq!(chunk, ChunkPos::new(0, 0));
        assert_eq!(pos, 0);
        let (chunk, pos) = TilePos::new(4, 2).chunk_and_pos();
        assert_eq!(chunk, ChunkPos::new(0, 0));
        assert_eq!(pos, 4 * 32 + 2);
        let (chunk, pos) = TilePos::new(-1, -1).chunk_and_pos();
        assert_eq!(chunk, ChunkPos::new(-1, -1));
        assert_eq!(pos, 1023);
    }
}
