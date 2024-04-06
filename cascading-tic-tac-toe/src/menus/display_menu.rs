use bevy::prelude::*;
use crate::{DisplaySize,SelectedOption,MenuButtonAction,OnDisplaySettingsMenuScreen,ResolutionSettings,main_menu::*};


pub fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplaySize>) {
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
            OnDisplaySettingsMenuScreen,
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
                    // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                    // use the default value, `FlexDirection::Row`, from left to right.
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
                            // Display a label for the current setting
                            parent.spawn(TextBundle::from_section(
                                "Display Size",
                                button_text_style.clone(),
                            ));
                            // Display a button for each possible value
                            for display_size in [
                                DisplaySize::Small,
                                DisplaySize::Medium,
                                DisplaySize::Large,
                            ] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(150.0),
                                            height: Val::Px(65.0),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    display_size,
                                ));
                                entity.with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        format!("{display_size:?}"),
                                        button_text_style.clone(),
                                    ));
                                });
                                if *display_quality == display_size {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    // Display the back button to return to the settings screen
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

/// This system shows how to request the window to a new resolution
pub fn toggle_resolution(
    interaction_query: Query<
    (&Interaction, &DisplaySize),
    (Changed<Interaction>, With<Button>),>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = windows.single_mut();
    for (interaction, display_size) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match display_size {
                &DisplaySize :: Small => {
                    let res = resolution.small;
                    window.resolution.set(res.x, res.y);
                }
                DisplaySize :: Medium =>{
                    let res = resolution.medium;
                    window.resolution.set(res.x, res.y);
                }
                DisplaySize :: Large =>{
                    let res = resolution.large;
                    window.resolution.set(res.x, res.y);
                }
            }
        }
    }
}