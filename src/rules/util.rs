use std::num::NonZeroU8;

use crate::Board;
use crate::Cell;

pub fn is_valid_classic<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
) -> bool {
    // Each row, column, and block must not contain duplicate digits.
    let mut row_values = [[false; LENGTH]; LENGTH];
    let mut column_values = [[false; LENGTH]; LENGTH];
    let mut block_values = [[false; LENGTH]; LENGTH];
    for (i, v) in board.cells.iter().enumerate().filter_map(|(i, c)| match c {
        Cell::Unfilled => None,
        Cell::Filled(v) => Some((i, v)),
    }) {
        let value_index: usize = (v.get() - 1).into();
        let row = i / LENGTH;
        let column = i % LENGTH;
        let block = (row / BOX_HEIGHT) * BOX_WIDTH + column / BOX_WIDTH;

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

pub fn is_valid_irregular<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
) -> bool {
    // Each row, and column must not contain duplicate digits.
    let mut row_values = [[false; LENGTH]; LENGTH];
    let mut column_values = [[false; LENGTH]; LENGTH];
    for (i, v) in board.cells.iter().enumerate().filter_map(|(i, c)| match c {
        Cell::Unfilled => None,
        Cell::Filled(v) => Some((i, v)),
    }) {
        let value_index: usize = (v.get() - 1).into();
        let row = i / LENGTH;
        let column = i % LENGTH;

        if row_values[row][value_index] {
            return false;
        }
        if column_values[column][value_index] {
            return false;
        }
        row_values[row][value_index] = true;
        column_values[column][value_index] = true;
    }
    true
}

pub fn passes_knights_move_constraint<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
) -> bool {
    !board
        .cells
        .iter()
        .enumerate()
        .filter(|(_i, &c)| matches!(c, Cell::Filled(_)))
        .any(|(i, &v)| {
            let row = i / LENGTH;
            let column = i % LENGTH;
            row >= 1 && column >= 2 && board.cells[i - (LENGTH + 2)] == v // Left up.
                || row >= 1 && column <= (LENGTH - 3) && board.cells[i - (LENGTH - 2)] == v // Right up.
                || row <= (LENGTH - 2) && column >= 2 && board.cells[i + (LENGTH - 2)] == v // Left down.
                || row <= (LENGTH - 2)
                    && column <= (LENGTH - 3)
                    && board.cells[i + (LENGTH + 2)] == v // Right down.
                || column >= 1 && row >= 2 && board.cells[i - (2 * LENGTH + 1)] == v // Up left.
                || column <= (LENGTH - 2) && row >= 2 && board.cells[i - (2 * LENGTH - 1)] == v // Up right.
                || column >= 1 && row <= (LENGTH - 3) && board.cells[i + (2 * LENGTH - 1)] == v // Down left.
                || column <= (LENGTH - 2)
                    && row <= (LENGTH - 3)
                    && board.cells[i + (2 * LENGTH + 1)] == v // Down right.
        })
}

pub fn passes_kings_move_constraint<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
) -> bool {
    !board
        .cells
        .iter()
        .enumerate()
        .filter(|(_i, &c)| matches!(c, Cell::Filled(_)))
        .any(|(i, &v)| {
            let row = i / LENGTH;
            let column = i % LENGTH;
            row >= 1 && column >= 1 && board.cells[i - (LENGTH + 1)] == v // Left up.
                || row >= 1 && column <= (LENGTH - 2) && board.cells[i - (LENGTH - 1)] == v // Right up.
                || row <= (LENGTH - 2) && column >= 1 && board.cells[i + (LENGTH - 1)] == v // Left down.
                || row <= (LENGTH - 2)
                    && column <= (LENGTH - 2)
                    && board.cells[i + (LENGTH + 1)] == v // Right down.
        })
}

pub fn passes_nonconsecutive_constraint<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
) -> bool {
    !board
        .cells
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| match c {
            Cell::Unfilled => None,
            Cell::Filled(v) => Some((i, v.get())),
        })
        .any(|(i, v)| {
            let row = i / LENGTH;
            let column = i % LENGTH;
            column >= 1 && consecutive_val(board.cells[i - 1], v) // Left.
                || column <= (LENGTH - 2) && consecutive_val(board.cells[i + 1], v) // Right.
                || row >= 1 && consecutive_val(board.cells[i - LENGTH], v) // Up.
                || row <= (LENGTH - 2)
                    && consecutive_val(board.cells[i + LENGTH], v) // Down.
        })
}

fn consecutive_val(c: Cell, v: u8) -> bool {
    c == Cell::Filled(NonZeroU8::new(v + 1).unwrap())
        || v >= 2 && c == Cell::Filled(NonZeroU8::new(v - 1).unwrap())
}
