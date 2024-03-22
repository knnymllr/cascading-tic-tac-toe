use bevy::prelude::*;

pub use states::*;
pub use board::*;

mod states;
mod components;
mod game_instructions;
mod board;

fn main() {
    println!("Hello, world!");
}
