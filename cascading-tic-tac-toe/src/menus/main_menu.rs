use crate::{
    display_menu::*, sound_menu::*, DisplaySize, GameState, MenuButtonAction, MenuState,
    OnDisplaySettingsMenuScreen, OnMainMenuScreen, OnSettingsMenuScreen, OnSoundSettingsMenuScreen,
    PlayingState, ResolutionSettings, SelectedOption, SoundVolume,
};
use bevy::{app::AppExit, prelude::*};

use crate::ui_components::bundles::{button_bundle, image_bundle, text_bundle};
use crate::utils::despawn_screen::despawn_screen;

/// Colors for main menu
pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// Struct to store button parameters
struct ButtonParams {
    text: &'static str,
    text_color: Color,
    icon_path: &'static str,
    action: MenuButtonAction,
}
/// A struct for the MenuPlugin
pub struct MenuPlugin;

/// A plugin that implements the MenuPlugin for the main application initialization
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(1000.0, 600.0),
            small: Vec2::new(560.0, 820.0),
        })
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        // Systems to handle the settings menu screen
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        // Systems to handle the display settings screen
        .add_systems(
            OnEnter(MenuState::SettingsDisplay),
            display_settings_menu_setup,
        )
        .add_systems(
            Update,
            setting_button::<DisplaySize>.run_if(in_state(MenuState::SettingsDisplay)),
        )
        .add_systems(
            OnExit(MenuState::SettingsDisplay),
            despawn_screen::<OnDisplaySettingsMenuScreen>,
        )
        // Systems to handle the sound settings screen
        .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
        .add_systems(
            Update,
            setting_button::<SoundVolume>.run_if(in_state(MenuState::SettingsSound)),
        )
        .add_systems(
            OnExit(MenuState::SettingsSound),
            despawn_screen::<OnSoundSettingsMenuScreen>,
        )
        // Systems to adjust Audio volume
        // .add_systems(Update, toggle_volume)
        // Systems to adjust screen resolution
        // .add_systems(Update, toggle_resolution)
        // Common systems to all screens that handles buttons behavior
        .add_systems(
            Update,
            (menu_action, button_system, toggle_volume, toggle_resolution)
                .run_if(in_state(PlayingState::NotPlaying)),
        );
    }
}

/// A system that creates the main menu
fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let buttons = vec![
        ButtonParams {
            text: "New Game",
            text_color: TEXT_COLOR,
            icon_path: "texture/icons/right-arrow.png",
            action: MenuButtonAction::Play,
        },
        ButtonParams {
            text: "Settings",
            text_color: TEXT_COLOR,
            icon_path: "texture/icons/wrench.png",
            action: MenuButtonAction::Settings,
        },
        ButtonParams {
            text: "Quit",
            text_color: TEXT_COLOR,
            icon_path: "texture/icons/exit.png",
            action: MenuButtonAction::Quit,
        },
    ];

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        text_bundle("Cascading Tic-Tac-Toe", &asset_server, (80.0, TEXT_COLOR))
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(50.0)),
                                ..default()
                            }),
                    );

                    // Display three buttons for each action available from the main menu:
                    for params in buttons {
                        parent
                            .spawn((
                                button_bundle(
                                    (
                                        Val::Px(250.0),
                                        Val::Px(65.0),
                                        Option::from(UiRect::all(Val::Px(20.0))),
                                        JustifyContent::Center,
                                        AlignItems::Center,
                                    ),
                                    NORMAL_BUTTON.into(),
                                ),
                                params.action,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load(params.icon_path);
                                parent.spawn(image_bundle(UiImage::new(icon)));
                                parent.spawn(text_bundle(
                                    params.text,
                                    &asset_server,
                                    (40.0, params.text_color),
                                ));
                            });
                    }
                });
        });
}

/// This system updates the settings when a new value for a setting is selected,
/// and marks the button as the one currently selected
fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

/// This system sets up the settings menu to toggle resolution and sound volume
fn settings_menu_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                    }
                });
        });
}

/// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

/// A system for handling individual menu actions (Pressed)
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut playing_state: ResMut<NextState<PlayingState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    playing_state.set(PlayingState::Local);
                    game_state.set(GameState::LoadingNewGame);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}
