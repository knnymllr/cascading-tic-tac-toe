use crate::CellState;
use bevy::ecs::component::Component;
use bevy::prelude::{Entity, NextState, ResMut, Resource, States};

/// Represents a single cell in the Tic-Tac-Toe grid
#[derive(Component, Clone)]
pub struct TicTacToeCell {
    pub cell_id: u32,      // Unique identifier for the cell
    pub state: CellState,  // State of the cell (Empty, Filled(Player))
}

/// Represents a grid cell containing a TicTacToeCell
#[derive(Component, Clone)]
pub struct GridCell {
    // Max board 4,294,967,295 x 4,294,967,295
    // max number of games, n = 4,294,967,292 (MAX - 3)
    // Third option: No target, no time, play til you want to quit
    pub position: (u32, u32),  // Position of the cell in the grid
    pub cell: TicTacToeCell,    // TicTacToeCell component associated with the grid cell
}

/// Wrapper for managing state transitions
#[derive(Debug)]
pub struct StateWrapper<'w, T: States> {
    pub current: T,              // Current state
    pub next: ResMut<'w, NextState<T>>,  // Next state
}


// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenuScreen;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// Tag component used to mark which setting is currently selected
#[derive(Resource)]
pub struct MainCamera {
    pub id: Option<Entity>
}

