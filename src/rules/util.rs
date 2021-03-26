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
