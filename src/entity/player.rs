use crate::world::WorldCoordinate;

pub struct Player {
    username: String,
    position: WorldCoordinate,
}

impl Player {
    pub fn get_position(&self) -> WorldCoordinate {
        self.position
    }
}
