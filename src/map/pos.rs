#![allow(dead_code)]
use geometry::direction::Direction;
use geometry::point::Point;
use geometry::Vec2;
use map::chunk::Chunk;
use std::ops::{Add, AddAssign, Sub};

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl TilePos {
    pub fn new(x: i32, y: i32) -> Self {
        TilePos { x, y }
    }

    // TODO: probably use From trait?
    pub fn from(chunk: ChunkPos, pos: usize) -> Self {
        let left_top = chunk.left_top();
        let dx = pos as i32 / Chunk::SIZE;
        let dy = pos as i32 % Chunk::SIZE;
        TilePos::new(left_top.x + dx, left_top.y + dy)
    }

    pub fn chunk_and_pos(&self) -> (ChunkPos, usize) {
        let chunk = ChunkPos::new(
            (self.x as f32 / Chunk::SIZE as f32).floor() as i32,
            (self.y as f32 / Chunk::SIZE as f32).floor() as i32,
        );
        let left_top = chunk.left_top();
        let pos = ((self.x - left_top.x) * Chunk::SIZE + self.y - left_top.y) as usize;
        (chunk, pos)
    }

    pub fn square_dist_to(&self, other: TilePos) -> u32 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx * dx + dy * dy
    }
}

impl Add<Direction> for TilePos {
    type Output = TilePos;

    fn add(self, dir: Direction) -> Self::Output {
        Self::new(self.x + dir.dx() as i32, self.y + dir.dy() as i32)
    }
}

impl Add<&Direction> for TilePos {
    type Output = TilePos;

    fn add(self, dir: &Direction) -> Self::Output {
        Self::new(self.x + dir.dx() as i32, self.y + dir.dy() as i32)
    }
}

impl AddAssign<Direction> for TilePos {
    fn add_assign(&mut self, dir: Direction) {
        self.x += dir.dx();
        self.y += dir.dy();
    }
}

impl AddAssign<&Direction> for TilePos {
    fn add_assign(&mut self, dir: &Direction) {
        self.x += dir.dx();
        self.y += dir.dy();
    }
}

impl Add<(i32, i32)> for TilePos {
    type Output = TilePos;

    fn add(self, (dx, dy): (i32, i32)) -> Self::Output {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl Add<&(i32, i32)> for TilePos {
    type Output = TilePos;

    fn add(self, (dx, dy): &(i32, i32)) -> Self::Output {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl Sub<TilePos> for TilePos {
    type Output = Vec2;

    fn sub(self, rhs: TilePos) -> Self::Output {
        Vec2::new((rhs.x - self.x) as f32, (rhs.y - self.y) as f32)
    }
}

impl Add<Point> for TilePos {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Point> for TilePos {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> Self {
        ChunkPos { x, y }
    }

    pub fn left_top(&self) -> TilePos {
        TilePos::new(self.x * Chunk::SIZE, self.y * Chunk::SIZE)
    }

    #[allow(dead_code)]
    pub fn right_bottom(&self) -> TilePos {
        TilePos::new(
            self.x * Chunk::SIZE + Chunk::SIZE - 1,
            self.y * Chunk::SIZE + Chunk::SIZE - 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use map::chunk::Chunk;
    use map::pos::{ChunkPos, TilePos};

    #[test]
    fn test_pos_to_chunk() {
        let (chunk, pos) = TilePos::new(0, 0).chunk_and_pos();
        assert_eq!(chunk, ChunkPos::new(0, 0));
        assert_eq!(pos, 0);
        let (chunk, pos) = TilePos::new(4, 2).chunk_and_pos();
        assert_eq!(chunk, ChunkPos::new(0, 0));
        assert_eq!(pos, 4 * Chunk::SIZE as usize + 2);
        let (chunk, pos) = TilePos::new(-1, -1).chunk_and_pos();
        assert_eq!(chunk, ChunkPos::new(-1, -1));
        assert_eq!(pos, Chunk::USIZE - 1);
    }

    #[test]
    fn test_tile_from_chunk() {
        let chunk = ChunkPos::new(0, 0);
        assert_eq!(TilePos::from(chunk, 0), TilePos::new(0, 0));
        assert_eq!(
            TilePos::from(chunk, Chunk::USIZE - 1),
            TilePos::new(Chunk::SIZE - 1, Chunk::SIZE - 1)
        );
        assert_eq!(
            TilePos::from(chunk, Chunk::SIZE as usize - 1),
            TilePos::new(0, Chunk::SIZE - 1)
        );
        assert_eq!(
            TilePos::from(chunk, (Chunk::SIZE * (Chunk::SIZE - 1)) as usize),
            TilePos::new(Chunk::SIZE - 1, 0)
        );
        let chunk = ChunkPos::new(-1, -1);
        assert_eq!(
            TilePos::from(chunk, 0),
            TilePos::new(-Chunk::SIZE, -Chunk::SIZE)
        );
        assert_eq!(TilePos::from(chunk, Chunk::USIZE - 1), TilePos::new(-1, -1));
        assert_eq!(
            TilePos::from(chunk, Chunk::SIZE as usize - 1),
            TilePos::new(-Chunk::SIZE, -1)
        );
        assert_eq!(
            TilePos::from(chunk, (Chunk::SIZE * (Chunk::SIZE - 1)) as usize),
            TilePos::new(-1, -Chunk::SIZE)
        );
    }

    #[test]
    fn test_chunk_to_tile() {
        let chunk = ChunkPos::new(0, 0);
        assert_eq!(chunk.left_top(), TilePos::new(0, 0));
        assert_eq!(
            chunk.right_bottom(),
            TilePos::new(Chunk::SIZE - 1, Chunk::SIZE - 1)
        );
        let chunk = ChunkPos::new(-1, -1);
        assert_eq!(chunk.left_top(), TilePos::new(-Chunk::SIZE, -Chunk::SIZE));
        assert_eq!(chunk.right_bottom(), TilePos::new(-1, -1));
    }
}
