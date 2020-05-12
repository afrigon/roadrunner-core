use crate::chunk::Chunk;
use crate::world::WorldCoordinate;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Default, Copy, Clone, Debug)]
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

    pub fn are_neighbours(left: &ChunkGridCoordinate, right: &ChunkGridCoordinate) -> bool {
        ((left.x - right.x).abs() == 1 && left.z - right.z == 0)
            || (left.x - right.x == 0 && (left.z - right.z).abs() == 1)
    }
}

pub type ChunkGrid = HashMap<ChunkGridCoordinate, Chunk>;
