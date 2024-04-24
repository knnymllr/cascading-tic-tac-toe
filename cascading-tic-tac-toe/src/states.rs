use bevy::prelude::*;

/// State that tracks the name of the opposing players
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect)]
pub enum PlayerTag {
    X,
    O,
}

/// Formatting constructor for PlayerTag
impl std::fmt::Display for PlayerTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self) // Use Debug formatting for simplicity
    }
  }

/// State that keeps track of current player turn
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect)]
pub enum PlayerTurn {
    X,
    O,
}

/// State to keep track of whether a cell is valid, invalid, filled,
/// or part of a winning combination
#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum CellState {
    Valid,
    Invalid,
    Filled(PlayerTag),
    Won(PlayerTag), // TODO: Fill cells in winning combos with bg color (color chooser?)
}

/// State to keep track of the inner gamestate
/// Otherwise GameState::NotPlaying is the default value
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect, Default)]
pub enum GameState {
    #[default]
    NotPlaying,
    LoadingNewGame,  
    RestartingGame,       
    GameOngoing,  
    Updating,  
    Won(PlayerTag),
}

/// State to keep track of the inner round state, which is either updating or not updating
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect, Default)]
pub enum RoundState {
    #[default]
    NotUpdating,
    UpdatingRound,
}

/// State to keep track of gameplay arena
/// Online and VsComputer are not implemented
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect, Default)]
pub enum PlayingState {
    #[default]
    NotPlaying,
    Loading,
    Local,
    Online, // TODO
    VsComputer, // TODO
}

/// State used for the start menu screen
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
