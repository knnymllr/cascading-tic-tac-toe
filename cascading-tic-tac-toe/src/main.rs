#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Cursor;
use bevy_kira_audio::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use winit::window::Icon;

pub use states::*;
pub use components::*;
pub use resources::*;
pub use game_instructions::*;
pub use winning_logic::*;
pub use in_game_menu::*;
pub use board::*;
pub use start_menu::*;
pub use game_screen::*;

mod states;
mod components;
mod resources;
mod game_instructions;
mod winning_logic;
mod in_game_menu;
mod board;
mod start_menu;
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
    .init_resource::<UiTheme>()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource::<MainCamera>(MainCamera{id:None})
    .insert_resource(RoundCount::new(4))
    .insert_state(MenuState::Main)
    .insert_state(PlayingState::NotPlaying)
    .insert_state(PlayerTurn::X)
    .insert_state(GameState::GameOngoing)
    .add_plugins(WinningLogicPlugin)
    .add_plugins(MenuPlugin)
    .add_plugins(GameScreen)
    .add_systems(Startup, (set_window_icon, start_background_audio))
    .run();
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/mammoth.ogg")).looped();
}

fn set_window_icon(windows: NonSend<WinitWindows>, primary_window: Query<Entity, With<PrimaryWindow>>) {

    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else { return };
    let icon_buf = Cursor::new(include_bytes!("../assets/texture/icons/icon.png"));

    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}