use crate::chunk::{Chunk, ChunkGrid, ChunkGridCoordinate, ChunkGroup};
use crate::utils::ThreadPool;
use crate::world::generation::generate_chunk;
use crate::world::generation::WorldSeed;
use crate::world::WorldCoordinate;

use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

#[cfg(debug_assertions)]
pub const LOAD_DISTANCE: i64 = 2;
#[cfg(not(debug_assertions))]
pub const LOAD_DISTANCE: i64 = 12;

type ChunkLoadingChannel = (Sender<Chunk>, Receiver<Chunk>);

pub struct World {
    pub chunks: ChunkGrid,
    pub chunk_updates: HashSet<ChunkGridCoordinate>,
    world_seed: WorldSeed,
    chunk_loading_chan: ChunkLoadingChannel,
    threadpool: ThreadPool,
    loading_chunks: HashSet<ChunkGridCoordinate>,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: ChunkGrid::default(),
            chunk_updates: HashSet::new(),
            world_seed: WorldSeed::new(),
            chunk_loading_chan: channel(),
            loading_chunks: HashSet::new(),
            threadpool: ThreadPool::new(8),
        }
    }

    pub fn load_chunk(&mut self, coords: ChunkGridCoordinate) {
        if !self.loading_chunks.contains(&coords) && !self.chunks.contains_key(&coords) {
            let seed = self.world_seed;

            // start a generating thread for the chunk
            let (sender, _) = &self.chunk_loading_chan;
            let tx = sender.clone();
            self.threadpool
                .run(move || tx.send(generate_chunk(coords, seed)).unwrap());
            self.loading_chunks.insert(coords);
        }
    }

    pub fn load_around(&mut self, positions: Vec<WorldCoordinate>) {
        self.chunk_updates.clear();
        // get back chunks from generating thread
        let (_, receiver) = &self.chunk_loading_chan;
        match receiver.try_recv() {
            Ok(chunk) => {
                self.loading_chunks.remove(&chunk.coords);
                self.chunk_updates.insert(chunk.coords);
                for coord in chunk.coords.neighbours() {
                    if self.chunks.contains_key(&coord) {
                        self.chunk_updates.insert(coord);
                    }
                }
                self.chunks.insert(chunk.coords, chunk);
            }
            Err(_) => (),
        };

        // (un?)load chunks as the players move
        let mut chunks_to_load = HashSet::new();
        let mut chunks_to_keep = HashSet::new();
        for position in positions {
            let target_chunk = ChunkGridCoordinate::from_world_coordinate(position);

            let mut counter: i64 = 0;
            for i in 0..=LOAD_DISTANCE {
                for x in -i..=i {
                    for z in -i..=i {
                        let coords =
                            ChunkGridCoordinate::new(target_chunk.x + x, target_chunk.z + z);
                        if !self.chunks.contains_key(&coords) {
                            if counter < LOAD_DISTANCE * 2 {
                                chunks_to_load.insert(coords);
                                counter += 1;
                            }
                        } else {
                            chunks_to_keep.insert(coords);
                        }
                    }
                }
            }

            self.chunks
                .retain(|coords, _| chunks_to_keep.contains(coords));
        }

        for coord in chunks_to_load {
            self.load_chunk(coord);
        }
    }

    pub fn get_chunk_group(&self, coords: ChunkGridCoordinate) -> ChunkGroup {
        ChunkGroup::new(
            self.chunks.get(&coords),
            self.chunks.get(&coords.north()),
            self.chunks.get(&coords.south()),
            self.chunks.get(&coords.east()),
            self.chunks.get(&coords.west()),
        )
    }
}
