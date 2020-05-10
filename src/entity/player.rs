use crate::world::WorldCoordinate;

pub struct Player {
    username: String,
    position: WorldCoordinate,
}

impl Player {
    pub fn new(username: String, position: WorldCoordinate) -> Self {
        Self { username, position }
    }

    pub fn position(&self) -> WorldCoordinate {
        self.position
    }
}
