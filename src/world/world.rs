use crate::chunk::{Chunk, ChunkGrid, ChunkGridCoordinate, ChunkGroup};
use crate::entity::Player;
use crate::world::generation::generate_chunk;
use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

const LOAD_DISTANCE: i64 = 16;

type ChunkLoadingChannel = (Sender<Chunk>, Receiver<Chunk>);

pub struct World {
    chunks: ChunkGrid,
    chunk_loading_chan: ChunkLoadingChannel,
    loading_chunks: HashSet<ChunkGridCoordinate>,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: ChunkGrid::default(),
            chunk_loading_chan: channel(),
            loading_chunks: HashSet::new(),
        }
    }

    pub fn load_chunk(&mut self, coords: ChunkGridCoordinate) {
        if !self.loading_chunks.contains(&coords) && !self.chunks.contains_key(&coords) {
            // start a generating thread for the chunk
            let (sender, _) = &self.chunk_loading_chan;
            let tx = sender.clone();
            thread::spawn(move || tx.send(generate_chunk(coords)).unwrap());
            self.loading_chunks.insert(coords);
        }
    }

    pub fn update(&mut self, players: &Vec<Player>) {
        // get back chunks from generating thread
        let (_, receiver) = &self.chunk_loading_chan;
        match receiver.try_recv() {
            Ok(chunk) => self.chunks.insert(chunk.coords, chunk),
            Err(_) => None,
        };

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
