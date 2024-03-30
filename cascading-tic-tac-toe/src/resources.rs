// @see https://bevy-cheatbook.github.io/programming/res.html

use bevy::prelude::*;

#[derive(Resource)]
struct GameProgress {
    number_of_games: u32,
    // TODO: mode: timer/target,
    // TODO: versus_computer: bool,
}