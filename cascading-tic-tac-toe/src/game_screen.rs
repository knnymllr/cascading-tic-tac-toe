use crate::utils::despawn_screen::despawn_screen;
use crate::{
    board_cell_interaction_system, button_interactions, on_cell_clicked, setup_board,
    setup_instructions, setup_menu_button, spawn_scores_text, update_instruction_on_state_change,
    update_scores_on_state_change, update_scores_text_on_state_change, GameState, MenuState,
    PlayerTurn, PlayingState, RoundInit, RoundState, WinningLogicPlugin,
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
            .insert_resource(RoundInit::new(2, 3))
            .insert_state(GameState::GameOngoing)
            .insert_state(PlayerTurn::X)
            .insert_state(RoundState::NotPlaying)
            .add_plugins(WinningLogicPlugin)
            .add_systems(
                OnEnter(PlayingState::Local),
                (
                    setup_board,
                    setup_menu_button,
                    setup_instructions,
                    spawn_scores_text,
                    begin_round,
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
fn begin_round(mut next_player_turn: ResMut<NextState<PlayerTurn>>, mut round_init: ResMut<RoundInit>) {
    next_player_turn.set(PlayerTurn::X);
    *round_init = RoundInit::new(2,3);
}
