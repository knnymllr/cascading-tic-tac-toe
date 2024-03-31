// @see https://bevy-cheatbook.github.io/programming/res.html

use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundCount(u32);

impl RoundCount {
    pub fn new(initial_value: u32) -> Self {
        RoundCount(initial_value)
    }
    pub fn get_current(&self) -> u32 {
        self.0
    }
}