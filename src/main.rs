use bevy::prelude::*;

pub use states::*;
pub use components::*;
pub use game_instructions::*;
pub use winning_logic::*;
pub use in_game_menu::*;
pub use board::*;
pub use main_menu::*;
pub use game_screen::*;
use crate::theme::theme::UiTheme;

mod states;
mod components;
mod game_instructions;
mod winning_logic;
mod in_game_menu;
mod board;
mod main_menu;
mod game_screen;

mod theme {
    pub mod theme;
}

mod ui_components {
    pub mod bundles;
}

mod utils {
    pub mod despawn_screen;
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            window_level: bevy::window::WindowLevel::Normal,
            title: "Tic Tac Toe!".to_string(),
            ..default()
        }),
        ..default()
    }))
    .init_resource::<UiTheme>()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource::<MainCamera>(MainCamera{id:None})
    .insert_state(MenuState::Main)
    .insert_state(PlayingState::NotPlaying)
    .insert_state(PlayerTurn::X)
    .insert_state(GameState::GameOngoing)
    .add_plugins(WinningLogicPlugin)
    .add_plugins(MenuPlugin)
    .add_plugins(GameScreen)
    .add_systems(
        Startup,
        place_camera,
    )
    .run();
}

fn place_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}