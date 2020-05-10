use crate::block::Block;
use crate::chunk::ChunkGridCoordinate;
use crate::chunk::{Chunk, CHUNK_DEPTH, CHUNK_WIDTH};
use crate::world::generation::HeightMap;

pub struct WorldGenerator {
    height_map: HeightMap,
}

impl WorldGenerator {
    pub fn generate_chunk(&self, coords: ChunkGridCoordinate, chunk: &mut Chunk) {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                let absx = x as i64 + coords.x * CHUNK_WIDTH as i64;
                let absz = z as i64 + coords.z * CHUNK_DEPTH as i64;

                let height = self.height_map.get_height(absx, absz) as usize;

                for y in 0..5 {
                    chunk.blocks[x][y][z] = Block { id: 7 };
                }

                for y in 5..10 {
                    chunk.blocks[x][y][z] = Block { id: 4 };
                }

                for y in 10..(height - 3) {
                    chunk.blocks[x][y][z] = Block { id: 1 };
                }

                for y in (height - 3)..height {
                    let id = if height < 59 { 12 } else { 3 };
                    chunk.blocks[x][y][z] = Block { id };
                }

                let id = if height < 59 { 12 } else { 2 };
                chunk.blocks[x][height][z] = Block { id };

                if height < 58 {
                    for y in height..59 {
                        chunk.blocks[x][y][z] = Block { id: 9 };
                    }
                }
            }
        }
    }
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self {
            height_map: HeightMap::new(50..75, 12923874),
        }
    }
}
