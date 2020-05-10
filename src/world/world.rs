use crate::chunk::{Chunk, ChunkGrid, ChunkGridCoordinate, ChunkGroup};
use crate::entity::Player;
use crate::world::generation::WorldGenerator;

const LOAD_DISTANCE: i64 = 16;

#[derive(Default)]
pub struct World {
    chunks: ChunkGrid,
    generator: WorldGenerator,
}

impl World {
    pub fn load_chunk(&mut self, coords: ChunkGridCoordinate) {
        if !self.chunks.contains_key(&coords) {
            let mut chunk = Chunk::new(coords);
            self.generator.generate_chunk(coords, &mut chunk);

            self.chunks.insert(coords, chunk);
        }
    }

    pub fn update(&mut self, players: &Vec<Player>) {
        // (un?)load chunks as the players move
        for player in players {
            let target_chunk = ChunkGridCoordinate::from_world_coordinate(player.position());
            let is_near = |middle, point| -> bool {
                (middle - LOAD_DISTANCE..middle + LOAD_DISTANCE).contains(&point)
            };

            self.chunks.retain(|coord, _| {
                is_near(target_chunk.x, coord.x) && is_near(target_chunk.z, coord.z)
            });

            let xrange = target_chunk.x - LOAD_DISTANCE..target_chunk.x + LOAD_DISTANCE;
            let zrange = target_chunk.z - LOAD_DISTANCE..target_chunk.z + LOAD_DISTANCE;

            for x in xrange {
                for z in zrange.clone() {
                    let coords = ChunkGridCoordinate::new(x, z);
                    self.load_chunk(coords);
                }
            }
        }
    }

    pub fn get_chunk_group(&self, coords: ChunkGridCoordinate) -> ChunkGroup {
        ChunkGroup::new(
            // TODO: handle the case where the current chunk is not in the hashmap
            self.chunks.get(&coords).unwrap(),
            self.chunks.get(&coords.north()),
            self.chunks.get(&coords.south()),
            self.chunks.get(&coords.east()),
            self.chunks.get(&coords.west()),
        )
    }
}
