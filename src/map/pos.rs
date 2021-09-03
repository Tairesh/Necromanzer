use direction::Direction;
use map::chunk::Chunk;

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ChunkPos {
    x: i32,
    y: i32,
}

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> Self {
        ChunkPos { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
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
}
