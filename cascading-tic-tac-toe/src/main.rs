use bevy::prelude::*;

pub use states::*;
pub use components::*;
pub use game_instructions::*;
pub use winning_logic::*;
pub use new_game::*;
pub use board::*;

mod states;
mod components;
mod game_instructions;
mod winning_logic;
mod new_game;
mod board;

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
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_state(PlayingState::Local)
    .insert_state(PlayerTurn::X)
    .insert_state(GameState::GameOngoing)
    .add_plugins(BoardPlugin)
    .add_plugins(WinningLogicPlugin)
    .add_plugins(GameInstructionsPlugin)
    .add_plugins(NewGamePlugin)
    .run();
}
