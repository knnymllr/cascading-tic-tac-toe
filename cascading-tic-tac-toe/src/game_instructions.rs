use bevy::prelude::*;

use crate::{GameScreenTag, GameState, Player, PlayerTurn, UiTheme};

#[derive(Component)]
pub struct InstructionText;

// Define the root node for the UI instruction text
fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(7.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            padding: UiRect {
                left: Val::Px(0.),
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

// System to set up the game instructions
pub fn setup_instructions(mut commands: Commands, theme: Res<UiTheme>, asset_server: Res<AssetServer>) {
    commands.spawn(root()).with_children(|parent| {
        parent
            .spawn((text(&asset_server, &theme, "Test"), GameScreenTag)) // Spawn text node for instruction
            .insert(InstructionText); // Add InstructionText component to the text node entity
    });
}

// System to update the game instructions based on state changes
pub fn update_instruction_on_state_change(
    player_turn_state: Res<State<PlayerTurn>>,
    game_state: Res<State<GameState>>,
    mut instructions: Query<&mut Text, With<InstructionText>>,
) {
    // If player turn changes, update instruction text accordingly
    if player_turn_state.is_changed() {
        let next_text = match player_turn_state.clone() {
            PlayerTurn::X => "Player's turn: X",
            PlayerTurn::O => "Player's turn: O",
        };
        let mut ui_text = instructions.single_mut();
        ui_text.sections[0].value = next_text.to_string();
    }

    // If game state changes, update instruction text accordingly
    if game_state.is_changed() {
        let mut ui_text = instructions.single_mut();

        match game_state.get() {
            &GameState::Won(Player::X) => ui_text.sections[0].value = "X Won!!!".to_string(),
            &GameState::Won(Player::O) => ui_text.sections[0].value = "O Won!!!".to_string(),
            &GameState::Draw => ui_text.sections[0].value = "Draw :-(".to_string(),
            &GameState::GameOngoing => (),
        }
    }
}