use crate::chunk::{Chunk, ChunkGrid, ChunkGridCoordinate, ChunkGroup};
use crate::world::generation::generate_chunk;
use crate::world::generation::WorldSeed;
use crate::world::WorldCoordinate;
use std::collections::BinaryHeap;

use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[cfg(debug_assertions)]
pub const LOAD_DISTANCE: i64 = 2;
#[cfg(not(debug_assertions))]
pub const LOAD_DISTANCE: i64 = 12;

type ChunkLoadingChannel = (Sender<Chunk>, Receiver<Chunk>);

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct WeightedChunkGridCoordinate {
    distance: usize,
    coord: ChunkGridCoordinate,
}

pub struct World {
    pub chunks: ChunkGrid,
    chunk_loading_chan: ChunkLoadingChannel,
    chunk_loading_queue: Arc<Mutex<BinaryHeap<WeightedChunkGridCoordinate>>>,
}

impl World {
    pub fn new() -> Self {
        let world_seed = WorldSeed::new();
        let chunk_loading_queue =
            Arc::new(Mutex::new(BinaryHeap::<WeightedChunkGridCoordinate>::new()));
        let c_queue = chunk_loading_queue.clone();
        let (sender, receiver) = channel();
        let tx = sender.clone();
        thread::spawn(move || loop {
            let msg = {
                let mut lock = c_queue.lock().unwrap();
                lock.pop()
            };

            match msg {
                Some(chunk) => tx.send(generate_chunk(chunk.coord, world_seed)).unwrap(),
                _ => (),
            }
        });
        World {
            chunks: ChunkGrid::default(),
            chunk_loading_chan: (sender, receiver),
            chunk_loading_queue,
        }
    }

    pub fn load_around(&mut self, positions: Vec<WorldCoordinate>) {
        // get back chunks from generating thread
        let (_, receiver) = &self.chunk_loading_chan;
        match receiver.try_recv() {
            Ok(chunk) => {
                self.chunks.insert(chunk.coords, chunk);
            }
            Err(_) => (),
        };

        // (un?)load chunks as the players move
        let mut chunks_to_load = HashMap::<ChunkGridCoordinate, usize>::new();
        for position in positions {
            let target_chunk = ChunkGridCoordinate::from_world_coordinate(position);
            let xrange = target_chunk.x - LOAD_DISTANCE..target_chunk.x + LOAD_DISTANCE;
            let zrange = target_chunk.z - LOAD_DISTANCE..target_chunk.z + LOAD_DISTANCE;

            for x in xrange {
                for z in zrange.clone() {
                    let coord = ChunkGridCoordinate::new(x, z);
                    if let Some(new_dist) = coord.manhattan_distance_to(target_chunk) {
                        if let Some(old_dist) = chunks_to_load.get(&coord) {
                            if &new_dist < old_dist {
                                chunks_to_load.insert(coord, std::usize::MAX - new_dist);
                            }
                        } else {
                            chunks_to_load.insert(coord, std::usize::MAX - new_dist);
                        }
                    }
                }
            }
        }

        let _ = {
            if let Ok(mut lock) = self.chunk_loading_queue.try_lock() {
                println!("len(chunk_loading_queue) {}", lock.len());

                lock.clear();
                for (coord, distance) in &chunks_to_load {
                    if !self.chunks.contains_key(coord) {
                        lock.push(WeightedChunkGridCoordinate {
                            coord: *coord,
                            distance: *distance,
                        });
                    }
                }
            } else {
                println!("didn't get lock");
            }
        };

        self.chunks
            .retain(|coord, _| chunks_to_load.contains_key(coord));
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
