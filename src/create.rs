use crate::rules::{util::is_valid_classic, PuzzleRules};
use crate::solve::solve;
use crate::solve::SolveResult;
use crate::Board;
use crate::Cell;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::convert::TryInto;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PuzzleCreateError {
    NoSolution,
    MultipleSolutions,
}

impl fmt::Display for PuzzleCreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::NoSolution => "No solution",
            Self::MultipleSolutions => "Multiple solutions",
        };
        f.write_str(error)
    }
}

impl Error for PuzzleCreateError {}

struct Piece(usize, usize, usize, usize);

const PIECES: [Piece; 7] = [
    Piece(0, 9, 18, 27),
    Piece(1, 10, 19, 20),
    Piece(2, 3, 4, 11),
    Piece(5, 6, 15, 24),
    Piece(7, 8, 16, 17),
    Piece(12, 21, 22, 31),
    Piece(13, 14, 23, 32),
];

pub fn create_tetris() -> Option<Board> {
    let mut board = Board {
        cells: [Cell::Unfilled; 81],
    };
    create_tetris_recursive(&mut board, 0)
}

use itertools::Itertools;
fn create_tetris_recursive(board: &mut Board, piece_index: usize) -> Option<Board> {
    if !is_valid_classic(board) {
        return None;
    }
    let piece = match PIECES.get(piece_index) {
        None => return Some(board.clone()),
        Some(v) => v,
    };
    for min in 1..=6u8 {
        let assign = [
            Cell::Filled(min.try_into().unwrap()),
            Cell::Filled((min + 1).try_into().unwrap()),
            Cell::Filled((min + 2).try_into().unwrap()),
            Cell::Filled((min + 3).try_into().unwrap()),
        ];
        for a in assign.iter().permutations(4) {
            let (a1, a2, a3, a4) = (a[0], a[1], a[2], a[3]);
            board.cells[piece.0] = *a1;
            board.cells[piece.1] = *a2;
            board.cells[piece.2] = *a3;
            board.cells[piece.3] = *a4;
            match create_tetris_recursive(board, piece_index + 1) {
                None => (),
                Some(v) => return Some(v),
            }
        }
    }
    board.cells[piece.0] = Cell::Unfilled;
    board.cells[piece.1] = Cell::Unfilled;
    board.cells[piece.2] = Cell::Unfilled;
    board.cells[piece.3] = Cell::Unfilled;
    None
}

pub fn create_puzzle_solution(rules: &impl PuzzleRules) -> Option<Board> {
    let mut rng = thread_rng();
    create_puzzle_solution_recursive(
        &mut Board {
            cells: [Cell::Unfilled; 81],
        },
        rules,
        &mut rng,
    )
}

fn create_puzzle_solution_recursive(
    board: &mut Board,
    rules: &impl PuzzleRules,
    rng: &mut impl Rng,
) -> Option<Board> {
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
    let mut options = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    options.shuffle(rng);
    for &guess in options.iter() {
        board.cells[index] = Cell::Filled(guess.try_into().unwrap());
        match create_puzzle_solution_recursive(board, rules, rng) {
            None => (),
            Some(b) => return Some(b),
        }
    }
    // Make sure we exit this function with the board unchanged if we found no solution.
    board.cells[index] = Cell::Unfilled;
    None
}

pub fn create_puzzle_from(
    board: &mut Board,
    rules: &impl PuzzleRules,
) -> Result<(), PuzzleCreateError> {
    match solve(board, rules) {
        SolveResult::NoSolution => return Err(PuzzleCreateError::NoSolution),
        SolveResult::MultipleSolutions(_) => return Err(PuzzleCreateError::MultipleSolutions),
        SolveResult::UniqueSolution(_) => (),
    }
    // Keep removing digits while there exists a unique solution.
    let mut rng = thread_rng();
    while remove_digit(board, rules, &mut rng) {}
    Ok(())
}

fn remove_digit(board: &mut Board, rules: &impl PuzzleRules, rng: &mut impl Rng) -> bool {
    let mut filled_indexes: Vec<usize> = board
        .cells
        .iter()
        .enumerate()
        .filter(|(_i, c)| matches!(c, Cell::Filled(_)))
        .map(|e| e.0)
        .collect();
    filled_indexes.shuffle(rng);
    let len = filled_indexes.len();
    for i in filled_indexes {
        let old_value = board.cells[i];
        board.cells[i] = Cell::Unfilled;
        match solve(board, rules) {
            SolveResult::NoSolution => {
                board.cells[i] = old_value;
                return false;
            }
            SolveResult::UniqueSolution(_) => {
                dbg!(board);
                dbg!(len - 1);
                return true;
            }
            SolveResult::MultipleSolutions(_) => {
                board.cells[i] = old_value;
                dbg!(i);
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::ClassicSudoku;

    #[test]
    fn test_create_solution() {
        let rules = ClassicSudoku {};
        let board = create_puzzle_solution(&rules).unwrap();
        assert!(rules.is_valid(&board));
    }
}
