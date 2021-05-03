use rand::{prelude::SliceRandom, thread_rng};

use crate::rules::PuzzleRules;
use crate::Board;
use crate::Cell;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub enum SolveResult<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
> {
    NoSolution,
    UniqueSolution(Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>),
    MultipleSolutions(Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>),
}

pub fn solve<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &mut Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
    rules: &impl PuzzleRules<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
) -> SolveResult<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT> {
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
    for guess in 1..=LENGTH {
        let guess: u8 = guess.try_into().unwrap();
        board.cells[index] = Cell::Filled(guess.try_into().unwrap());
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

pub fn solve_one<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &mut Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
    rules: &impl PuzzleRules<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
) -> Option<Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>> {
    if !rules.is_valid(board) {
        return None;
    }
    // Find an empty cell.
    let index = board
        .cells
        .iter()
        .enumerate()
        .find(|(_i, &cell)| matches!(cell, Cell::Unfilled))
        .map(|e| e.0);
    let index = match index {
        None => return Some(board.clone()),
        Some(v) => v,
    };
    let mut rng = thread_rng();
    let mut options: Vec<_> = (1..=LENGTH).collect();
    options.shuffle(&mut rng);
    for &guess in options.iter() {
        let guess: u8 = guess.try_into().unwrap();
        board.cells[index] = Cell::Filled(guess.try_into().unwrap());
        match solve_one(board, rules) {
            None => (),
            Some(b) => {
                return Some(b);
            }
        }
    }
    // Make sure we exit this function with the board unchanged if we did not find a solution.
    board.cells[index] = Cell::Unfilled;
    None
}

pub fn derive<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &mut Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
    rules: &impl PuzzleRules<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
) -> Option<Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>> {
    let mut acc = None;
    derive_recursive(board, rules, &mut acc);
    acc
}

fn derive_recursive<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &mut Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
    rules: &impl PuzzleRules<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
    acc: &mut Option<Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>>,
) {
    if !rules.is_valid(board) {
        return;
    }
    // Find an empty cell.
    let index = board
        .cells
        .iter()
        .enumerate()
        .find(|(_i, &cell)| matches!(cell, Cell::Unfilled))
        .map(|e| e.0);
    let index = match index {
        None => {
            match acc {
                None => *acc = Some(board.clone()),
                Some(acc) => {
                    for (dst, src) in acc.cells.iter_mut().zip(board.cells.iter()) {
                        if *dst != *src {
                            *dst = Cell::Unfilled;
                        }
                    }
                }
            }
            dbg!(acc);
            return;
        }
        Some(v) => v,
    };
    for guess in 1..=LENGTH {
        let guess: u8 = guess.try_into().unwrap();
        board.cells[index] = Cell::Filled(guess.try_into().unwrap());
        derive_recursive(board, rules, acc);
    }
    // Make sure we exit this function with the board unchanged.
    board.cells[index] = Cell::Unfilled;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::ClassicSudoku;
    use crate::rules::KnightsRestrictionSudoku;

    #[test]
    fn test_unique_solution() {
        // From https://raw.githubusercontent.com/maxbergmark/sudoku-solver/master/data-sets/hard_sudokus_solved.txt.
        let mut puzzle: Board<81, 9, 3, 3> =
            "000075400000000008080190000300001060000000034000068170204000603900000020530200000"
                .parse()
                .unwrap();
        let solved: Board<81, 9, 3, 3> =
            "693875412145632798782194356357421869816957234429368175274519683968743521531286947"
                .parse()
                .unwrap();
        assert_eq!(
            solve(&mut puzzle, &ClassicSudoku {}),
            SolveResult::UniqueSolution(solved)
        );
    }

    #[test]
    fn test_multiple_solutions() {
        let mut puzzle: Board<81, 9, 3, 3> = Board::unfilled();
        assert!(matches!(
            solve(&mut puzzle, &ClassicSudoku {}),
            SolveResult::MultipleSolutions(_)
        ));
    }

    #[test]
    fn test_no_solutions() {
        let mut puzzle: Board<81, 9, 3, 3> = Board {
            cells: [Cell::Filled((1).try_into().unwrap()); 81],
        };
        assert!(matches!(
            solve(&mut puzzle, &ClassicSudoku {}),
            SolveResult::NoSolution
        ));
    }

    #[test]
    fn test_unique_solution_solve_one() {
        // From https://raw.githubusercontent.com/maxbergmark/sudoku-solver/master/data-sets/hard_sudokus_solved.txt.
        let mut puzzle: Board<81, 9, 3, 3> =
            "693875412000000008080190000300001060000000034000068170204000603900000020530200000"
                .parse()
                .unwrap();
        let solved: Board<81, 9, 3, 3> =
            "693875412145632798782194356357421869816957234429368175274519683968743521531286947"
                .parse()
                .unwrap();
        assert_eq!(solve_one(&mut puzzle, &ClassicSudoku {}), Some(solved));
    }

    #[test]
    fn test_derive_solution() {
        let solved: Board<81, 9, 3, 3> =
            "693875412145632798782194356357421869816957234429368175274519683968743521531286947"
                .parse()
                .unwrap();
        let mut missing: Board<81, 9, 3, 3> =
            "093875412145632798782194356357421869816957234429368175274519683968743521531286947"
                .parse()
                .unwrap();
        assert_eq!(derive(&mut missing, &ClassicSudoku {}), Some(solved));
    }

    #[test]
    fn test_derive_no_unique_solution() {
        let mut missing: Board<81, 9, 3, 3> =
            "050971624946823157007654983000549768465387219789162005090715006634298571571436892"
                .parse()
                .unwrap();
        let still_missing = missing.clone();
        assert_eq!(
            derive(&mut missing, &KnightsRestrictionSudoku {}),
            Some(still_missing)
        );
        // Disambiguate the left side.
        missing.cells[56] = Cell::Filled(8.try_into().unwrap());
        let still_missing: Board<81, 9, 3, 3> =
            "853971624946823157127654983312549768465387219789162005298715006634298571571436892"
                .parse()
                .unwrap();
        assert_eq!(
            derive(&mut missing, &KnightsRestrictionSudoku {}),
            Some(still_missing)
        );
        missing.cells[51] = Cell::Filled(4.try_into().unwrap());
        let finished: Board<81, 9, 3, 3> =
            "853971624946823157127654983312549768465387219789162435298715346634298571571436892"
                .parse()
                .unwrap();
        assert_eq!(
            derive(&mut missing, &KnightsRestrictionSudoku {}),
            Some(finished)
        );
    }

    #[test]
    fn test_knights_move() {
        // https://logic-masters.de/Raetselportal/Raetsel/zeigen.php?id=0005HX with some digits filled in for speed.
        let mut puzzle: Board<81, 9, 3, 3> =
            "894562371000403000006000500010000020008000700020000030009000200000206000452789613"
                .parse()
                .unwrap();
        let solution: Board<81, 9, 3, 3> =
            "894562371275413896136978542513697428948321765627845139369154287781236954452789613"
                .parse()
                .unwrap();
        assert_eq!(
            solve(&mut puzzle, &KnightsRestrictionSudoku {}),
            SolveResult::UniqueSolution(solution)
        );
    }
}
