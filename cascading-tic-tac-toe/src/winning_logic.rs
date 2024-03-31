use bevy::prelude::*;

use crate::{CellState, GameState, GridCell, Player, RoundCount};

const WINNING_COMBINATIONS: [[(u32, u32); 3]; 8] = [
    // horizontal
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    // vertical
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    // diagonals
    [(0, 0), (1, 1), (2, 2)],
    [(2, 0), (1, 1), (0, 2)],
    // reach-back
    // [(0,), (3,), (6,)],
    // [(1,), (4,), (7,)],
    // [(2,), (5,), (8,)],
    // [(2,), (5,), (8,)],
];
/// Plugin for handling winning logic in tic-tac-toe game
pub struct WinningLogicPlugin;

impl Plugin for WinningLogicPlugin {
    fn build(&self, app: &mut App) {
        // Add the system for checking winning conditions
        app.add_systems(
            Update,
            is_game_over.run_if(in_state(GameState::GameOngoing)),
        );
    }
}

/// System for checking if the game is over
pub fn is_game_over(
    cells_query: Query<&GridCell>,
    mut update_winner: ResMut<NextState<GameState>>,
    // mut round_count: ResMut<RoundCount>,
    round_count: ResMut<RoundCount>,
) {
    // Collect the states of all cells into a vector
    let n = round_count.get_current();
    let grid_size = (2 * n + 3) * (n + 3);

    let mut cells = vec![CellState::Valid; grid_size as usize];
    for cell in cells_query.iter() {
        cells[cell.cell_id as usize] = cell.state.clone();
    }

    // Check if player X has won
    if is_winner(&cells, n, Player::X) {
        update_winner.set(GameState::Won(Player::X))
    }
    // Check if player O has won
    if is_winner(&cells, n, Player::O) {
        update_winner.set(GameState::Won(Player::O))
    }
    // Check if the game is a draw
    if is_draw(&cells) {
        update_winner.set(GameState::Draw)
    }
}

/// Check if a player has won
fn is_winner(cells: &Vec<CellState>, n: u32, player: Player) -> bool {
    let state = CellState::Filled(player);
    // Iterate over all winning combinations
    for winning_combination in WINNING_COMBINATIONS {
        let mut all_match = true;

        for cell in winning_combination.iter() {
            let index = get_index(cell.0, cell.1, n + 3);

            if cells[index] != state {
                all_match = false;
                break;
            }
        }

        if all_match {
            return true;
        }
    }

    return false;
}

fn get_index(x: u32, y: u32, num_cols: u32) -> usize {
    let index = (y * num_cols) + x;
    index as usize// Cast to usize if needed
  }

/// Check if the game is a draw
///! WILL BE REFACTORED
fn is_draw(cells: &Vec<CellState>) -> bool {
    // If there are no Valid cells left, the game is a draw
    !cells.iter().any(|element| *element == CellState::Valid)
}

/// Unit tests for the winning logic functions
#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    /// Test cases for the `is_draw` function
    #[test_case(vec![CellState::Filled(Player::X), CellState::Filled(Player::O)], true)]
    #[test_case(vec![CellState::Filled(Player::X), CellState::Valid], false)]
    fn test_is_draw(input: Vec<CellState>, expected: bool) {
        assert_eq!(is_draw(&input), expected);
    }

    // /// Test cases for the `is_winner` function
    // #[test_case(vec![CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Valid, CellState::Valid, CellState::Valid, CellState::Valid, CellState::Valid, CellState::Valid], Player::X, true)]
    // #[test_case(vec![CellState::Valid, CellState::Valid, CellState::Valid, CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Valid, CellState::Valid, CellState::Valid], Player::X, true)]
    // #[test_case(vec![CellState::Valid, CellState::Valid, CellState::Valid, CellState::Valid, CellState::Valid, CellState::Valid, CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Filled(Player::X)], Player::X, true)]
    // #[test_case(vec![CellState::Filled(Player::X), CellState::Valid, CellState::Valid, CellState::Filled(Player::X), CellState::Valid, CellState::Valid, CellState::Filled(Player::X), CellState::Valid, CellState::Valid], Player::X, true)]
    // #[test_case(vec![CellState::Filled(Player::X), CellState::Filled(Player::O), CellState::Filled(Player::X), CellState::Valid, CellState::Valid, CellState::Valid, CellState::Valid, CellState::Valid, CellState::Valid], Player::X, false)]
    // fn test_is_winner(input: Vec<CellState>, player: Player, expected: bool) {
    //     assert_eq!(is_winner(&input, player), expected);
    // }
}
