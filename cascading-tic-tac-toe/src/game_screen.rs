use crate::utils::despawn_screen::despawn_screen;
use crate::{
    board_cell_interaction_system, button_interactions, on_cell_clicked, setup_board,
    setup_instructions, setup_menu_button, setup_scores_text, update_instruction_on_state_change,
    GameState, PlayerTag, PlayerTurn, PlayingState, RoundInit, RoundState, WinningLogicPlugin,
};

use crate::timer::{time, Counter, TEXT_COLOR, TIME};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use bevy::prelude::{in_state, Component, IntoSystemConfigs, NextState, OnEnter, ResMut};
use std::time::Duration;

/// Component that stores the gameboard screen to spawn/despawn
#[derive(Component)]
pub struct GameScreenTag;

/// Defines a struct for a Plugin for app creation in main.rs
pub struct GameScreen;

/// A Plugin for use in primary app creation in main.rs
impl Plugin for GameScreen {
    fn build(&self, app: &mut App) {
        app.add_event::<crate::CellClickedEvent>()
            // setup
            .insert_resource(RoundInit::new(3))
            .insert_state(GameState::NotPlaying)
            .insert_state(RoundState::NotUpdating)
            .insert_state(PlayerTurn::X)
            .add_plugins(WinningLogicPlugin)
            .add_systems(
                OnEnter(GameState::LoadingNewGame),
                (
                    setup_board,
                    setup_menu_button,
                    setup_instructions,
                    setup_scores_text,
                    setup_timer_text,
                    loading_finished,
                )
                    .chain(),
            )
            .add_systems(
                OnEnter(RoundState::UpdatingRound),
                (
                    despawn_screen::<GameScreenTag>,
                    (
                        setup_board,
                        setup_menu_button,
                        setup_instructions,
                        setup_scores_text,
                        setup_timer_text,
                        loading_finished,
                    )
                        .chain(),
                ),
            )
            // interactions
            .add_systems(Update, button_interactions)
            .add_systems(
                Update,
                (
                    board_cell_interaction_system,
                    on_cell_clicked,
                    update_instruction_on_state_change,
                    update_time,
                )
                    // .chain()
                    .run_if(in_state(GameState::GameOngoing)),
            )
            .add_systems(
                OnEnter(GameState::Won(PlayerTag::X)),
                (
                    despawn_screen::<GameScreenTag>,
                    (
                        setup_board,
                        setup_menu_button,
                        setup_instructions,
                        update_instruction_on_state_change,
                        setup_scores_text,
                        setup_timer_text,
                        loading_finished,
                    )
                        .chain(),
                ),
            )
            .add_systems(
                OnEnter(GameState::Won(PlayerTag::O)),
                (
                    despawn_screen::<GameScreenTag>,
                    (
                        setup_board,
                        setup_menu_button,
                        setup_instructions,
                        update_instruction_on_state_change,
                        setup_scores_text,
                        setup_timer_text,
                        loading_finished,
                    )
                        .chain(),
                ),
            )
            // teardown
            .add_systems(OnExit(PlayingState::Local), despawn_screen::<GameScreenTag>)
            //restarting game
            .add_systems(
                OnEnter(GameState::RestartingGame),
                (
                    despawn_screen::<GameScreenTag>,
                    restart_game,
                    finished_restarting,
                )
                    .chain(),
            )
            .add_systems(
                OnEnter(GameState::NotPlaying),
                (despawn_screen::<GameScreenTag>, restart_game),
            );
    }
}

/// A system that handles state change after all setup systems have run
fn loading_finished(
    mut next_round_state: ResMut<NextState<RoundState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_round_state.set(RoundState::NotUpdating);
    next_game_state.set(GameState::GameOngoing);
}

/// A system that triggers restarting the game by changing states
fn restart_game(
    mut round_init: ResMut<RoundInit>,
    mut next_player_turn: ResMut<NextState<PlayerTurn>>,
) {
    *round_init = RoundInit::new(3);
    next_player_turn.set(PlayerTurn::X);
}

/// A system that flags when restarting the game has completed
fn finished_restarting(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::LoadingNewGame);
}

/// A system that handles the timer countdown in the gamescreen
fn setup_timer_text(mut commands: Commands, asset_sever: Res<AssetServer>) {
    let counter = Counter::new();
    //counter.pause();

    commands
        .spawn((
            TextBundle {
                text: Text::from_section(
                    format!("{}", time(Duration::from_secs(TIME.into()))),
                    TextStyle {
                        font: asset_sever.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 50.,
                        color: TEXT_COLOR,
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            GameScreenTag,
        ))
        .insert(counter);
}

/// A system that updates the current value of the countdown timer
fn update_time(mut query: Query<(&mut Text, &mut Counter)>, os_time: Res<Time>) {
    for (mut text, mut counter) in &mut query {
        if counter.paused() {
            continue;
        }
        counter.tick(os_time.delta());
        if counter.unit_just_finished() {
            text.sections[0].value = format!(
                "{}",
                time(counter.duration() - Duration::from_secs_f32(counter.elapsed_secs_round()))
            )
        }
    }
}
