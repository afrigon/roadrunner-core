use crate::block::Block;
use crate::chunk::ChunkGridCoordinate;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

const CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT;
// pre-computed [ x * WIDTH for x in 0..WIDTH ]
const LOOKUPX: [usize; 16] = [
    0, 16, 32, 48, 64, 80, 96, 112, 128, 144, 160, 176, 192, 208, 224, 240,
];
// pre-computed [ y * HEIGHT for x in 0..HEIGHT ]
const LOOKUPY: [usize; 256] = [
    0, 256, 512, 768, 1024, 1280, 1536, 1792, 2048, 2304, 2560, 2816, 3072, 3328, 3584, 3840, 4096,
    4352, 4608, 4864, 5120, 5376, 5632, 5888, 6144, 6400, 6656, 6912, 7168, 7424, 7680, 7936, 8192,
    8448, 8704, 8960, 9216, 9472, 9728, 9984, 10240, 10496, 10752, 11008, 11264, 11520, 11776,
    12032, 12288, 12544, 12800, 13056, 13312, 13568, 13824, 14080, 14336, 14592, 14848, 15104,
    15360, 15616, 15872, 16128, 16384, 16640, 16896, 17152, 17408, 17664, 17920, 18176, 18432,
    18688, 18944, 19200, 19456, 19712, 19968, 20224, 20480, 20736, 20992, 21248, 21504, 21760,
    22016, 22272, 22528, 22784, 23040, 23296, 23552, 23808, 24064, 24320, 24576, 24832, 25088,
    25344, 25600, 25856, 26112, 26368, 26624, 26880, 27136, 27392, 27648, 27904, 28160, 28416,
    28672, 28928, 29184, 29440, 29696, 29952, 30208, 30464, 30720, 30976, 31232, 31488, 31744,
    32000, 32256, 32512, 32768, 33024, 33280, 33536, 33792, 34048, 34304, 34560, 34816, 35072,
    35328, 35584, 35840, 36096, 36352, 36608, 36864, 37120, 37376, 37632, 37888, 38144, 38400,
    38656, 38912, 39168, 39424, 39680, 39936, 40192, 40448, 40704, 40960, 41216, 41472, 41728,
    41984, 42240, 42496, 42752, 43008, 43264, 43520, 43776, 44032, 44288, 44544, 44800, 45056,
    45312, 45568, 45824, 46080, 46336, 46592, 46848, 47104, 47360, 47616, 47872, 48128, 48384,
    48640, 48896, 49152, 49408, 49664, 49920, 50176, 50432, 50688, 50944, 51200, 51456, 51712,
    51968, 52224, 52480, 52736, 52992, 53248, 53504, 53760, 54016, 54272, 54528, 54784, 55040,
    55296, 55552, 55808, 56064, 56320, 56576, 56832, 57088, 57344, 57600, 57856, 58112, 58368,
    58624, 58880, 59136, 59392, 59648, 59904, 60160, 60416, 60672, 60928, 61184, 61440, 61696,
    61952, 62208, 62464, 62720, 62976, 63232, 63488, 63744, 64000, 64256, 64512, 64768, 65024,
    65280,
];

type Blocks = Vec<Block>;

/// computes flat array index from a 3d coordinate
fn at(x: usize, y: usize, z: usize) -> usize {
    LOOKUPX[x] + LOOKUPY[y] + z
}

pub struct Chunk {
    blocks: Blocks,
    pub coords: ChunkGridCoordinate,
}

impl Chunk {
    pub fn new(coords: ChunkGridCoordinate) -> Self {
        Self {
            blocks: vec![Block { id: 0 }; CHUNK_SIZE],
            coords,
        }
    }

    /// returns the block at relative position x, y ,z
    pub fn block(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[at(x, y, z)]
    }

    /// replaces the block at relative position x, y, z
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.blocks[at(x, y, z)] = block
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn at_returns_within_volume() {
        for x in 0..super::CHUNK_WIDTH {
            for z in 0..super::CHUNK_DEPTH {
                for y in 0..super::CHUNK_HEIGHT {
                    let i = super::at(x, y, z);
                    assert!((0..super::CHUNK_SIZE).contains(&i));
                }
            }
        }
    }

    #[test]
    fn at_returns_unique_values() {
        let mut s = std::collections::HashSet::new();
        for x in 0..super::CHUNK_WIDTH {
            for z in 0..super::CHUNK_DEPTH {
                for y in 0..super::CHUNK_HEIGHT {
                    let i = super::at(x, y, z);
                    assert!(!s.contains(&i));
                    s.insert(i);
                }
            }
        }
    }
}
