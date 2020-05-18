use serde::{Deserialize, Serialize};

use crate::world::WorldCoordinate;

#[derive(Serialize, Deserialize)]
pub enum ServerEvent {
    PlayerConnected { id: u8 },
    PlayerDisconnected { id: u8 },
    PlayerMoved { id: u8, position: WorldCoordinate },
}
