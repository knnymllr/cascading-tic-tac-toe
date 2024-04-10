use bevy::prelude::*;

use crate::theme::theme::UiTheme;
use crate::{GameScreenTag, RoundInit, RoundState};

#[derive(Component)]
pub struct ScoresText;

// Define the root node for the UI scores text
fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(10.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: UiRect {
                left: Val::Px(20.),
                right: Val::Px(0.),
                top: Val::Px(70.),
                bottom: Val::Px(0.),
            },
            ..Default::default()
        },
        background_color: Color::NONE.into(),
        ..Default::default()
    }
}

// Function to create the text node for UI instruction
fn text(asset_server: &Res<AssetServer>, theme: &Res<UiTheme>, label: &str) -> TextBundle {
    return TextBundle {
        text: Text::from_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: theme.button_text,
            },
        ),
        ..Default::default()
    };
}

// System to set up the game scores
pub fn spawn_scores_text(mut commands: Commands, theme: Res<UiTheme>, asset_server: Res<AssetServer>, round: Res<RoundInit>) {
    
    let label = format!("X Score: {}\nO Score: {}", round.x_score, round.o_score);

    commands.spawn(root()).with_children(|parent| {
        parent
            .spawn((text(&asset_server, &theme, &label), GameScreenTag)) // Spawn text node for instruction
            .insert(ScoresText); // Add ScoresText component to the text node entity
    });
}

pub fn update_scores_on_state_change(mut round: ResMut<RoundInit>, round_state: Res<State<RoundState>>, mut next_round_state: ResMut<NextState<RoundState>>) {
    if round_state.is_changed() {
        match round_state.get() {
            &RoundState::UpdatingX => {
                round.x_score += 1;
                next_round_state.set(RoundState::Playing);
            },
            &RoundState::UpdatingO => {
                round.o_score += 1;
                next_round_state.set(RoundState::Playing);
            },
            &RoundState::NotPlaying => {
                round.x_score = 0;
                round.o_score = 0;
                round.game_combinations = Vec::new();
            },
            &RoundState::Playing => (),
        }
    }
}

// System to update score text on state change
pub fn update_scores_text_on_state_change(
    round: Res<RoundInit>,
    round_state: Res<State<RoundState>>,
    mut scores_text
    : Query<&mut Text, With<ScoresText>>,
) {
    // If game state changes, update scores text accordingly
    if round_state.is_changed() {
        let mut ui_text = scores_text.single_mut();
        let new_scores_text = format!("X Score: {}\nO Score: {}", round.x_score, round.o_score);

        match round_state.get() {
            &RoundState::UpdatingX => (),
            &RoundState::UpdatingO => (),
            // &RoundState::NotPlaying => ui_text.sections[0].value = "".to_string(),
            &RoundState::NotPlaying => ui_text.sections[0].value = new_scores_text,
            &RoundState::Playing => ui_text.sections[0].value = new_scores_text,
        }
    }
}