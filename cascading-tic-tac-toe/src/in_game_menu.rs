use bevy::prelude::*;

use crate::{GameScreenTag, GameState, MenuState, PlayingState, RoundState};
use crate::theme::theme::UiTheme;

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
pub fn button_game(theme: &Res<UiTheme>) -> ButtonBundle {
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
pub fn button_text_game(
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
            .spawn(button_game(&theme))
            .with_children(|parent| {
                parent.spawn(button_text_game(&asset_server, &theme, "Main Menu"));
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
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_playing_state: ResMut<NextState<PlayingState>>,
    mut next_round_state: ResMut<NextState<RoundState>>,
) {
    for (interaction, mut color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = theme.button;
                next_menu_state.set(MenuState::Main);
                next_round_state.set(RoundState::NotPlaying);
                next_playing_state.set(PlayingState::NotPlaying);
                next_game_state.set(GameState::Reset);
            }
            Interaction::Hovered => *color = theme.button_hovered,
            Interaction::None => *color = theme.button,
        }
    }
}