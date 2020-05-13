use crate::block::Block;
use crate::chunk::{Chunk, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};

pub struct ChunkGroup<'c> {
    pub current: &'c Chunk,
    pub north: Option<&'c Chunk>,
    pub south: Option<&'c Chunk>,
    pub east: Option<&'c Chunk>,
    pub west: Option<&'c Chunk>,
}

impl<'c> ChunkGroup<'c> {
    pub fn new(
        current: &'c Chunk,
        north: Option<&'c Chunk>,
        south: Option<&'c Chunk>,
        east: Option<&'c Chunk>,
        west: Option<&'c Chunk>,
    ) -> Self {
        Self {
            current,
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

        return Some(self.current.block(x, y, z));
    }
}

impl<'c> IntoIterator for ChunkGroup<'c> {
    type Item = Option<&'c Chunk>;
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
    type Item = Option<&'c Chunk>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index {
            1 => Some(self.group.north),
            2 => Some(self.group.south),
            3 => Some(self.group.east),
            4 => Some(self.group.west),
            _ => None,
        }
    }
}
