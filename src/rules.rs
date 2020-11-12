pub mod util;

use crate::Board;
use crate::Cell;
use crate::ParseError;
use std::str::FromStr;
use util::is_valid_classic;
use util::passes_knights_move_constraint;

pub trait PuzzleRules {
    fn is_valid(&self, board: &Board) -> bool;
}
pub struct ClassicSudoku {}

impl PuzzleRules for ClassicSudoku {
    fn is_valid(&self, board: &Board) -> bool {
        is_valid_classic(board)
    }
}

pub struct KnightsRestrictionSudoku {}

impl PuzzleRules for KnightsRestrictionSudoku {
    fn is_valid(&self, board: &Board) -> bool {
        passes_knights_move_constraint(board) && is_valid_classic(board)
    }
}

// A very uninteresting puzzle constraint.
// However, it can be used with the solver to quickly find interesting solutions.
// For example, meeting the 112121212121212121212121112121212121212111212121212121211121212121212121212121211 restriction
// guarantees all even digits only have odd neighbors.
pub struct ParityMask {
    mask: Board,
}

impl FromStr for ParityMask {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { mask: s.parse()? })
    }
}

impl PuzzleRules for ParityMask {
    fn is_valid(&self, board: &Board) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve::solve;
    use crate::solve::SolveResult;
    use std::convert::TryInto;

    #[test]
    fn test_even_odd_valid() {
        let board: Board =
            "132547698547698123698123574321456789874931256965872341419765832783214965256389417"
                .parse()
                .unwrap();
        let invalid = Board {
            cells: [Cell::Filled(1.try_into().unwrap()); 81],
        };
        let rule = EvenOddNeighbors {};
        assert!(rule.is_valid(&board));
        assert!(!rule.is_valid(&invalid));
    }

    #[test]
    fn test_even_odd_solve() {
        let mut puzzle: Board =
            "000000698000090100000000000000006089004000050000070000000700000700000900000300000"
                .parse()
                .unwrap();
        let solution: Board =
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
        let puzzle: Board =
            "000000698000090100000000000000006089004000050000070000000700000700000900000300000"
                .parse()
                .unwrap();
        let solution: Board =
            "132547698547698123698123574321456789874931256965872341419765832783214965256389417"
                .parse()
                .unwrap();
        let empty = Board {
            cells: [Cell::Unfilled; 81],
        };
        let invalid = Board {
            cells: [Cell::Filled(1.try_into().unwrap()); 81],
        };
        let mask: ParityMask =
            "112121212121212121212121112121212121212111212121212121211121212121212121212121211"
                .parse()
                .unwrap();
        assert!(mask.is_valid(&puzzle));
        assert!(mask.is_valid(&solution));
        assert!(mask.is_valid(&empty));
        assert!(!mask.is_valid(&invalid));
    }
}
