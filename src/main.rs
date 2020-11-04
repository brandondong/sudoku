use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU8;

#[derive(Clone, PartialEq)]
struct Board {
    // Rows are read from left to right and then top to bottom.
    cells: [Cell; 81],
}

impl Board {
    fn new(s: &str) -> Board {
        let mut cells = [Cell::Unfilled; 81];
        for (dst, src) in cells.iter_mut().zip(s.chars().map(|c| {
            let digit = c.to_digit(10).unwrap();
            let digit: u8 = digit.try_into().unwrap();
            if digit == 0 {
                Cell::Unfilled
            } else {
                Cell::Filled(NonZeroU8::new(digit).unwrap())
            }
        })) {
            *dst = src
        }
        Board { cells }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in self.cells.iter() {
            match cell {
                Cell::Unfilled => f.write_str("0")?,
                Cell::Filled(v) => v.fmt(f)?,
            };
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Unfilled,
    Filled(NonZeroU8),
}

#[derive(Debug, PartialEq)]
enum SolveResult {
    NoSolution,
    UniqueSolution(Board),
    MultipleSolutions(Board),
}

#[derive(Debug)]
enum PuzzleCreateError {
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

trait PuzzleRules {
    fn is_valid(&self, board: &Board) -> bool;
}

struct ClassicSudoku {}

impl PuzzleRules for ClassicSudoku {
    fn is_valid(&self, board: &Board) -> bool {
        is_valid_classic(board)
    }
}

struct ParityRestriction {}

impl PuzzleRules for ParityRestriction {
    fn is_valid(&self, board: &Board) -> bool {
        fn is_even_cell(c: Cell) -> bool {
            match c {
                Cell::Unfilled => false,
                Cell::Filled(v) => v.get() % 2 == 0,
            }
        }

        if !is_valid_classic(board) {
            return false;
        }
        // Additional rule: each even cell must have at least 3 odd neighbors.
        for i in board
            .cells
            .iter()
            .enumerate()
            .filter(|(_i, &c)| is_even_cell(c))
            .map(|e| e.0)
        {
            let mut odd_count = 0;
            let row = i / 9;
            let column = i % 9;
            if row > 0 && !is_even_cell(board.cells[i - 9]) {
                odd_count += 1;
            }
            if row < 8 && !is_even_cell(board.cells[i + 9]) {
                odd_count += 1;
            }
            if column > 0 && !is_even_cell(board.cells[i - 1]) {
                odd_count += 1;
            }
            if column < 8 && !is_even_cell(board.cells[i + 1]) {
                odd_count += 1;
            }
            if odd_count < 3 {
                return false;
            }
        }
        true
    }
}

fn is_valid_classic(board: &Board) -> bool {
    // Each row, column, and block must not contain duplicate digits.
    let mut row_values = [[false; 9]; 9];
    let mut column_values = [[false; 9]; 9];
    let mut block_values = [[false; 9]; 9];
    for (i, v) in board.cells.iter().enumerate().filter_map(|(i, c)| match c {
        Cell::Unfilled => None,
        Cell::Filled(v) => Some((i, v)),
    }) {
        let value_index: usize = (v.get() - 1).into();
        let row = i / 9;
        let column = i % 9;
        let block = (row / 3) * 3 + column / 3;

        if row_values[row][value_index] {
            return false;
        }
        if column_values[column][value_index] {
            return false;
        }
        if block_values[block][value_index] {
            return false;
        }
        row_values[row][value_index] = true;
        column_values[column][value_index] = true;
        block_values[block][value_index] = true;
    }
    true
}

fn main() {
    let mut board = Board::new(
        "123456789456789132789123465215834976978561243634972518367245891892317654541698327",
    );
    dbg!(create_puzzle(&mut board, &ParityRestriction {}).unwrap());
}

fn solve(board: &mut Board, rules: &impl PuzzleRules) -> SolveResult {
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
                current_result = SolveResult::UniqueSolution(b)
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

fn create_puzzle(board: &mut Board, rules: &impl PuzzleRules) -> Result<(), PuzzleCreateError> {
    match solve(board, rules) {
        SolveResult::NoSolution => return Err(PuzzleCreateError::NoSolution),
        SolveResult::MultipleSolutions(_) => return Err(PuzzleCreateError::MultipleSolutions),
        SolveResult::UniqueSolution(_) => (),
    }
    // Keep removing digits while there exists a unique solution.
    let mut rng = thread_rng();
    let mut count_filled = board
        .cells
        .iter()
        .filter(|c| matches!(c, Cell::Filled(_)))
        .count();
    while count_filled > 0 {
        // Each iteration attempts to remove one digit.
        let mut failed_removals = HashSet::new();
        loop {
            // Choose a random digit to remove.
            let choice = rng.gen_range(0, count_filled - failed_removals.len());
            // Find its index.
            let cell_index = board
                .cells
                .iter()
                .enumerate()
                .filter(|(i, c)| matches!(c, Cell::Filled(_)) && !failed_removals.contains(i))
                .nth(choice)
                .map(|e| e.0)
                .unwrap();
            let old_value = board.cells[cell_index];
            board.cells[cell_index] = Cell::Unfilled;
            match solve(board, rules) {
                SolveResult::NoSolution => return Err(PuzzleCreateError::NoSolution), // Something has gone terribly wrong with the puzzle constraints...
                SolveResult::UniqueSolution(_) => {
                    count_filled -= 1;
                    dbg!(&board);
                    dbg!(count_filled);
                    break;
                }
                SolveResult::MultipleSolutions(_) => {
                    board.cells[cell_index] = old_value;
                    failed_removals.insert(cell_index);
                    dbg!(failed_removals.len());
                    if failed_removals.len() == count_filled {
                        // All options of digit removal resulted in multiple solution puzzles.
                        return Ok(());
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
