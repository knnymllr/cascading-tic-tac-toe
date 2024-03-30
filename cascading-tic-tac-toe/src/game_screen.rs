use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, in_state, IntoSystemConfigs, NextState, OnEnter, ResMut};

use crate::{board_cell_interaction_system, button_interactions, despawn_screen, MenuState, on_cell_clicked, PlayerTurn, PlayingState, setup_board, setup_instructions, setup_menu_button, update_instruction_on_state_change};

#[derive(Component)]
pub struct GameScreenTag;

pub struct GameScreen;

impl Plugin for GameScreen {
    fn build(&self, app: &mut App) {
        app.add_event::<crate::CellClickedEvent>()

            // setup
            .add_systems(OnEnter(PlayingState::Local), (setup_board, setup_menu_button, setup_instructions, select_player))

            // interactions
            .add_systems(Update, (
                board_cell_interaction_system,
                on_cell_clicked,
                button_interactions,
                update_instruction_on_state_change
            ).run_if(in_state(PlayingState::Local)))

            // teardown
            .add_systems(OnEnter(MenuState::Main), despawn_screen::<GameScreenTag>);
    }
}

/// selects which player goes first
fn select_player(mut next_player_turn: ResMut<NextState<PlayerTurn>>) {
    next_player_turn.set(PlayerTurn::X);
}