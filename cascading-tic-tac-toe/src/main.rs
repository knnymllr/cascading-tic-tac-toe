use bevy_kira_audio::prelude::*;
use bevy::prelude::*;

pub use menus::*;
pub use states::*;
pub use components::*;
pub use resources::*;
pub use game_instructions::*;
pub use winning_logic::*;
pub use in_game_menu::*;
pub use board::*;
pub use game_screen::*;

mod states;
mod components;
mod resources;
mod game_instructions;
mod winning_logic;
mod in_game_menu;
mod board;
mod menus;
mod game_screen;

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
    .add_plugins(AudioPlugin)
    .insert_resource(DisplayQuality::Medium)
    .insert_resource(SoundVolume(7))
    .init_resource::<UiTheme>()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource::<MainCamera>(MainCamera{id:None})
    .insert_resource(RoundCount::new(4))
    .insert_state(MenuState::Main)
    .insert_state(PlayingState::NotPlaying)
    .insert_state(PlayerTurn::X)
    .insert_state(GameState::GameOngoing)
    .add_plugins(WinningLogicPlugin)
    .add_plugins(main_menu::MenuPlugin)
    .add_plugins(GameScreen)
    .add_systems(Startup, start_background_audio)
    .run();
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/mammoth.ogg")).looped();
}