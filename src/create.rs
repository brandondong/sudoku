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

const PIECES: [Piece; 3] = [
    Piece(26, 35, 44, 53),
    Piece(25, 34, 33, 42),
    Piece(28, 37, 46, 47),
];

pub fn create_parity() -> Board {
    loop {
        let mut board = Board {
            cells: [Cell::Unfilled; 81],
        };
        let solution = create_parity_recursive(&mut board).unwrap();
        if valid_connected_components(&solution) {
            return solution;
        }
    }
}

fn create_parity_recursive(board: &mut Board) -> Option<Board> {
    if !is_valid_parity(board) {
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
    let mut options = [1, 2];
    options.shuffle(&mut thread_rng());
    for &guess in &options {
        board.cells[index] = Cell::Filled(guess.try_into().unwrap());
        match create_parity_recursive(board) {
            None => (),
            Some(b) => return Some(b),
        }
    }
    // Make sure we exit this function with the board unchanged if we found no solution.
    board.cells[index] = Cell::Unfilled;
    None
}

fn is_valid_parity(board: &Board) -> bool {
    // Each row, column, and block must contain 4 even digits and 5 odd digits.
    let mut row_evens = [0; 9];
    let mut column_evens = [0; 9];
    let mut block_evens = [0; 9];
    let mut row_odds = [0; 9];
    let mut column_odds = [0; 9];
    let mut block_odds = [0; 9];
    for (i, v) in board.cells.iter().enumerate().filter_map(|(i, c)| match c {
        Cell::Unfilled => None,
        Cell::Filled(v) => Some((i, v)),
    }) {
        let row = i / 9;
        let column = i % 9;
        let block = (row / 3) * 3 + column / 3;
        if v.get() % 2 == 0 {
            if row_evens[row] == 4 {
                return false;
            }
            if column_evens[column] == 4 {
                return false;
            }
            if block_evens[block] == 4 {
                return false;
            }
            row_evens[row] += 1;
            column_evens[column] += 1;
            block_evens[block] += 1;
        } else {
            if row_odds[row] == 5 {
                return false;
            }
            if column_odds[column] == 5 {
                return false;
            }
            if block_odds[block] == 5 {
                return false;
            }
            row_odds[row] += 1;
            column_odds[column] += 1;
            block_odds[block] += 1;
        }
    }
    true
}

fn valid_connected_components(board: &Board) -> bool {
    if !board.cells.iter().all(|&c| matches!(c, Cell::Filled(_))) {
        return true;
    }
    let c: Vec<_> = board
        .cells
        .iter()
        .filter_map(|c| match c {
            Cell::Unfilled => None,
            Cell::Filled(v) => Some(v.get()),
        })
        .collect();
    let mut visited = [false; 81];
    let mut inversion_count = 0;
    loop {
        match c.iter().enumerate().find(|(i, _c)| !visited[*i]) {
            None => break,
            Some((i, v)) => {
                let parity = *v % 2;
                let size = dfs_connected(&c, i, &mut visited, parity);
                if parity == 0 {
                    if size > 9 {
                        return false;
                    }
                    if size % 2 != 0 {
                        inversion_count += size;
                    }
                } else {
                    if size > 9 {
                        return false;
                    }
                    if size % 2 == 0 {
                        inversion_count += size;
                    }
                }
            }
        }
    }
    if inversion_count > 7 {
        return false;
    }
    dbg!(inversion_count);
    true
}

fn dfs_connected(c: &[u8], i: usize, visited: &mut [bool], parity: u8) -> usize {
    let v = match c.get(i) {
        None => return 0,
        Some(v) => *v,
    };
    if v % 2 != parity {
        return 0;
    }
    if visited[i] {
        return 0;
    }
    visited[i] = true;
    let mut total = 1;
    // Up.
    if i >= 9 {
        total += dfs_connected(c, i - 9, visited, parity);
    }
    // Left.
    if i % 9 != 0 {
        total += dfs_connected(c, i - 1, visited, parity);
    }
    // Right.
    if i % 9 != 8 {
        total += dfs_connected(c, i + 1, visited, parity);
    }
    // Down.
    total += dfs_connected(c, i + 9, visited, parity);
    total
}

pub fn create_tetris() -> Option<Board> {
    let mut board: Board =
        "174635289085914367069782451040063872050000603023000004000000000000000000000000000"
            .parse()
            .unwrap();
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
