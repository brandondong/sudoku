use crate::rules::PuzzleRules;
use crate::Board;
use crate::Cell;
use std::num::NonZeroU8;
#[derive(Debug, PartialEq)]
pub enum SolveResult {
    NoSolution,
    UniqueSolution(Board),
    MultipleSolutions(Board),
}

pub fn solve(board: &mut Board, rules: &impl PuzzleRules) -> SolveResult {
    if !rules.is_valid(board) {
        return SolveResult::NoSolution;
    }
    // Find an empty cell.
    let index = board
        .cells
        .iter()
        .enumerate()
        .find(|(_i, &cell)| matches!(cell, Cell::Unfilled))
        .map(|e| e.0);
    let index = match index {
        None => return SolveResult::UniqueSolution(board.clone()),
        Some(v) => v,
    };
    let mut current_result = SolveResult::NoSolution;
    for guess in 1..=9 {
        board.cells[index] = Cell::Filled(NonZeroU8::new(guess).unwrap());
        let sub_result = solve(board, rules);
        match (sub_result, &current_result) {
            (SolveResult::NoSolution, _) => (),
            (SolveResult::UniqueSolution(b), SolveResult::NoSolution) => {
                current_result = SolveResult::UniqueSolution(b);
            }
            (SolveResult::UniqueSolution(b), SolveResult::UniqueSolution(_)) => {
                current_result = SolveResult::MultipleSolutions(b);
                break;
            }
            (SolveResult::UniqueSolution(_), SolveResult::MultipleSolutions(_)) => (),
            (SolveResult::MultipleSolutions(b), _) => {
                current_result = SolveResult::MultipleSolutions(b);
                break;
            }
        }
    }
    // Make sure we exit this function with the board unchanged.
    board.cells[index] = Cell::Unfilled;
    current_result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::ClassicSudoku;

    #[test]
    fn test_unique_solution() {
        // From https://raw.githubusercontent.com/maxbergmark/sudoku-solver/master/data-sets/hard_sudokus_solved.txt.
        let mut puzzle = Board::new(
            "000075400000000008080190000300001060000000034000068170204000603900000020530200000",
        );
        let solved = Board::new(
            "693875412145632798782194356357421869816957234429368175274519683968743521531286947",
        );
        assert_eq!(
            solve(&mut puzzle, &ClassicSudoku {}),
            SolveResult::UniqueSolution(solved)
        );
    }

    #[test]
    fn test_multiple_solutions() {
        let mut puzzle = Board {
            cells: [Cell::Unfilled; 81],
        };
        assert!(matches!(
            solve(&mut puzzle, &ClassicSudoku {}),
            SolveResult::MultipleSolutions(_)
        ));
    }

    #[test]
    fn test_no_solutions() {
        let mut puzzle = Board {
            cells: [Cell::Filled(NonZeroU8::new(1).unwrap()); 81],
        };
        assert!(matches!(
            solve(&mut puzzle, &ClassicSudoku {}),
            SolveResult::NoSolution
        ));
    }
}
