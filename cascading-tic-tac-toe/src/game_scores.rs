use bevy::prelude::*;

use crate::theme::theme::UiTheme;
use crate::{GameScreenTag, RoundInit};

#[derive(Component)]
pub struct ScoresText;

// Define the root node for the UI instruction text
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
                top: Val::Px(20.),
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
    let mut x_score = format!("X Score: {}", round.x_score);
    let mut o_score = format!("O Score: {}", round.o_score);
    
    commands.spawn(root()).with_children(|parent| {
        parent
            .spawn((text(&asset_server, &theme, &mut x_score), GameScreenTag)) // Spawn text node for instruction
            .insert(ScoresText); // Add ScoresText component to the text node entity
        parent
            .spawn((text(&asset_server, &theme, &mut o_score), GameScreenTag)) // Spawn text node for instruction
            .insert(ScoresText); // Add ScoresText component to the text node entity
    });
}


// // System to update the game instructions based on state changes
// pub fn update_scores_on_state_change(
//     player_turn_state: Res<State<PlayerTurn>>,
//     game_state: Res<State<GameState>>,
//     mut scores_text
//     : Query<&mut Text, With<ScoresText>>,
// ) {
//     // If player turn changes, update instruction text accordingly
//     if player_turn_state.is_changed() {
//         let next_text = match player_turn_state.clone() {
//             PlayerTurn::X => "Player's turn: X",
//             PlayerTurn::O => "Player's turn: O",
//         };
//         let mut ui_text = scores_text.single_mut();
//         ui_text.sections[0].value = next_text.to_string();
//     }

//     // If game state changes, update instruction text accordingly
//     if game_state.is_changed() {
//         let mut ui_text = scores_text.single_mut();

//         match game_state.get() {
//             &GameState::Won(PlayerTag::X) => ui_text.sections[0].value = "X Won!!!".to_string(),
//             &GameState::Won(PlayerTag::O) => ui_text.sections[0].value = "O Won!!!".to_string(),
//             &GameState::Draw => (),
//             &GameState::GameOngoing => (),
//         }
//     }
// }