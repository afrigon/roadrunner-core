use crate::entity::Player;
use crate::world::{World, WorldCoordinate};

use std::vec::Vec;

pub struct Game {
    world: World,
    players: Vec<Player>,
}

impl Game {
    pub fn update(&mut self, time_delta: f64) {
        self.world.update(&self.players)
    }

    pub fn add_player(&mut self, username: String) {
        let player = Player::new(username, WorldCoordinate::default());
        self.players.push(player);
    }
}
