use crate::chunk::{Chunk, ChunkGrid, ChunkGridCoordinate, ChunkGroup};
use crate::utils::ThreadPool;
use crate::world::generation::generate_chunk;
use crate::world::WorldCoordinate;
use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

#[cfg(debug_assertions)]
const LOAD_DISTANCE: i64 = 2;
#[cfg(not(debug_assertions))]
const LOAD_DISTANCE: i64 = 12;

type ChunkLoadingChannel = (Sender<Chunk>, Receiver<Chunk>);

pub struct World {
    pub chunks: ChunkGrid,
    chunk_loading_chan: ChunkLoadingChannel,
    threadpool: ThreadPool,
    loading_chunks: HashSet<ChunkGridCoordinate>,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: ChunkGrid::default(),
            chunk_loading_chan: channel(),
            loading_chunks: HashSet::new(),
            threadpool: ThreadPool::new(8),
        }
    }

    pub fn load_chunk(&mut self, coords: ChunkGridCoordinate) {
        if !self.loading_chunks.contains(&coords) && !self.chunks.contains_key(&coords) {
            // start a generating thread for the chunk
            let (sender, _) = &self.chunk_loading_chan;
            let tx = sender.clone();
            self.threadpool
                .run(move || tx.send(generate_chunk(coords)).unwrap());
            self.loading_chunks.insert(coords);
        }
    }

    pub fn load_around(&mut self, positions: Vec<WorldCoordinate>) {
        // get back chunks from generating thread
        let (_, receiver) = &self.chunk_loading_chan;
        match receiver.try_recv() {
            Ok(chunk) => {
                self.loading_chunks.remove(&chunk.coords);
                self.chunks.insert(chunk.coords, chunk);
            }
            Err(_) => (),
        };

        // (un?)load chunks as the players move
        let mut chunks_to_load = HashSet::new();
        for position in positions {
            let target_chunk = ChunkGridCoordinate::from_world_coordinate(position);
            let xrange = target_chunk.x - LOAD_DISTANCE..target_chunk.x + LOAD_DISTANCE;
            let zrange = target_chunk.z - LOAD_DISTANCE..target_chunk.z + LOAD_DISTANCE;

            for x in xrange {
                for z in zrange.clone() {
                    chunks_to_load.insert(ChunkGridCoordinate::new(x, z));
                }
            }
        }

        self.chunks
            .retain(|coord, _| chunks_to_load.contains(coord));
        for coord in chunks_to_load {
            self.load_chunk(coord);
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
