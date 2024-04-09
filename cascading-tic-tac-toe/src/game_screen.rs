use crate::utils::despawn_screen::despawn_screen;
use crate::{
    board_cell_interaction_system, button_interactions, on_cell_clicked, setup_board,
    setup_instructions, setup_menu_button, spawn_scores_text, update_instruction_on_state_change,
    update_scores_on_state_change, update_scores_text_on_state_change, MenuState, PlayerTurn,
    PlayingState,
};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, Component, IntoSystemConfigs, NextState, OnEnter, ResMut};

#[derive(Component)]
pub struct GameScreenTag;

pub struct GameScreen;

impl Plugin for GameScreen {
    fn build(&self, app: &mut App) {
        app.add_event::<crate::CellClickedEvent>()
            // setup
            .add_systems(
                OnEnter(PlayingState::Local),
                (
                    setup_board,
                    setup_menu_button,
                    setup_instructions,
                    spawn_scores_text,
                    select_player,
                ),
            )
            // interactions
            .add_systems(
                Update,
                (
                    board_cell_interaction_system,
                    on_cell_clicked,
                    button_interactions,
                    update_instruction_on_state_change,
                    update_scores_on_state_change,
                    update_scores_text_on_state_change,
                )
                    .run_if(in_state(PlayingState::Local)),
            )
            // teardown
            .add_systems(OnEnter(MenuState::Main), despawn_screen::<GameScreenTag>);
    }
}

/// selects which player goes first
fn select_player(mut next_player_turn: ResMut<NextState<PlayerTurn>>) {
    next_player_turn.set(PlayerTurn::X);
}
