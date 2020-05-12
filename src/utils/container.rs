use crate::chunk::{ChunkGridCoordinate, CHUNK_DEPTH, CHUNK_WIDTH};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Area {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

impl Area {
    pub fn new(x: i64, y: i64, width: i64, height: i64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn new_chunk(coords: ChunkGridCoordinate) -> Self {
        Self {
            x: coords.x * CHUNK_WIDTH as i64,
            y: coords.z * CHUNK_DEPTH as i64,
            width: CHUNK_WIDTH as i64,
            height: CHUNK_DEPTH as i64,
        }
    }

    pub fn contains(&self, x: i64, y: i64) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

impl IntoIterator for Area {
    type Item = (i64, i64);
    type IntoIter = AreaIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            area: self,
            iter_x: 0,
            iter_y: 0,
        }
    }
}

pub struct AreaIterator {
    area: Area,
    iter_x: i64,
    iter_y: i64,
}

impl Iterator for AreaIterator {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_y == self.area.height {
            return None;
        }

        let value = (self.area.x + self.iter_x, self.area.y + self.iter_y);

        self.iter_x += 1;
        if self.iter_x == self.area.width {
            self.iter_x = 0;
            self.iter_y += 1;
        }

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const X: i64 = 4;
    const Y: i64 = 4;
    const WIDTH: i64 = 6;
    const HEIGHT: i64 = 3;

    #[test]
    fn area_does_not_contain_outside_points() {
        let area = Area::new(X, Y, WIDTH, HEIGHT);

        assert!(!area.contains(X - 1, Y));
        assert!(!area.contains(X, Y - 1));

        assert!(!area.contains(X + WIDTH, Y));
        assert!(!area.contains(X, Y + HEIGHT));
    }

    #[test]
    fn area_does_contain_inside_points() {
        let area = Area::new(X, Y, WIDTH, HEIGHT);

        assert!(area.contains(X, Y));
    }

    #[test]
    fn iterator_every_item_is_unique() {
        let mut set = HashSet::new();
        let area = Area::new(X, Y, WIDTH, HEIGHT);

        area.into_iter().for_each(|(x, y)| {
            assert!(!set.contains(&(x, y)));
            set.insert((x, y));
        });
    }

    #[test]
    fn iterator_every_item_is_in_area() {
        let area = Area::new(X, Y, WIDTH, HEIGHT);
        assert!(area.into_iter().all(|(x, y)| area.contains(x, y)))
    }

    #[test]
    fn iterator_return_enough_items() {
        let area = Area::new(X, Y, WIDTH, HEIGHT);
        let expected = (WIDTH * HEIGHT) as usize;
        assert_eq!(area.into_iter().count(), expected);
    }
}
