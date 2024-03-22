use bevy::prelude::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Won(Player),
    Draw,
    GameOngoing,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum PlayingState {
    NotPlaying,
    Local,
}