use crate::Board;
use crate::Cell;

pub trait PuzzleRules {
    fn is_valid(&self, board: &Board) -> bool;
}
pub struct ClassicSudoku {}

impl PuzzleRules for ClassicSudoku {
    fn is_valid(&self, board: &Board) -> bool {
        is_valid_classic(board)
    }
}

// A very uninteresting puzzle constraint.
// However, it can be used with the solver to quickly find interesting solutions.
// For example, meeting the 112121212121212121212121112121212121212111212121212121211121212121212121212121211 restriction
// guarantees all even digits only have odd neighbors.
pub struct ParityMask {
    even_mask: [bool; 81],
}

impl ParityMask {
    pub fn new(s: &str) -> ParityMask {
        let mut even_mask = [false; 81];
        for (dst, src) in even_mask.iter_mut().zip(s.chars().map(|c| {
            let digit = c.to_digit(10).unwrap();
            digit % 2 == 0
        })) {
            *dst = src
        }
        ParityMask { even_mask }
    }
}

impl PuzzleRules for ParityMask {
    fn is_valid(&self, board: &Board) -> bool {
        let parity_mismatch = board
            .cells
            .iter()
            .zip(self.even_mask.iter())
            .any(|(c, &is_even)| match c {
                Cell::Unfilled => false,
                Cell::Filled(v) => (v.get() % 2 == 0) != is_even,
            });
        if parity_mismatch {
            return false;
        }
        is_valid_classic(board)
    }
}

// Even digits must have odd orthogonally adjacent cells.
pub struct EvenOddNeighbors {}

impl PuzzleRules for EvenOddNeighbors {
    fn is_valid(&self, board: &Board) -> bool {
        fn is_even_cell(c: Cell) -> bool {
            match c {
                Cell::Unfilled => false,
                Cell::Filled(v) => v.get() % 2 == 0,
            }
        }
        let has_even_neighbor = board
            .cells
            .iter()
            .enumerate()
            .filter(|(_i, &c)| is_even_cell(c))
            .any(|(i, _v)| {
                let row = i / 9;
                let column = i % 9;
                row > 0 && is_even_cell(board.cells[i - 9])
                    || row < 8 && is_even_cell(board.cells[i + 9])
                    || column > 0 && is_even_cell(board.cells[i - 1])
                    || column < 8 && is_even_cell(board.cells[i + 1])
            });
        if has_even_neighbor {
            return false;
        }
        is_valid_classic(board)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve::solve;
    use crate::solve::SolveResult;

    #[test]
    fn test_even_odd_valid() {
        let board = Board::new(
            "132547698547698123698123574321456789874931256965872341419765832783214965256389417",
        );
        assert!(EvenOddNeighbors {}.is_valid(&board));
    }

    #[test]
    fn test_even_odd_solve() {
        let mut puzzle = Board::new(
            "000000698000090100000000000000006089004000050000070000000700000700000900000300000",
        );
        let solution = Board::new(
            "132547698547698123698123574321456789874931256965872341419765832783214965256389417",
        );
        assert_eq!(
            solve(&mut puzzle, &EvenOddNeighbors {}),
            SolveResult::UniqueSolution(solution)
        );
    }
}
