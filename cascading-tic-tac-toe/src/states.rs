use bevy::prelude::{Reflect, States};

#[derive(Reflect, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Player {
    X,
    O,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayerTurn {
    X,
    O,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum CellState {
    Empty,
    Filled(Player),
    Grid,
}

#[derive(Reflect, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Won(Player),
    Draw,
    GameOngoing,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayingState {
    NotPlaying,
    Local,
}