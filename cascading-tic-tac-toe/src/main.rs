use bevy::prelude::*;

mod states;
pub use states::*;
mod components;
pub use components::*;
mod board;
pub use board::*;
mod winning_logic;
pub use winning_logic::*;
mod game_instructions;
pub use game_instructions::*;
mod new_game;
pub use new_game::*;

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
