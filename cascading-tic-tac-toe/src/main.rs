use bevy::prelude::*;

pub use states::*;
pub use components::*;
pub use resources::*;
pub use game_instructions::*;
pub use winning_logic::*;
pub use in_game_menu::*;
pub use board::*;
pub use start_menu::*;

mod states;
mod components;
mod resources;
mod game_instructions;
mod winning_logic;
mod in_game_menu;
mod board;
mod start_menu;

// #[derive(Resource)]
// pub struct RoundCount(u32);

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
    .insert_resource(RoundCount::new(0))
    .insert_state(MenuState::Main)
    .insert_state(PlayingState::Waiting)
    .insert_state(PlayerTurn::X)
    .insert_state(GameState::GameOngoing)
    .add_plugins(BoardPlugin)
    .add_plugins(WinningLogicPlugin)
    .add_plugins(GameInstructionsPlugin)
    .add_plugins(NewGamePlugin)
    .add_plugins(MenuPlugin)
    .run();
}
