use crate::utils::despawn_screen::despawn_screen;
use crate::{
    board_cell_interaction_system, button_interactions, on_cell_clicked, setup_board,
    setup_instructions, setup_menu_button, setup_scores_text, update_instruction_on_state_change,
    update_scores_on_state_change, GameState, PlayerTurn, RoundInit, RoundState,
    WinningLogicPlugin,
};

use crate::timer::{time, Counter, TEXT_COLOR, TIME};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use bevy::prelude::{in_state, Component, IntoSystemConfigs, NextState, OnEnter, ResMut};
use std::time::Duration;

#[derive(Component)]
pub struct GameScreenTag;

pub struct GameScreen;

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
            // interactions
            .add_systems(
                Update,
                (
                    board_cell_interaction_system,
                    on_cell_clicked,
                    button_interactions,
                    update_instruction_on_state_change,
                    update_time,
                )
                    .run_if(in_state(GameState::GameOngoing)),
            )
            .add_systems(
                Update,
                (update_scores_on_state_change,).run_if(in_state(RoundState::UpdatingRound)),
            )
            // teardown
            .add_systems(
                OnExit(GameState::GameOngoing),
                despawn_screen::<GameScreenTag>,
            )
            //restarting game
            .add_systems(OnEnter(GameState::LoadingNewGame), load_new_game);
    }
}

fn loading_finished(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::GameOngoing);
}

//restart the game by changing states
fn load_new_game(
    mut next_player_turn: ResMut<NextState<PlayerTurn>>,
    mut round_init: ResMut<RoundInit>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_player_turn.set(PlayerTurn::X);
    *round_init = RoundInit::new(3);
    next_game_state.set(GameState::LoadingNewGame);
}

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
