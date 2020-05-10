use crate::entity::Player;
use crate::world::World;

use std::vec::Vec;

#[derive(Default)]
pub struct Game {
    world: World,
    players: Vec<Player>,
}

impl Game {
    pub fn update(&mut self, time_delta: f64) {
        self.world.update(&self.players)
    }
}
