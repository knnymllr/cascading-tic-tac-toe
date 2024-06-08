use bevy::prelude::*;

use crate::{CellState, GameScreenTag, GameState, Player, PlayerTurn, StateWrapper, TicTacToeCell};
use crate::theme::theme::UiTheme;
use crate::ui_components::bundles::{button_bundle, text_bundle};

/// Event triggered when a cell is clicked
#[derive(Event)]
pub struct CellClickedEvent {
    entity: Entity,
}

/// System for handling board cell interaction
pub fn board_cell_interaction_system(
    theme: Res<UiTheme>,
    mut send_cell_clicked: EventWriter<CellClickedEvent>,
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &TicTacToeCell, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    game_state: ResMut<State<GameState>>,
) {
    for (interaction, mut color, cell, entity) in buttons.iter_mut() {
        if cell.state != CellState::Empty || game_state.clone() != GameState::GameOngoing {
            return;
        }

        match *interaction {
            Interaction::Pressed => {
                send_cell_clicked.send(CellClickedEvent { entity });
                *color = theme.button;
            }
            Interaction::Hovered => *color = theme.button_hovered,
            Interaction::None => *color = theme.button,
        }
    }
}

/// System for handling cell click events
pub fn on_cell_clicked(
    mut events: EventReader<CellClickedEvent>,
    mut cell_query: Query<(&mut TicTacToeCell, &Children)>,
    mut cell_text_query: Query<&mut Text>,
    player_turn_state: ResMut<State<PlayerTurn>>,
    player_turn_next_state: ResMut<NextState<PlayerTurn>>,
) {
    let mut state = StateWrapper {
        current: player_turn_state.clone(),
        next: player_turn_next_state,
    };

    for event in events.read() {
        let (mut cell, children) = cell_query
            .get_mut(event.entity)
            .expect("on_cell_clicked: Cell not found.");

        update_cell_state(&mut cell, &player_turn_state.get());
        update_cell_text(&mut cell_text_query, children, &player_turn_state.get());
        update_player_turn(&mut state);
    }
}

/// Updates the state of the clicked cell based on the current player turn
fn update_cell_state(cell: &mut Mut<TicTacToeCell>, player_turn: &PlayerTurn) {
    cell.state = match player_turn {
        PlayerTurn::X => CellState::Filled(Player::X),
        PlayerTurn::O => CellState::Filled(Player::O),
    };
}

/// Updates the text of the clicked cell based on the current player turn
fn update_cell_text(
    cell_text_query: &mut Query<&mut Text>,
    children: &Children,
    player_turn: &PlayerTurn,
) {
    let text = match player_turn {
        PlayerTurn::X => "X",
        PlayerTurn::O => "O",
    };

    for child in children.iter() {
        if let Ok(mut cell_text) = cell_text_query.get_mut(*child) {
            cell_text.sections[0].value = text.to_string();
        }
    }
}

/// Updates the player turn state to the next player
fn update_player_turn(state: &mut StateWrapper<PlayerTurn>) {
    let next_state = match state.current {
        PlayerTurn::X => PlayerTurn::O,
        PlayerTurn::O => PlayerTurn::X,
    };
    state.next.set(next_state);
}

/// Creates the root node for the UI
pub fn root(theme: &Res<UiTheme>) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: theme.root,
        ..Default::default()
    }
}

pub fn main_border(theme: &Res<UiTheme>) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Auto,
            border: UiRect::all(Val::Px(2.0)),
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        background_color: theme.border,
        ..Default::default()
    }
}

pub fn square_row() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Auto,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn square_border(theme: &Res<UiTheme>) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(2.0)),
            ..Default::default()
        },
        background_color: theme.border,
        ..Default::default()
    }
}

pub fn setup_board(mut commands: Commands, theme: Res<UiTheme>, asset_server: Res<AssetServer>) {
    commands.spawn((root(&theme), GameScreenTag)).with_children(|parent| {
        parent
            .spawn(main_border(&theme))
            .with_children(|parent| {
                for row_index in 0..3 {
                    parent.spawn(square_row()).with_children(|parent| {
                        for column_index in 1..=3 {
                            let cell_id = 3 * row_index + column_index - 1;
                            parent
                                .spawn(square_border(&theme))
                                .with_children(|parent| {
                                    parent
                                        .spawn(button_bundle(
                                            (
                                                Val::Percent(100.0),
                                                Val::Percent(100.0),
                                                None,
                                                JustifyContent::Center,
                                                AlignItems::Center,
                                            ),
                                            theme.button,
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(text_bundle(
                                                "",
                                                &asset_server,
                                                (30.0, theme.button_text),
                                            ));
                                        })
                                        .insert(TicTacToeCell {
                                            cell_id,
                                            state: CellState::Empty,
                                        });
                                });
                        }
                    });
                }
            });
    });
}