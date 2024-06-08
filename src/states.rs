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

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect, Default)]
pub enum PlayingState {
    #[default]
    NotPlaying,
    Local,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    Disabled,
}