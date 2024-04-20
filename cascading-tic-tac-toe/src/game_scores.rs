use bevy::prelude::*;

use crate::theme::theme::UiTheme;
use crate::{GameScreenTag, RoundInit};

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
pub fn setup_scores_text(mut commands: Commands, theme: Res<UiTheme>, asset_server: Res<AssetServer>, round: Res<RoundInit>) {
    
    let label = format!("X Score: {}\nO Score: {}", round.x_score, round.o_score);

    commands.spawn(root()).with_children(|parent| {
        parent
            .spawn((text(&asset_server, &theme, &label), GameScreenTag)) // Spawn text node for instruction
            .insert(ScoresText); // Add ScoresText component to the text node entity
    });
}