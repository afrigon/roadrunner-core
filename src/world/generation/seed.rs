use crate::chunk::ChunkGridCoordinate;

use math::random::Seed;
use std::num::Wrapping;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WorldSeed(pub Seed);

impl WorldSeed {
    pub fn new() -> Self {
        Self(Seed::new())
    }

    pub fn to_chunk_seed(&self, coords: ChunkGridCoordinate) -> ChunkSeed {
        ChunkSeed::new(&self.0, coords)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ChunkSeed(pub Seed);

impl ChunkSeed {
    pub fn new(seed: &Seed, coords: ChunkGridCoordinate) -> Self {
        let seed = Wrapping(seed.0);
        let x = Wrapping(coords.x);
        let z = Wrapping(coords.z);
        let delta = Wrapping(((x * z).0 & 0xffffffff) as u32);
        let value = (seed + delta).0;

        Self(Seed(value))
    }
}
