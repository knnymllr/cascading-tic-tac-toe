use bevy::prelude::*;

use crate::{GameState, PlayerTurn, PlayingState, UiTheme};

#[derive(Component)]
struct ReloadButton;

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::Local), setup_restart_button)
            .add_systems(
                OnTransition {
                    from: PlayingState::NotPlaying,
                    to: PlayingState::Local,
                },
                reload_button_interactions,
            )
            .add_systems(
                OnTransition {
                    from: PlayingState::Local,
                    to: PlayingState::NotPlaying,
                },
                reload_button_interactions,
            )
            .add_systems(OnEnter(PlayingState::NotPlaying), reload_game);
    }
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
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
        // background_color: theme.button.clone(),
        background_color: theme.button,
        ..Default::default()
    }
}

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
                // color: theme.button_text.clone(),
                color: theme.button_text,
            },
        ),
        ..Default::default()
    };
}

fn setup_restart_button(
    mut commands: Commands,
    theme: Res<UiTheme>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(root()).with_children(|parent| {
        parent
            .spawn(button(&theme))
            .with_children(|parent| {
                parent.spawn(button_text(&asset_server, &theme, "New Game"));
            })
            .insert(ReloadButton);
    });
}

fn reload_button_interactions(
    theme: Res<UiTheme>,
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<ReloadButton>),
    >,
    mut game_state: ResMut<State<PlayingState>>,
) {
    for (interaction, mut color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = theme.button;
                game_state
                    .set(Box::new(PlayingState::Local) as Box<dyn Reflect>)
                    .expect("Could not set game state.");
            }
            Interaction::Hovered => *color = theme.button_hovered,
            Interaction::None => *color = theme.button,
        }
    }
}

fn reload_game(
    mut commands: Commands,
    query: Query<Entity>,
    mut playing_state: ResMut<State<PlayingState>>,
    mut game_state: ResMut<State<GameState>>,
    mut player_turn: ResMut<State<PlayerTurn>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    playing_state
        .set(Box::new(PlayingState::NotPlaying) as Box<dyn Reflect>)
        .expect("Could not set game state.");

    if game_state.get() != &GameState::GameOngoing {
        game_state
            .set(Box::new(GameState::GameOngoing) as Box<dyn Reflect>)
            .expect("Could not set game state.");
    }

    if player_turn.get() != &PlayerTurn::X {
        player_turn
            .set(Box::new(PlayerTurn::X) as Box<dyn Reflect>)
            .expect("Could not set game state.");
    }
}
