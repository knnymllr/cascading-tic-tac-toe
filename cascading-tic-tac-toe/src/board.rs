use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    CellState, GameScreenTag, GameState, GridCell, Player, PlayerTurn, RoundCount, StateWrapper,
};

/// Event triggered when a cell is clicked
#[derive(Event)]
pub struct CellClickedEvent {
    entity: Entity,
}

/// Resource containing UI theme settings
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

/// System for handling board cell interaction
pub fn board_cell_interaction_system(
    theme: Res<UiTheme>,
    mut send_cell_clicked: EventWriter<CellClickedEvent>,
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &GridCell, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    game_state: ResMut<State<GameState>>,
) {
    for (interaction, mut color, cell, entity) in buttons.iter_mut() {
        if cell.state != CellState::Valid || game_state.clone() != GameState::GameOngoing {
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
    mut cell_query: Query<(&mut GridCell, &Children)>,
    mut cell_text_query: Query<&mut Text>,
    player_turn_state: ResMut<State<PlayerTurn>>,
    player_turn_next_state: ResMut<NextState<PlayerTurn>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let mut state = StateWrapper {
        current: player_turn_state.clone(),
        next: player_turn_next_state,
    };

    let movement_sound = asset_server.load("sounds/Crush8-Bit.ogg");

    for event in events.read() {
        let (mut cell, children) = cell_query
            .get_mut(event.entity)
            .expect("on_cell_clicked: Cell not found.");

        audio.play(movement_sound.clone());
        update_cell_state(&mut cell, &player_turn_state.get());
        update_cell_text(&mut cell_text_query, children, &player_turn_state.get());
        update_player_turn(&mut state);
    }
}

/// Updates the state of the clicked cell based on the current player turn
fn update_cell_state(cell: &mut Mut<GridCell>, player_turn: &PlayerTurn) {
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
    // Define the style for the main border node
    NodeBundle {
        style: Style {
            // Set the width to auto
            width: Val::Auto,
            // Set the height to auto
            height: Val::Auto,
            // Add a border with 2 pixels width
            border: UiRect::all(Val::Px(2.0)),
            // Set the flex direction to column-reverse
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        // Set the background color of the main border node to the border color defined in the theme
        background_color: theme.border,
        ..Default::default()
    }
}

pub fn square_row() -> NodeBundle {
    // Define the style for a square row node
    NodeBundle {
        style: Style {
            // Set the width to auto
            width: Val::Auto,
            // Set the height to auto
            height: Val::Auto,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn square_border(theme: &Res<UiTheme>) -> NodeBundle {
    // Define the style for a square border node
    NodeBundle {
        style: Style {
            // Set the width to 50 pixels
            width: Val::Px(50.0),
            // Set the height to 50 pixels
            height: Val::Px(50.0),
            // Add a border with 2 pixels width
            border: UiRect::all(Val::Px(2.0)),
            ..Default::default()
        },
        // Set the background color of the square border node to the border color defined in the theme
        background_color: theme.border,
        ..Default::default()
    }
}

pub fn menu_background(theme: &Res<UiTheme>) -> NodeBundle {
    // Define the style for the menu background node
    NodeBundle {
        style: Style {
            // Set the width to 100% of the parent's width
            width: Val::Percent(100.0),
            // Set the height to 100% of the parent's height
            height: Val::Percent(100.0),
            // Align items to the center
            align_items: AlignItems::Center,
            // Justify content to the center
            justify_content: JustifyContent::Center,
            // Set the flex direction to column-reverse
            flex_direction: FlexDirection::ColumnReverse,
            // Add padding of 5 pixels to all sides
            padding: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        // Set the background color of the menu background node to the menu color defined in the theme
        background_color: theme.menu,
        ..Default::default()
    }
}

pub fn button(theme: &Res<UiTheme>) -> ButtonBundle {
    // Define the button bundle with styles and properties
    ButtonBundle {
        style: Style {
            // Set the width to 100% of the parent's width
            width: Val::Percent(100.0),
            // Set the height to 100% of the parent's height
            height: Val::Percent(100.0),
            // Justify content to the center
            justify_content: JustifyContent::Center,
            // Align items to the center
            align_items: AlignItems::Center,
            ..Default::default()
        },
        // Set the background color of the button to the button color defined in the theme
        background_color: theme.button,
        ..Default::default()
    }
}

pub fn button_text(
    asset_server: &Res<AssetServer>,
    theme: &Res<UiTheme>,
    label: &str,
) -> TextBundle {
    // Define the text bundle with styles and properties
    TextBundle {
        text: Text::from_section(
            label,
            TextStyle {
                // Load the FiraSans-Bold font from the asset server
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                // Set the font size to 30 pixels
                font_size: 30.0,
                // Set the color of the text to the button text color defined in the theme
                color: theme.button_text,
            },
        ),
        ..Default::default()
    }
}

fn generate_invalid_cells(n: u32, list: &mut Vec<u32>) {
    let cols = n + 3;
    for current_n in 1..=n {
        let mut x;
        let mut y;
        for i in 0..2 * current_n {
            x = i;
            y = current_n + 2;

            list.push(x * cols + y);
        }
        for i in 0..current_n {
            x = (2 * current_n) + 1;
            y = i;
            list.push(x * cols + y);
            x = (2 * current_n) + 2;
            list.push(x * cols + y);
        }
    }
}

pub fn setup_board(
    mut commands: Commands,
    theme: Res<UiTheme>,
    asset_server: Res<AssetServer>,
    round_count: Res<RoundCount>,
) {
    let n = round_count.get_current();
    let mut invalid_cells = Vec::new();
    generate_invalid_cells(n, &mut invalid_cells);

    // Spawn the root node with children
    commands
        .spawn((root(&theme), GameScreenTag))
        .with_children(|parent| {
            // Spawn the main border node with children
            parent.spawn(main_border(&theme)).with_children(|parent| {
                // Loop through rows
                for row_index in (0..2 * n + 3).rev() {
                    // Spawn the square row node with children
                    parent.spawn(square_row()).with_children(|parent| {
                        // Loop through columns
                        for column_index in 0..n + 3 {
                            // Calculate the cell ID
                            let cell_id = (n + 3) * row_index + (column_index + 1) - 1;
                            // println!("{} {} = {}", row_index, column_index, cell_id);

                            if invalid_cells.contains(&cell_id) {
                                // Spawn the square border node with children
                                parent.spawn(square_border(&theme)).with_children(|parent| {
                                    // Spawn the button node with children
                                    parent
                                        .spawn(button(&theme))
                                        .with_children(|parent| {
                                            // Spawn the button text node
                                            parent.spawn(button_text(&asset_server, &theme, "-"));
                                        })
                                        // Insert the GridCell component
                                        .insert(GridCell {
                                            cell_id,
                                            state: CellState::Invalid,
                                        });
                                });
                            } else {
                                // Spawn the square border node with children
                                parent.spawn(square_border(&theme)).with_children(|parent| {
                                    // Spawn the button node with children
                                    parent
                                        .spawn(button(&theme))
                                        .with_children(|parent| {
                                            // Spawn the button text node
                                            parent.spawn(button_text(&asset_server, &theme, ""));
                                        })
                                        // Insert the GridCell component
                                        .insert(GridCell {
                                            cell_id,
                                            state: CellState::Valid,
                                        });
                                });
                            }
                        }
                    });
                }
            });
        });
}
