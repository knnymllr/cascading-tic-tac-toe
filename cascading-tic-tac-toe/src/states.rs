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
    Won(PlayerTag), // TODO: Fill cells in winning combos with bg color (color chooser?)
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect, Default)]
pub enum GameState {
    #[default]
    NotPlaying,
    LoadingNewGame,         
    GameOngoing,    
    Won(PlayerTag),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect, Default)]
pub enum RoundState {
    #[default]
    NotUpdating,
    UpdatingRound,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect, Default)]
pub enum PlayingState {
    #[default]
    NotPlaying,
    Loading,
    Local,
    Online, // TODO
    VsComputer, // TODO
}

// State used for the start menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Main,
    Round,
    RoundTarget,
    RoundTimer,
    Settings,
    SettingsDisplay,
    SettingsSound,
    Disabled,
}
