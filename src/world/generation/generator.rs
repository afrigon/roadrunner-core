use crate::block::Block;
use crate::chunk::ChunkGridCoordinate;
use crate::chunk::{Chunk, CHUNK_DEPTH, CHUNK_WIDTH};
use crate::utils::container::Area;
use crate::world::generation::HeightMap;
use crate::world::generation::WorldSeed;

use math::random::noise::{CombinedNoise, LayeredNoiseOptions};
use math::random::Prng;

pub fn generate_chunk(coords: ChunkGridCoordinate, seed: WorldSeed) -> Chunk {
    let generator = WorldGenerator::new(seed);
    generator.generate_chunk(coords)
}

const BASE_BLOCK: Block = Block { id: 7 };
const STONE_BLOCK: Block = Block { id: 1 };
const DIRT_BLOCK: Block = Block { id: 3 };
const GRASS_BLOCK: Block = Block { id: 2 };

const BASE_THICKNESS: usize = 5;
const BASE_FILL_DECREASE: f32 = 0.2;

const MIN_DIRT_THICKNESS: usize = 3;
const MAX_DIRT_THICKNESS: usize = 5;

pub struct WorldGenerator {
    seed: WorldSeed,
}

impl WorldGenerator {
    pub fn new(seed: WorldSeed) -> Self {
        Self { seed }
    }

    fn generate_base(&self, chunk: &mut Chunk, prng: &mut Prng) {
        let mut value = 1.0;
        for y in 0..BASE_THICKNESS {
            for x in 0..CHUNK_WIDTH {
                for z in 0..CHUNK_DEPTH {
                    if prng.next_f32() <= value {
                        chunk.set_block(x, y, z, BASE_BLOCK);
                    } else {
                        chunk.set_block(x, y, z, STONE_BLOCK);
                    }
                }
            }

            value -= BASE_FILL_DECREASE;
        }
    }

    fn generate_strata(&self, chunk: &mut Chunk, prng: &mut Prng, height_map: &HeightMap) {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                let height = height_map.height(x as u8, z as u8) as usize;

                let dirt_thickness = prng.next_in_range(MIN_DIRT_THICKNESS..MAX_DIRT_THICKNESS + 1);
                let dirt_height = height - dirt_thickness;

                for y in BASE_THICKNESS..dirt_height {
                    chunk.set_block(x, y, z, STONE_BLOCK);
                }

                for y in dirt_height..height {
                    chunk.set_block(x, y, z, DIRT_BLOCK);
                }

                chunk.set_block(x, height, z, GRASS_BLOCK);
            }
        }
    }

    pub fn generate_chunk(self, coords: ChunkGridCoordinate) -> Chunk {
        let chunk_seed = self.seed.to_chunk_seed(coords);
        let area = Area::new_chunk(coords);

        let mut prng = Prng::new(chunk_seed.0);
        let noise = CombinedNoise::new(
            LayeredNoiseOptions::new(4, 100.0, 0.50, 2.0, self.seed.0),
            LayeredNoiseOptions::new(6, 60.0, 0.50, 1.9, self.seed.0),
            10.0,
        );
        let height_map = HeightMap::new(area, 40..200, noise);

        let mut chunk = Chunk::new(coords);

        self.generate_base(&mut chunk, &mut prng);
        self.generate_strata(&mut chunk, &mut prng, &height_map);

        chunk
    }
}
