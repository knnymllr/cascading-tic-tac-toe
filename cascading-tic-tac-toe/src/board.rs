use bevy::prelude::*;

use crate::{CellState, GameState, Player, PlayerTurn, PlayingState, StateWrapper, TicTacToeCell};

pub struct BoardPlugin;

#[derive(Event)]
pub struct CellClickedEvent {
    entity: Entity,
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTheme>()
           .add_event::<CellClickedEvent>()
           .add_systems(OnEnter(PlayingState::Local), setup_board)
           .add_systems(Update, (board_cell_interaction_system, on_cell_clicked).run_if(in_state(GameState::GameOngoing)));
    }
}

#[derive(Resource)]
pub struct UiTheme {
    pub root: BackgroundColor,
    pub border: BackgroundColor,
    pub menu: BackgroundColor,
    pub button: BackgroundColor,
    pub button_hovered: BackgroundColor,
    pub button_pressed: BackgroundColor,
    pub button_text: Color,
}

impl FromWorld for UiTheme {
    fn from_world(_: &mut World) -> Self {
        UiTheme {
            root: Color::NONE.into(),
            border: Color::rgb(0.65, 0.65, 0.65).into(),
            menu: Color::rgb(0.15, 0.15, 0.15).into(),
            button: Color::rgb(0.15, 0.15, 0.15).into(),
            button_hovered: Color::rgb(0.35, 0.75, 0.35).into(),
            button_pressed: Color::rgb(0.35, 0.75, 0.35).into(),
            button_text: Color::WHITE,
        }
    }
}

pub fn board_cell_interaction_system(
    theme: Res<UiTheme>,
    mut send_cell_clicked: EventWriter<CellClickedEvent>,
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &TicTacToeCell, Entity),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, cell, entity) in buttons.iter_mut() {
        if cell.state != CellState::Empty {
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

pub fn on_cell_clicked(
    mut events: EventReader<CellClickedEvent>,
    mut cell_query: Query<(&mut TicTacToeCell, &Children)>,
    mut cell_text_query: Query<&mut Text>,
    player_turn_state: ResMut<State<PlayerTurn>>,
    player_turn_next_state: ResMut<NextState<PlayerTurn>>,
) {
    // let player_turn = player_turn_state.get().clone();
    let player_turn = player_turn_state.get();

    let mut state = StateWrapper {
        current: player_turn_state.clone(),
        next: player_turn_next_state,
    };

    for event in events.read() {
        let (mut cell, children) = cell_query
            .get_mut(event.entity)
            .expect("on_cell_clicked: Cell not found.");

        update_cell_state(&mut cell, &player_turn);
        update_cell_text(&mut cell_text_query, children, &player_turn);
        update_player_turn(&mut state);
    }
}

fn update_cell_state(cell: &mut Mut<TicTacToeCell>, player_turn: &PlayerTurn) {
    cell.state = match player_turn {
        PlayerTurn::X => CellState::Filled(Player::X),
        PlayerTurn::O => CellState::Filled(Player::O),
    };
}

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

fn update_player_turn(state: &mut StateWrapper<PlayerTurn>) {
    let next_state = match state.current {
        PlayerTurn::X => PlayerTurn::O,
        PlayerTurn::O => PlayerTurn::X,
    };
    state.next.set(next_state);
}

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

pub fn menu_background(theme: &Res<UiTheme>) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        background_color: theme.menu,
        ..Default::default()
    }
}

pub fn button(theme: &Res<UiTheme>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
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
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                // color: theme.button_text.clone(),
                color: theme.button_text,
            },
        ),
        ..Default::default()
    };
}

pub fn setup_board(mut commands: Commands, theme: Res<UiTheme>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(root(&theme)).with_children(|parent| {
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
                                        .spawn(button(&theme))
                                        .with_children(|parent| {
                                            parent.spawn(button_text(
                                                &asset_server,
                                                &theme,
                                                "",
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
