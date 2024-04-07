// @see https://bevy-cheatbook.github.io/programming/res.html

use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundInit {
    pub round_count: u32,
    pub target: u32,
    // pub time: u32,
}

impl RoundInit {
    pub fn new(init_round_count: u32, init_target: u32) -> Self {
        RoundInit { round_count: init_round_count, target: init_target }
    }
}