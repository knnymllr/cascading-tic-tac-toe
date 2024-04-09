use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, in_state, IntoSystemConfigs, NextState, OnEnter, ResMut};
use bevy::prelude::*;

use crate::{board_cell_interaction_system, button_interactions, MenuState, on_cell_clicked, PlayerTurn, PlayingState, setup_board, setup_instructions, setup_menu_button, update_instruction_on_state_change};
use crate::utils::despawn_screen::despawn_screen;
use std::time::Duration;
use crate::timer::{Counter, TEXT_COLOR, TIME, time};

#[derive(Component)]
pub struct GameScreenTag;

pub struct GameScreen;

impl Plugin for GameScreen {
    fn build(&self, app: &mut App) {
        app.add_event::<crate::CellClickedEvent>()

            // setup
            .add_systems(OnEnter(PlayingState::Local), (setup_board, setup_menu_button, setup_instructions, select_player, add_text))

            // interactions
            .add_systems(Update, (
                board_cell_interaction_system,
                on_cell_clicked,
                button_interactions,
                update_instruction_on_state_change
            ).run_if(in_state(PlayingState::Local)))

            .add_systems(Update,update_time)
            // teardown
            .add_systems(OnEnter(MenuState::Main), despawn_screen::<GameScreenTag>);
    }
}


fn add_text(mut commands: Commands, asset_sever: Res<AssetServer>){
    let counter = Counter::new();
    //counter.pause();

    commands.spawn(TextBundle{
        text: Text::from_section(
            format!("{}", time(Duration::from_secs(TIME.into()))),
            TextStyle {
                 font: asset_sever.load("fonts/FiraMono-Medium.ttf"),
                 font_size: 50., 
              color: TEXT_COLOR,
               },
        ),
          style: Style{
              position_type: PositionType:: Absolute,
              ..default()
       },
     ..default()
   }).insert(counter);
 } 

fn update_time(mut query: Query<(&mut Text, &mut Counter)>, os_time: Res<Time>){
    for (mut text, mut counter) in &mut query{
        if counter.paused(){
            continue;
        }
        counter.tick(os_time.delta());
        if counter.unit_just_finished(){
            text.sections[0].value = format!("{}", time(counter.duration() - Duration::from_secs_f32(counter.elapsed_secs_round())))
        }
        
    }
}

/// selects which player goes first
fn select_player(mut next_player_turn: ResMut<NextState<PlayerTurn>>) {
    next_player_turn.set(PlayerTurn::X);
}