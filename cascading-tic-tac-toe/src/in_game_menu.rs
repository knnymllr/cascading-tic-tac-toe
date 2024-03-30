use bevy::prelude::*;

use crate::{GameScreenTag, MenuState, PlayingState, UiTheme};

#[derive(Component)]
pub struct ReloadButton;

// Define the root node for the UI button
fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(7.0),
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexEnd,
            padding: UiRect {
                left: Val::Px(0.),
                right: Val::Px(20.),
                top: Val::Px(20.),
                bottom: Val::Px(0.),
            },
            ..Default::default()
        },
        background_color: Color::NONE.into(),
        ..Default::default()
    }
}

// Define the button node for the UI
pub fn button(theme: &Res<UiTheme>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            padding: UiRect::all(Val::Px(5.)),
            width: Val::Auto,
            height: Val::Auto,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: theme.button,
        ..Default::default()
    }
}

// Define the text node for the button
pub fn button_text(
    asset_server: &Res<AssetServer>,
    theme: &Res<UiTheme>,
    label: &str,
) -> TextBundle {
    return TextBundle {
        text: Text::from_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: theme.button_text,
            },
        ),
        ..Default::default()
    };
}

// System to set up the restart button
pub fn setup_menu_button(
    mut commands: Commands,
    theme: Res<UiTheme>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((root(), GameScreenTag)).with_children(|parent| {
        parent
            .spawn(button(&theme))
            .with_children(|parent| {
                parent.spawn(button_text(&asset_server, &theme, "Main Menu"));
            })
            .insert(ReloadButton);
    });
}

// System to handle interactions with the reload button
pub fn button_interactions(
    theme: Res<UiTheme>,
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<ReloadButton>),
    >,
    mut game_next_state: ResMut<NextState<PlayingState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>
) {
    for (interaction, mut color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = theme.button;
                next_menu_state.set(MenuState::Main);
                game_next_state.set(PlayingState::NotPlaying);
            }
            Interaction::Hovered => *color = theme.button_hovered,
            Interaction::None => *color = theme.button,
        }
    }
}