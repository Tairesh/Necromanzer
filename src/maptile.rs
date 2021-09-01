use chunk::{Chunk, ChunkPos};
use rand::distributions::Distribution;
use rand::Rng;

#[derive(Debug)]
pub enum DirtVariant {
    Dirt1,
    Dirt2,
    Dirt3,
    Dirt4,
    Dirt5,
}

pub struct DirtDistribution {}
impl Distribution<DirtVariant> for DirtDistribution {
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

#[derive(Debug)]
pub enum BoulderVariant {
    One1,
    One2,
    One3,
    Two1,
    Two2,
    Three1,
    Three2,
}

pub struct BoulderDistribution {}
impl Distribution<BoulderVariant> for BoulderDistribution {
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

#[derive(Debug)]
pub enum TileBase {
    Dirt(DirtVariant),
    Boulder(BoulderVariant),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TilePos {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
impl TilePos {
    pub fn new(x: i32, y: i32) -> Self {
        TilePos { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
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

    pub fn add(&self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    pub fn translate(&mut self, dx: i32, dy: i32) -> &Self {
        self.x += dx;
        self.y += dy;
        self
    }

    pub fn diff(&self, other: TilePos) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
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
