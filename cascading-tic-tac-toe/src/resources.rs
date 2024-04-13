// @see https://bevy-cheatbook.github.io/programming/res.html

use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundInit {
    pub round_count: u32,
    pub target: u32,
    pub x_score: u32,
    pub o_score: u32,
    // pub time: u32,
    pub game_combinations: Vec<[(u32,u32);3]>
}

impl RoundInit {
    pub fn new(init_target: u32) -> Self {
        RoundInit {
            round_count: 3,
            target: init_target,
            x_score: 0,
            o_score: 0,
            game_combinations: Vec::new(),
        }
    }
}
