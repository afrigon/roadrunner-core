use math::container::Area;
use math::random::noise::NoiseFn;
use std::ops::Range;

pub struct HeightMapOptions {
    pub area: Area,
    pub range: Range<u8>,
}

pub struct HeightMap {
    values: Vec<u8>,
    options: HeightMapOptions,
}

impl HeightMap {
    pub fn new<N: NoiseFn<[i64; 2]>>(area: Area, range: Range<u8>, noise: N) -> Self {
        let height = (range.end - range.start) as f64;
        Self {
            values: area
                .into_iter()
                .map(|(x, y)| (noise.get([x, y]) * height) as u8 + range.start)
                .collect(),
            options: HeightMapOptions { area, range },
        }
    }

    pub fn height(&self, x: u8, z: u8) -> u8 {
        self.values[(x + z * self.options.area.width as u8) as usize]
    }
}
