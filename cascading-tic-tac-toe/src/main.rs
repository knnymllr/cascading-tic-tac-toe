#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::theme::theme::UiTheme;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy_kira_audio::prelude::*;
use std::io::Cursor;
use winit::window::Icon;

pub use board::*;
pub use components::*;
pub use game_instructions::*;
pub use game_screen::*;
pub use game_scores::*;
pub use in_game_menu::*;
pub use menus::*;
pub use resources::*;
pub use states::*;
pub use winning_logic::*;

mod board;
mod components;
mod game_instructions;
mod game_screen;
mod game_scores;
mod in_game_menu;
mod menus;
mod resources;
mod states;
mod winning_logic;

mod ui_components {
    pub mod bundles;
}

mod theme {
    pub mod theme;
}

mod utils {
    pub mod despawn_screen;
    pub mod modify_text;
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
    .add_plugins(AudioPlugin)
    .insert_resource(ResolutionSettings {
        large: Vec2::new(1920.0, 1080.0),
        medium: Vec2::new(1000.0, 600.0),
        small: Vec2::new(560.0, 820.0),
    })
    .insert_resource(DisplaySize::Medium)
    .insert_resource(SoundVolume(7))
    .init_resource::<UiTheme>()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource::<MainCamera>(MainCamera { id: None })
    .insert_state(MenuState::Main)
    // .add_plugins(WinningLogicPlugin)
    .add_plugins(main_menu::MenuPlugin)
    .add_plugins(GameScreen)
    .add_systems(
        Startup,
        (place_camera, set_window_icon, start_background_audio),
    )
    .run();
}

fn start_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/mammoth.ogg"),
            ..default()
        },
        MyMusic,
    ));
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!("../assets/texture/icons/icon.png"));

    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

fn place_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
