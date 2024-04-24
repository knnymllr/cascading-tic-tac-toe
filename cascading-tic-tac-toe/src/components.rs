use crate::CellState;
use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy::prelude::{Entity, NextState, ResMut, Resource, States};

/// Represents a grid cell containing a TicTacToeCell
#[derive(Component, Clone)]
pub struct GridCell {
    // Max board 4,294,967,295 x 4,294,967,295
    // max number of games, n = 4,294,967,292 (MAX - 3)
    // Third option: No target, no time, play til you want to quit
    // pub cell_id: u32,      // Unique identifier for the cell
    pub cell_coord: (u32, u32), // Unique identifier for the cell
    pub state: CellState,       // TicTacToeCell component associated with the grid cell
}

/// Wrapper for managing state transitions
#[derive(Debug)]
pub struct StateWrapper<'w, T: States> {
    pub current: T,                     // Current state
    pub next: ResMut<'w, NextState<T>>, // Next state
}

/// Tag component used to mark which setting is currently selected
#[derive(Resource)]
pub struct MainCamera {
    pub id: Option<Entity>,
}

/// Menu components
#[derive(Component)]
pub struct CameraMenu;

/// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
pub struct OnMainMenuScreen;

/// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
pub struct OnSettingsMenuScreen;

/// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
pub struct OnDisplaySettingsMenuScreen;

/// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
pub struct OnSoundSettingsMenuScreen;

/// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

/// All actions that can be triggered from a button click
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

/// Sound settings that can be set through setting submenu.
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct SoundVolume(pub u32);

/// Background music
#[derive(Component)]
pub struct MyMusic;

/// Display settings that can be set through the setting submenu.
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplaySize {
    Small,
    Medium,
    Large,
}
/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
pub struct ResolutionSettings {
    pub large: Vec2,
    pub medium: Vec2,
    pub small: Vec2,
}
