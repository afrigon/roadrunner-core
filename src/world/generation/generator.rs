use crate::block::Block;
use crate::chunk::ChunkGridCoordinate;
use crate::chunk::{Chunk, CHUNK_DEPTH, CHUNK_WIDTH};
use crate::world::generation::height_map::HeightMap;
use std::time::{Duration, Instant};

pub struct WorldGenerator {
    height_map: HeightMap,
}

pub fn generate_chunk(coords: ChunkGridCoordinate) -> Chunk {
    let wg = WorldGenerator::default();
    wg.generate_chunk(coords)
}

impl WorldGenerator {
    pub fn generate_chunk(&self, coords: ChunkGridCoordinate) -> Chunk {
        let start = Instant::now();
        let mut spent_in_height = Duration::new(0, 0);
        let mut chunk = Chunk::new(coords);
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                let absx = x as i64 + coords.x * CHUNK_WIDTH as i64;
                let absz = z as i64 + coords.z * CHUNK_DEPTH as i64;

                let now = Instant::now();
                let height = self.height_map.get_height(absx, absz) as usize;
                spent_in_height += now.elapsed();

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

        println!(
            "generated {:?} in {:?} (spent {:?} in get_height)",
            coords,
            start.elapsed(),
            spent_in_height
        );
        chunk
    }
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self {
            height_map: HeightMap::new(50..200, 12923874),
        }
    }
}
