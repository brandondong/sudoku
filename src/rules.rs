pub mod util;

use crate::Board;
use crate::Cell;
use crate::ParseError;
use std::str::FromStr;

use self::util::{
    is_valid_classic, passes_kings_move_constraint, passes_knights_move_constraint,
    passes_nonconsecutive_constraint,
};

pub struct Miracle {}

impl<
        const NUM_CELLS: usize,
        const LENGTH: usize,
        const BOX_WIDTH: usize,
        const BOX_HEIGHT: usize,
    > PuzzleRules<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT> for Miracle
{
    fn is_valid(&self, board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>) -> bool {
        is_valid_classic(board) && passes_nonconsecutive_constraint(board)
    }
}

pub fn count_10s<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
) -> usize {
    board
        .cells
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| match c {
            Cell::Unfilled => None,
            Cell::Filled(v) => Some((i, v.get())),
        })
        .map(|(i, _v)| {
            let row = i / LENGTH;
            let column = i % LENGTH;
            let mut sum = 0;
            if column <= (LENGTH - 2) {
                if is_add_10(board, i + 1, i) {
                    sum += 1;
                } // Right.
            }
            if row <= (LENGTH - 2) {
                if is_add_10(board, i + LENGTH, i) {
                    sum += 1;
                } // Down.
            }
            sum
        })
        .sum()
}

fn is_add_10<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>(
    board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
    i1: usize,
    i2: usize,
) -> bool {
    let c = board.cells;
    match (c[i1], c[i2]) {
        (Cell::Filled(a), Cell::Filled(b)) => a.get() + b.get() == 10,
        _ => true,
    }
}

pub trait PuzzleRules<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
>
{
    fn is_valid(&self, board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>) -> bool;
}
pub struct ClassicSudoku {}

impl<
        const NUM_CELLS: usize,
        const LENGTH: usize,
        const BOX_WIDTH: usize,
        const BOX_HEIGHT: usize,
    > PuzzleRules<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT> for ClassicSudoku
{
    fn is_valid(&self, board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>) -> bool {
        is_valid_classic(board)
    }
}

pub struct KnightsRestrictionSudoku {}

impl<
        const NUM_CELLS: usize,
        const LENGTH: usize,
        const BOX_WIDTH: usize,
        const BOX_HEIGHT: usize,
    > PuzzleRules<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT> for KnightsRestrictionSudoku
{
    fn is_valid(&self, board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>) -> bool {
        passes_knights_move_constraint(board) && is_valid_classic(board)
    }
}

// A very uninteresting puzzle constraint.
// However, it can be used with the solver to quickly find interesting solutions.
// For example, meeting the 112121212121212121212121112121212121212111212121212121211121212121212121212121211 restriction
// guarantees all even digits only have odd neighbors.
pub struct ParityMask<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
> {
    mask: Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>,
}

impl<
        const NUM_CELLS: usize,
        const LENGTH: usize,
        const BOX_WIDTH: usize,
        const BOX_HEIGHT: usize,
    > FromStr for ParityMask<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { mask: s.parse()? })
    }
}

impl<
        const NUM_CELLS: usize,
        const LENGTH: usize,
        const BOX_WIDTH: usize,
        const BOX_HEIGHT: usize,
    > PuzzleRules<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>
    for ParityMask<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>
{
    fn is_valid(&self, board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>) -> bool {
        let parity_mismatch =
            board
                .cells
                .iter()
                .zip(self.mask.cells.iter())
                .any(|(c1, c2)| match (c1, c2) {
                    (Cell::Filled(v1), Cell::Filled(v2)) => v1.get() % 2 != v2.get() % 2,
                    (_, _) => false,
                });
        if parity_mismatch {
            return false;
        }
        is_valid_classic(board)
    }
}

// Even digits must have odd orthogonally adjacent cells.
pub struct EvenOddNeighbors {}

impl<
        const NUM_CELLS: usize,
        const LENGTH: usize,
        const BOX_WIDTH: usize,
        const BOX_HEIGHT: usize,
    > PuzzleRules<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT> for EvenOddNeighbors
{
    fn is_valid(&self, board: &Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>) -> bool {
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
                let row = i / LENGTH;
                let column = i % LENGTH;
                row > 0 && is_even_cell(board.cells[i - LENGTH])
                    || row < LENGTH - 1 && is_even_cell(board.cells[i + LENGTH])
                    || column > 0 && is_even_cell(board.cells[i - 1])
                    || column < LENGTH - 1 && is_even_cell(board.cells[i + 1])
            });
        if has_even_neighbor {
            return false;
        }
        is_valid_classic(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve::solve;
    use crate::solve::SolveResult;
    use std::convert::TryInto;

    #[test]
    fn test_even_odd_valid() {
        let board: Board<81, 9, 3, 3> =
            "132547698547698123698123574321456789874931256965872341419765832783214965256389417"
                .parse()
                .unwrap();
        let invalid: Board<81, 9, 3, 3> = Board {
            cells: [Cell::Filled(1.try_into().unwrap()); 81],
        };
        let rule = EvenOddNeighbors {};
        assert!(rule.is_valid(&board));
        assert!(!rule.is_valid(&invalid));
    }

    #[test]
    fn test_even_odd_solve() {
        let mut puzzle: Board<81, 9, 3, 3> =
            "000000698000090100000000000000006089004000050000070000000700000700000900000300000"
                .parse()
                .unwrap();
        let solution: Board<81, 9, 3, 3> =
            "132547698547698123698123574321456789874931256965872341419765832783214965256389417"
                .parse()
                .unwrap();
        assert_eq!(
            solve(&mut puzzle, &EvenOddNeighbors {}),
            SolveResult::UniqueSolution(solution)
        );
    }

    #[test]
    fn test_parity_mask_valid() {
        let puzzle: Board<81, 9, 3, 3> =
            "000000698000090100000000000000006089004000050000070000000700000700000900000300000"
                .parse()
                .unwrap();
        let solution: Board<81, 9, 3, 3> =
            "132547698547698123698123574321456789874931256965872341419765832783214965256389417"
                .parse()
                .unwrap();
        let empty = Board::unfilled();
        let invalid = Board {
            cells: [Cell::Filled(1.try_into().unwrap()); 81],
        };
        let mask: ParityMask<81, 9, 3, 3> =
            "112121212121212121212121112121212121212111212121212121211121212121212121212121211"
                .parse()
                .unwrap();
        assert!(mask.is_valid(&puzzle));
        assert!(mask.is_valid(&solution));
        assert!(mask.is_valid(&empty));
        assert!(!mask.is_valid(&invalid));
    }
}
