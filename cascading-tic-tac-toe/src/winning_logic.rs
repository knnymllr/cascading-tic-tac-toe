use bevy::prelude::*;

use crate::{CellState, GameState, Player, TicTacToeCell};

const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
    // horizontal
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    // vertical
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    // diagonals
    [0, 4, 8],
    [2, 4, 6],
];

/// Plugin for handling winning logic in tic-tac-toe game
pub struct WinningLogicPlugin;

impl Plugin for WinningLogicPlugin {
    fn build(&self, app: &mut App) {
        // Add the system for checking winning conditions
        app.add_systems(Update, is_game_over.run_if(in_state(GameState::GameOngoing)));
    }
}

/// System for checking if the game is over
pub fn is_game_over(
    cells_query: Query<&TicTacToeCell>,
    mut update_winner: ResMut<NextState<GameState>>,
) {
    // Collect the states of all cells into a vector
    let mut cells = vec![CellState::Empty; 9];
    for cell in cells_query.iter() {
        cells[cell.cell_id as usize] = cell.state.clone();
    }

    // Check if player X has won
    if is_winner(&cells, Player::X) {
        update_winner.set(GameState::Won(Player::X))
    }
    // Check if player O has won
    if is_winner(&cells, Player::O) {
        update_winner.set(GameState::Won(Player::O))
    }
    // Check if the game is a draw
    if is_draw(&cells) {
        update_winner.set(GameState::Draw)
    }
}

/// Check if a player has won
fn is_winner(cells: &Vec<CellState>, player: Player) -> bool {
    let state = CellState::Filled(player);
    // Iterate over all winning combinations
    for winning_combination in WINNING_COMBINATIONS {
        // Check if the player has filled all cells in the combination
        if cells[winning_combination[0]] == state
            && cells[winning_combination[1]] == state
            && cells[winning_combination[2]] == state
        {
            return true;
        }
    }

    return false;
}

/// Check if the game is a draw
fn is_draw(cells: &Vec<CellState>) -> bool {
    // If there are no empty cells left, the game is a draw
    !cells.iter().any(|element| *element == CellState::Empty)
}

/// Unit tests for the winning logic functions
#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    /// Test cases for the `is_draw` function
    #[test_case(vec![CellState::Filled(Player::X), CellState::Filled(Player::O)], true)]
    #[test_case(vec![CellState::Filled(Player::X), CellState::Empty], false)]
    fn test_is_draw(input: Vec<CellState>, expected: bool) {
        assert_eq!(is_draw(&input), expected);
    }

    /// Test cases for the `is_winner` function
    #[test_case(vec![CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty], Player::X, true)]
    #[test_case(vec![CellState::Empty, CellState::Empty, CellState::Empty, CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Empty, CellState::Empty, CellState::Empty], Player::X, true)]
    #[test_case(vec![CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Filled(Player::X)], Player::X, true)]
    #[test_case(vec![CellState::Filled(Player::X), CellState::Empty, CellState::Empty, CellState::Filled(Player::X), CellState::Empty, CellState::Empty, CellState::Filled(Player::X), CellState::Empty, CellState::Empty], Player::X, true)]
    #[test_case(vec![CellState::Filled(Player::X), CellState::Filled(Player::O), CellState::Filled(Player::X), CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty], Player::X, false)]
    fn test_is_winner(input: Vec<CellState>, player: Player, expected: bool) {
        assert_eq!(is_winner(&input, player), expected);
    }
}