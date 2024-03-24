use bevy::prelude::{Reflect, States};

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect)]
pub enum PlayerTurn {
    X,
    O,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum CellState {
    Empty,
    Filled(Player),
    Grid,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect)]
pub enum GameState {
    Won(Player),
    Draw,
    GameOngoing,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect)]
pub enum PlayingState {
    NotPlaying,
    Local,
}