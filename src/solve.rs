use crate::rules::PuzzleRules;
use crate::Board;
use crate::Cell;
use std::convert::TryInto;

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

pub fn derive(board: &mut Board, rules: &impl PuzzleRules) -> Option<Board> {
    let mut acc = None;
    derive_recursive(board, rules, &mut acc);
    acc
}

fn derive_recursive(board: &mut Board, rules: &impl PuzzleRules, acc: &mut Option<Board>) {
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
    for guess in 1..=9 {
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
        let mut puzzle: Board =
            "000075400000000008080190000300001060000000034000068170204000603900000020530200000"
                .parse()
                .unwrap();
        let solved: Board =
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
            cells: [Cell::Filled((1).try_into().unwrap()); 81],
        };
        assert!(matches!(
            solve(&mut puzzle, &ClassicSudoku {}),
            SolveResult::NoSolution
        ));
    }

    #[test]
    fn test_derive_solution() {
        let solved: Board =
            "693875412145632798782194356357421869816957234429368175274519683968743521531286947"
                .parse()
                .unwrap();
        let mut missing: Board =
            "093875412145632798782194356357421869816957234429368175274519683968743521531286947"
                .parse()
                .unwrap();
        assert_eq!(derive(&mut missing, &ClassicSudoku {}), Some(solved));
    }

    #[test]
    fn test_derive_no_unique_solution() {
        let mut missing: Board =
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
        let still_missing: Board =
            "853971624946823157127654983312549768465387219789162005298715006634298571571436892"
                .parse()
                .unwrap();
        assert_eq!(
            derive(&mut missing, &KnightsRestrictionSudoku {}),
            Some(still_missing)
        );
        missing.cells[51] = Cell::Filled(4.try_into().unwrap());
        let finished: Board =
            "853971624946823157127654983312549768465387219789162435298715346634298571571436892"
                .parse()
                .unwrap();
        assert_eq!(
            derive(&mut missing, &KnightsRestrictionSudoku {}),
            Some(finished)
        );
    }
}
