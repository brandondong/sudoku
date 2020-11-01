use std::fmt;
use std::num::NonZeroU8;
#[derive(Clone)]
struct Board {
    // Rows are read from left to right and then top to bottom.
    cells: [Cell; 81],
}

impl Board {
    fn is_valid(&self) -> bool {
        // Each row, column, and block must not contain duplicate digits.
        let mut row_values = [[false; 9]; 9];
        let mut column_values = [[false; 9]; 9];
        let mut block_values = [[false; 9]; 9];
        for (i, v) in self.cells.iter().enumerate().filter_map(|(i, c)| match c {
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
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in self.cells.iter() {
            match cell {
                Cell::Unfilled => f.write_str(".")?,
                Cell::Filled(v) => v.fmt(f)?,
            };
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Unfilled,
    Filled(NonZeroU8),
}

#[derive(Debug)]
enum SolveResult {
    NoSolution,
    UniqueSolution(Board),
    MultipleSolutions(Board),
}

fn main() {
    let mut board = Board {
        cells: [Cell::Unfilled; 81],
    };
    dbg!(solve(&mut board));
}

fn solve(board: &mut Board) -> SolveResult {
    if !board.is_valid() {
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
        let sub_result = solve(board);
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
