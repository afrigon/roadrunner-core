use crate::chunk::ChunkGridCoordinate;

use std::num::Wrapping;
use math::random::{Prng, Seed};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WorldSeed(pub Seed);

impl WorldSeed {
    pub fn new() -> Self {
        let mut prng = Prng::new(Seed::new());
        Self(Seed(prng.next_u32()))
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
