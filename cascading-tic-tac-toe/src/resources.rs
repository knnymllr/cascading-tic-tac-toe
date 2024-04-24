// @see https://bevy-cheatbook.github.io/programming/res.html

use bevy::prelude::*;

/// A resource that keeps track of player score, target score/time,
/// the number of rounds played, and a vector holding arrays of tuples
/// that signify the valid winning combinations on the board
#[derive(Resource)]
pub struct RoundInit {
    pub round_count: u32,
    pub target: u32,
    pub x_score: u32,
    pub o_score: u32,
    // pub time: u32,
    pub game_combinations: Vec<[(u32,u32);3]>
}

/// A constructor for RoundInit
impl RoundInit {
    pub fn new(init_target: u32) -> Self {
        RoundInit {
            round_count: 0,
            target: init_target,
            x_score: 0,
            o_score: 0,
            game_combinations: Vec::new(),
        }
    }
}
