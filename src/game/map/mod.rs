use fov::FovMap;
use game::map::chunk::Chunk;
use game::map::pos::{ChunkPos, TilePos};
use game::map::terrain::TerrainView;
use game::map::tile::Tile;
use std::collections::{HashMap, HashSet};

pub mod chunk;
pub mod item;
pub mod items;
pub mod passage;
pub mod pos;
pub mod terrain;
pub mod terrains;
pub mod tile;

pub struct Map {
    pub seed: String,
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub changed: HashSet<ChunkPos>,
}

impl Map {
    pub fn get_chunk(&mut self, pos: ChunkPos) -> &Chunk {
        let seed = self.seed.clone();
        self.chunks
            .entry(pos)
            .or_insert_with_key(|pos| Chunk::generate(seed, *pos))
    }

    pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> &mut Chunk {
        let seed = self.seed.clone();
        self.changed.insert(pos);
        self.chunks
            .entry(pos)
            .or_insert_with_key(|pos| Chunk::generate(seed, *pos))
    }

    pub fn get_tile(&mut self, pos: TilePos) -> &Tile {
        let (chunk, pos) = pos.to_chunk();
        let chunk = self.get_chunk(chunk);
        &chunk.tiles[pos]
    }

    pub fn get_tile_mut(&mut self, pos: TilePos) -> &mut Tile {
        let (chunk, pos) = pos.to_chunk();
        let chunk = self.get_chunk_mut(chunk);
        &mut chunk.tiles[pos]
    }

    pub fn load_tiles_between(&mut self, left_top: TilePos, right_bottom: TilePos) {
        let (ChunkPos { x: lt_x, y: lt_y }, _) = left_top.to_chunk();
        let (ChunkPos { x: rb_x, y: rb_y }, _) = right_bottom.to_chunk();

        for x in lt_x..=rb_x {
            for y in lt_y..=rb_y {
                let pos = ChunkPos::new(x, y);
                self.get_chunk(pos);
            }
        }
    }

    pub fn tiles_between(&self, left_top: TilePos, right_bottom: TilePos) -> Vec<(TilePos, &Tile)> {
        let (ChunkPos { x: lt_x, y: lt_y }, _) = left_top.to_chunk();
        let (ChunkPos { x: rb_x, y: rb_y }, _) = right_bottom.to_chunk();

        let mut tiles =
            Vec::with_capacity(((rb_x - lt_x + 1) * (rb_y - lt_y + 1)) as usize * Chunk::USIZE);
        for x in lt_x..=rb_x {
            for y in lt_y..=rb_y {
                let chunk_pos = ChunkPos::new(x, y);
                let chunk = self.chunks.get(&chunk_pos).unwrap();
                for (i, tile) in chunk.tiles.iter().enumerate() {
                    tiles.push((TilePos::from_chunk(chunk_pos, i), tile));
                }
            }
        }
        tiles
    }
}

impl FovMap for Map {
    fn is_transparent(&self, pos: TilePos) -> bool {
        let (chunk, pos) = pos.to_chunk();
        self.chunks
            .get(&chunk)
            .map_or(true, |c| c.tiles[pos].terrain.is_transparent())
    }
}
