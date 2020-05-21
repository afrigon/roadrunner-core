use serde::{Deserialize, Serialize};

use crate::world::WorldCoordinate;

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerEvent {
    PlayerConnected { id: u128 },
    PlayerDisconnected { id: u128 },
    PlayerMoved { id: u128, position: WorldCoordinate },
    PlayerList { ids: Vec<u128> },
}
