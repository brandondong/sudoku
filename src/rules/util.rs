use crate::Board;
use crate::Cell;

pub fn is_valid_classic(board: &Board) -> bool {
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

pub fn passes_knights_move_constraint(board: &Board) -> bool {
    !board
        .cells
        .iter()
        .enumerate()
        .filter(|(_i, &c)| matches!(c, Cell::Filled(_)))
        .any(|(i, &v)| {
            let row = i / 9;
            let column = i % 9;
            row >= 1 && column >= 2 && board.cells[i - 11] == v
                || row >= 1 && column <= 6 && board.cells[i - 7] == v
                || row <= 7 && column >= 2 && board.cells[i + 7] == v
                || row <= 7 && column <= 6 && board.cells[i + 11] == v
                || column >= 1 && row >= 2 && board.cells[i - 19] == v
                || column <= 7 && row >= 2 && board.cells[i - 17] == v
                || column >= 1 && row <= 6 && board.cells[i + 17] == v
                || column <= 7 && row <= 6 && board.cells[i + 19] == v
        })
}
