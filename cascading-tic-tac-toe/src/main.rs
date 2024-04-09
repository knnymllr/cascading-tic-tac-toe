use bevy_kira_audio::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use winit::window::Icon;
// use std::time::Duration;
// use timer::{Counter, TEXT_COLOR, TIME, time};

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
    .init_resource::<UiTheme>()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource::<MainCamera>(MainCamera{id:None})
    .insert_resource(RoundInit::new(4, 5))
    .insert_state(MenuState::Main)
    .insert_state(PlayingState::NotPlaying)
    .insert_state(PlayerTurn::X)
    .insert_state(GameState::GameOngoing)
    .add_plugins(WinningLogicPlugin)
    .add_plugins(MenuPlugin)
    .add_plugins(GameScreen)
    .add_systems(Startup, (add_camera, set_window_icon, start_background_audio))
    // .add_systems(Startup, (add_camera, set_window_icon, start_background_audio, add_text))
    // .add_systems(Update,update_time)
    .add_systems(Startup, start_background_audio)
    .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


// fn add_text(mut commands: Commands, asset_sever: Res<AssetServer>){
//     let counter = Counter::new();
//     //counter.pause();

//     commands.spawn(TextBundle{
//         text: Text::from_section(
//             format!("{}", time(Duration::from_secs(TIME.into()))),
//             TextStyle {
//                  font: asset_sever.load("fonts/FiraMono-Medium.ttf"),
//                  font_size: 120., 
//               color: TEXT_COLOR,
//                },
//         ),
//           style: Style{
//               position_type: PositionType:: Absolute,
//               ..default()
//        },
//      ..default()
//    }).insert(counter);
//  } 

// fn update_time(mut query: Query<(&mut Text, &mut Counter)>, os_time: Res<Time>){
//     for (mut text, mut counter) in &mut query{
//         if counter.paused(){
//             continue;
//         }
//         counter.tick(os_time.delta());
//         if counter.unit_just_finished(){
//             text.sections[0].value = format!("{}", time(counter.duration() - Duration::from_secs_f32(counter.elapsed_secs_round())))
//         }
        
//     }
// }

fn start_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/mammoth.ogg"),
            ..default()
        },
        MyMusic,
    ));
    
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


fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/mammoth.ogg")).looped();
}