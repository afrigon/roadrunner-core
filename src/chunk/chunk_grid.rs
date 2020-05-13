use crate::chunk::Chunk;
use crate::world::WorldCoordinate;
use std::convert::TryInto;

use std::collections::HashMap;

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Default, Copy, Clone, Debug)]
pub struct ChunkGridCoordinate {
    pub x: i64,
    pub z: i64,
}

impl ChunkGridCoordinate {
    pub fn new(x: i64, z: i64) -> Self {
        Self { x, z }
    }

    pub fn from_world_coordinate(WorldCoordinate { x, z, .. }: WorldCoordinate) -> Self {
        Self {
            x: (x / 16.0).floor() as i64,
            z: (z / 16.0).floor() as i64,
        }
    }

    pub fn manhattan_distance_to(&self, other: ChunkGridCoordinate) -> Option<usize> {
        ((self.x - other.x).abs() + (self.z - other.z).abs())
            .try_into()
            .ok()
    }

    pub fn north(&self) -> ChunkGridCoordinate {
        ChunkGridCoordinate::new(self.x, self.z + 1)
    }

    pub fn south(&self) -> ChunkGridCoordinate {
        ChunkGridCoordinate::new(self.x, self.z - 1)
    }

    pub fn east(&self) -> ChunkGridCoordinate {
        ChunkGridCoordinate::new(self.x - 1, self.z)
    }

    pub fn west(&self) -> ChunkGridCoordinate {
        ChunkGridCoordinate::new(self.x + 1, self.z)
    }
}

pub type ChunkGrid = HashMap<ChunkGridCoordinate, Chunk>;
