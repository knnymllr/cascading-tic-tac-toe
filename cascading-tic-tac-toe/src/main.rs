use bevy::prelude::*;

pub use states::*;
pub use components::*;

mod states;
mod components;
mod winning_logic;

fn main() {
    println!("Hello, world!");
}
