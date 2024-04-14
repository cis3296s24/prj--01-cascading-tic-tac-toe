use bevy::prelude::*;

use crate::{CellState, GameState, GridCell, PlayerTag, RoundInit, RoundState};
/// Plugin for handling winning logic in tic-tac-toe game
pub struct WinningLogicPlugin;

impl Plugin for WinningLogicPlugin {
    fn build(&self, app: &mut App) {
        // Add the system for checking winning conditions
        app.add_systems(
            Update,
            (is_round_over, is_game_over.after(is_round_over))
                .run_if(in_state(GameState::GameOngoing)),
        );
    }
}

fn is_game_over(round: Res<RoundInit>, mut next_game_state: ResMut<NextState<GameState>>, mut next_round_state: ResMut<NextState<RoundState>>) {
    if round.x_score >= round.target {
        next_round_state.set(RoundState::NotUpdating);
        next_game_state.set(GameState::Won(PlayerTag::X));
    }
    if round.o_score >= round.target {
        next_round_state.set(RoundState::NotUpdating);
        next_game_state.set(GameState::Won(PlayerTag::O));
    }
}

/// System for checking if the game is over
fn is_round_over(
    cells_query: Query<&GridCell>,
    mut update_round: ResMut<NextState<RoundState>>,
    mut round_init: ResMut<RoundInit>,
) {
    // Collect the states of all cells into a vector
    let n = round_init.round_count;
    let grid_size = (2 * n + 3) * (n + 3);

    let mut cells = vec![CellState::Valid; grid_size as usize];
    for cell in cells_query.iter() {
        cells[cell.cell_id as usize] = cell.state.clone();
    }

    // Check if player X has won
    while is_winner(&cells, n, PlayerTag::X, &mut round_init.game_combinations) {
        round_init.x_score += 1;
        // round_init.round_count += 1;
        update_round.set(RoundState::UpdatingRound)
    }

    // Check if player O has won
    while is_winner(&cells, n, PlayerTag::O, &mut round_init.game_combinations) {
        round_init.o_score += 1;
        // round_init.round_count += 1;
        update_round.set(RoundState::UpdatingRound)
    }

    // TODO: REFACTOR
    // TODO: Check if the game is a draw, optimize to exclude cells that cannot be combos
    if is_draw(&cells) {
        // update_winner.set(GameState::Draw);
    }
}

fn has_two_tuples(
    game_combinations: &mut Vec<[(u32, u32); 3]>,
    winning_combination: &[(u32, u32); 3],
) -> bool {
    for combination in game_combinations {
        let mut count = 0;
        for tuple in winning_combination {
            if combination.iter().any(|comb_tuple| *comb_tuple == *tuple) {
                count += 1;
                if count >= 2 {
                    return true;
                }
            }
        }
    }
    false
}

// fn is_opposite(
//     game_combinations: &mut Vec<[(u32, u32); 3]>,
//     winning_combination: &[(u32, u32); 3],
// ) -> bool {
//     for combination in game_combinations {
//         for tuple in winning_combination {
//             if combination.iter().any(|comb_tuple| *comb_tuple == *tuple) {
//                 if has_opposite(combination, winning_combination, tuple) {
//                     return true;
//                 }
//             }
//         }
//     }
//     false
// }

// fn has_opposite(combination: &[(u32, u32); 3], proposed: &[(u32, u32); 3], common_tuple: &(u32,u32)) -> bool {
//     false
// }

/// Check if a player has won
fn is_winner(
    cells: &Vec<CellState>,
    n: u32,
    player: PlayerTag,
    game_combinations: &mut Vec<[(u32, u32); 3]>,
) -> bool {
    let state = CellState::Filled(player);

    let mut winning_combinations: Vec<[(u32, u32); 3]> = Vec::new();
    generate_winning_combinations(n, &mut winning_combinations);
    // Iterate over all winning combinations
    for winning_combination in winning_combinations {
        let mut all_match = true;

        if game_combinations.contains(&winning_combination)
            || has_two_tuples(game_combinations, &winning_combination)
        {
            continue; // Skip to the next combination
        }

        for cell in winning_combination.iter() {
            let index = get_index(cell.0, cell.1, n + 3);

            if cells[index] != state {
                all_match = false;
                break;
            }
        }

        if all_match {
            game_combinations.push(winning_combination);
            return true;
        }
    }

    return false;
}

fn generate_winning_combinations(round_init: u32, winners: &mut Vec<[(u32, u32); 3]>) {
    for n in 0..=round_init {
        // horizontal
        winners.push([(2 * n, n), (2 * n, n + 1), (2 * n, n + 2)]);
        winners.push([(2 * n + 1, n), (2 * n + 1, n + 1), (2 * n + 1, n + 2)]);
        winners.push([(2 * n + 2, n), (2 * n + 2, n + 1), (2 * n + 2, n + 2)]);
        // vertical
        winners.push([(2 * n, n), (2 * n + 1, n), (2 * n + 2, n)]);
        winners.push([(2 * n, n + 1), (2 * n + 1, n + 1), (2 * n + 2, n + 1)]);
        winners.push([(2 * n, n + 2), (2 * n + 1, n + 2), (2 * n + 2, n + 2)]);
        // diagonals
        winners.push([(2 * n, n), (2 * n + 1, n + 1), (2 * n + 2, n + 2)]);
        winners.push([(2 * n, n + 2), (2 * n + 1, n + 1), (2 * n + 2, n)]);
        if n > 0 {
            // reach-back
            winners.push([(2 * n - 2, n), (2 * n - 1, n + 1), (2 * n, n + 2)]);
            winners.push([(2 * n - 1, n), (2 * n, n + 1), (2 * n + 1, n + 2)]);
            winners.push([(2 * n - 1, n - 1), (2 * n, n), (2 * n + 1, n + 1)]);
            winners.push([(2 * n, n - 1), (2 * n + 1, n), (2 * n + 2, n + 1)]);
            winners.push([(2 * n - 1, n), (2 * n, n), (2 * n + 1, n)]);
            winners.push([(2 * n - 1, n + 1), (2 * n, n + 1), (2 * n + 1, n + 1)]);
        }
    }
}

fn get_index(x: u32, y: u32, num_cols: u32) -> usize {
    let index = (x * num_cols) + y;
    index as usize // Cast to usize if needed
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
    #[test_case(vec![CellState::Filled(PlayerTag::X), CellState::Filled(PlayerTag::O)], true)]
    #[test_case(vec![CellState::Filled(PlayerTag::X), CellState::Valid], false)]
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
