#![allow(dead_code)]

use std::fmt::{Display, Formatter};

use geometry::point::Point;

use super::chunk::Chunk;

pub type TilePos = Point;

impl TilePos {
    pub fn from_chunk(chunk: ChunkPos, pos: usize) -> Self {
        let left_top = chunk.left_top();
        let dx = pos as i32 / Chunk::SIZE;
        let dy = pos as i32 % Chunk::SIZE;
        TilePos::new(left_top.x + dx, left_top.y + dy)
    }

    pub fn to_chunk(self) -> (ChunkPos, usize) {
        let chunk = ChunkPos::new(
            (self.x as f32 / Chunk::SIZE as f32).floor() as i32,
            (self.y as f32 / Chunk::SIZE as f32).floor() as i32,
        );
        let left_top = chunk.left_top();
        let pos = ((self.x - left_top.x) * Chunk::SIZE + self.y - left_top.y) as usize;
        (chunk, pos)
    }
}

impl Display for TilePos {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
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

    pub fn left_top(self) -> TilePos {
        TilePos::new(self.x * Chunk::SIZE, self.y * Chunk::SIZE)
    }

    #[allow(dead_code)]
    pub fn right_bottom(self) -> TilePos {
        TilePos::new(
            self.x * Chunk::SIZE + Chunk::SIZE - 1,
            self.y * Chunk::SIZE + Chunk::SIZE - 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::chunk::Chunk;
    use super::{ChunkPos, TilePos};

    #[test]
    fn test_pos_to_chunk() {
        let (chunk, pos) = TilePos::new(0, 0).to_chunk();
        assert_eq!(chunk, ChunkPos::new(0, 0));
        assert_eq!(pos, 0);
        let (chunk, pos) = TilePos::new(4, 2).to_chunk();
        assert_eq!(chunk, ChunkPos::new(0, 0));
        assert_eq!(pos, 4 * Chunk::SIZE as usize + 2);
        let (chunk, pos) = TilePos::new(-1, -1).to_chunk();
        assert_eq!(chunk, ChunkPos::new(-1, -1));
        assert_eq!(pos, Chunk::USIZE - 1);
    }

    #[test]
    fn test_tile_from_chunk() {
        let chunk = ChunkPos::new(0, 0);
        assert_eq!(TilePos::from_chunk(chunk, 0), TilePos::new(0, 0));
        assert_eq!(
            TilePos::from_chunk(chunk, Chunk::USIZE - 1),
            TilePos::new(Chunk::SIZE - 1, Chunk::SIZE - 1)
        );
        assert_eq!(
            TilePos::from_chunk(chunk, Chunk::SIZE as usize - 1),
            TilePos::new(0, Chunk::SIZE - 1)
        );
        assert_eq!(
            TilePos::from_chunk(chunk, (Chunk::SIZE * (Chunk::SIZE - 1)) as usize),
            TilePos::new(Chunk::SIZE - 1, 0)
        );
        let chunk = ChunkPos::new(-1, -1);
        assert_eq!(
            TilePos::from_chunk(chunk, 0),
            TilePos::new(-Chunk::SIZE, -Chunk::SIZE)
        );
        assert_eq!(
            TilePos::from_chunk(chunk, Chunk::USIZE - 1),
            TilePos::new(-1, -1)
        );
        assert_eq!(
            TilePos::from_chunk(chunk, Chunk::SIZE as usize - 1),
            TilePos::new(-Chunk::SIZE, -1)
        );
        assert_eq!(
            TilePos::from_chunk(chunk, (Chunk::SIZE * (Chunk::SIZE - 1)) as usize),
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
