use crate::block::Block;
use crate::chunk::{Chunk, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};

pub struct ChunkGroup<'c> {
    pub middle: Option<&'c Chunk>,
    pub north: Option<&'c Chunk>,
    pub south: Option<&'c Chunk>,
    pub east: Option<&'c Chunk>,
    pub west: Option<&'c Chunk>,
}

impl<'c> ChunkGroup<'c> {
    pub fn new(
        middle: Option<&'c Chunk>,
        north: Option<&'c Chunk>,
        south: Option<&'c Chunk>,
        east: Option<&'c Chunk>,
        west: Option<&'c Chunk>,
    ) -> Self {
        Self {
            middle,
            north,
            south,
            east,
            west,
        }
    }

    pub fn get_block(&self, x: i8, y: i16, z: i8) -> Option<Block> {
        if y < 0 || y > CHUNK_HEIGHT as i16 {
            return None;
        }

        let y = y as usize;

        if x < 0 {
            return Some(
                self.east?
                    .block((x + CHUNK_WIDTH as i8) as usize, y, z as usize),
            );
        }

        let x = x as usize;

        if z < 0 {
            return Some(self.south?.block(x, y, (z + CHUNK_DEPTH as i8) as usize));
        }

        let z = z as usize;

        if x >= CHUNK_WIDTH {
            return Some(self.west?.block(x - CHUNK_WIDTH, y, z));
        }

        if z >= CHUNK_DEPTH {
            return Some(self.north?.block(x, y, z - CHUNK_DEPTH));
        }

        Some(self.middle?.block(x, y, z))
    }
}

impl<'c> IntoIterator for ChunkGroup<'c> {
    type Item = &'c Chunk;
    type IntoIter = ChunkGroupIterator<'c>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            group: self,
            index: 0,
        }
    }
}

pub struct ChunkGroupIterator<'c> {
    group: ChunkGroup<'c>,
    index: usize,
}

impl<'c> Iterator for ChunkGroupIterator<'c> {
    type Item = &'c Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        for i in self.index..=5 {
            match i {
                1 if self.group.middle.is_some() => return self.group.middle,
                2 if self.group.north.is_some() => return self.group.north,
                3 if self.group.south.is_some() => return self.group.south,
                4 if self.group.east.is_some() => return self.group.east,
                5 if self.group.west.is_some() => return self.group.west,
                _ => self.index = i + 1,
            };
        }

        None
    }
}
