use bevy::prelude::*;
use crate::{SelectedOption,SoundVolume,MenuButtonAction,MyMusic,OnSoundSettingsMenuScreen,main_menu::*};

pub fn sound_settings_menu_setup(mut commands: Commands, volume: Res<SoundVolume>) {
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
            OnSoundSettingsMenuScreen,
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
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::CRIMSON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Volume",
                                button_text_style.clone(),
                            ));
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(30.0),
                                            height: Val::Px(65.0),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    SoundVolume(volume_setting),
                                ));
                                if *volume == SoundVolume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}

pub fn volume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink, With<MyMusic>>,
) {
    if let Ok(sink) = music_controller.get_single() {
        if keyboard_input.just_pressed(KeyCode::Equal) {
            sink.set_volume(sink.volume() + 0.1);
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            sink.set_volume(sink.volume() - 0.1);
        }
    }
}