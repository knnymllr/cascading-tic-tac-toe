use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect)]
pub enum PlayerTag {
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
    Valid,
    Invalid,
    Filled(PlayerTag),
    Won(PlayerTag),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect)]
pub enum GameState {
    Restarting,
    NotPlaying,
    GameOngoing,    // Represents the game state when the game is still ongoing.
    Won(PlayerTag), // Represents the game state when a player has won.
    Draw,           // Represents the game state when the game ends in a draw.
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect, Default)]
pub enum RoundState {
    #[default]
    NotPlaying,
    Playing,
    UpdatingX,
    UpdatingO,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect, Default)]
pub enum PlayingState {
    #[default]
    NotPlaying,
    Local,
}

// State used for the start menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    Disabled,
}
