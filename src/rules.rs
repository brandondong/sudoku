pub mod util;

use crate::Board;
use crate::Cell;
use crate::ParseError;
use std::convert::TryInto;
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

pub struct Test {}

impl PuzzleRules for Test {
    fn is_valid(&self, board: &Board) -> bool {
        // https://f-puzzles.com/?id=y64s5p2z
        let c = board.cells;
        match c[3] {
            Cell::Filled(v1) => {
                if v1.get() != 7 && v1.get() != 9 {
                    return false;
                }
            }
            _ => (),
        }
        match c[4] {
            Cell::Filled(v1) => {
                if v1.get() % 2 == 0 {
                    return false;
                }
            }
            _ => (),
        }
        match c[5] {
            Cell::Filled(v1) => {
                if v1.get() % 2 == 0 {
                    return false;
                }
            }
            _ => (),
        }
        match (c[3], c[4], c[5]) {
            (Cell::Filled(v1), Cell::Filled(v2), Cell::Filled(v3)) => {
                if v1.get() + v2.get() + v3.get() != 17 {
                    return false;
                }
            }
            _ => (),
        }
        match (c[9], c[10], c[11]) {
            (Cell::Filled(v1), Cell::Filled(v2), Cell::Filled(v3)) => {
                let left = v1.get();
                let right = v2.get() + v3.get();
                let b1 = left + 1 == right || left - 1 == right;
                if !b1 {
                    return false;
                }
            }
            _ => (),
        }
        match c[12] {
            Cell::Filled(v1) => {
                if v1.get() % 2 != 0 {
                    return false;
                }
            }
            _ => (),
        }
        match (c[13], c[14]) {
            (Cell::Filled(v1), Cell::Filled(v2)) => {
                let left = v1.get();
                let right = v2.get();
                let b1 = left + 1 == right || left - 1 == right;
                if !b1 {
                    return false;
                }
            }
            _ => (),
        }
        match (c[15], c[16], c[17]) {
            (Cell::Filled(v1), Cell::Filled(v2), Cell::Filled(v3)) => {
                let left = v1.get() + v2.get();
                let right = v3.get();
                let b2 = left + 1 == right || left - 1 == right;
                if !b2 {
                    return false;
                }
            }
            _ => (),
        }
        match (c[18], c[19]) {
            (Cell::Filled(v1), Cell::Filled(v2)) => {
                let left = v1.get();
                let right = v2.get();
                let b1 = left + 1 == right || left - 1 == right;
                if !b1 {
                    return false;
                }
            }
            _ => (),
        }
        match c[21] {
            Cell::Filled(v1) => {
                if v1.get() != 2 && v1.get() != 4 && v1.get() != 6 {
                    return false;
                }
            }
            _ => (),
        }
        match (c[20], c[21]) {
            (Cell::Filled(v1), Cell::Filled(v2)) => {
                let left = v1.get();
                let right = v2.get();
                let b1 = left + 1 == right || left - 1 == right;
                if !b1 {
                    return false;
                }
            }
            _ => (),
        }
        match (c[21], c[22]) {
            (Cell::Filled(v1), Cell::Filled(v2)) => {
                let left = v1.get();
                let right = v2.get();
                let b1 = left + 1 == right || left - 1 == right;
                if !b1 {
                    return false;
                }
            }
            _ => (),
        }
        match c[23] {
            Cell::Filled(v1) => {
                if v1.get() != 2 && v1.get() != 4 && v1.get() != 6 {
                    return false;
                }
            }
            _ => (),
        }
        match (c[24], c[25]) {
            (Cell::Filled(v1), Cell::Filled(v2)) => {
                let left = v1.get();
                let right = v2.get();
                let b1 = left + 1 == right || left - 1 == right;
                if !b1 {
                    return false;
                }
            }
            _ => (),
        }
        match c[27] {
            Cell::Filled(v1) => {
                if v1.get() != 1 && v1.get() != 2 && v1.get() != 3 {
                    return false;
                }
            }
            _ => (),
        }
        match c[28] {
            Cell::Filled(v1) => {
                if v1.get() != 1 && v1.get() != 2 && v1.get() != 3 {
                    return false;
                }
            }
            _ => (),
        }
        match c[29] {
            Cell::Filled(v1) => {
                if v1.get() != 1 && v1.get() != 2 && v1.get() != 3 {
                    return false;
                }
            }
            _ => (),
        }
        match c[30] {
            Cell::Filled(v1) => {
                if v1.get() != 5 {
                    return false;
                }
            }
            _ => (),
        }
        match c[31] {
            Cell::Filled(v1) => {
                if v1.get() != 4 && v1.get() != 7 && v1.get() != 9 {
                    return false;
                }
            }
            _ => (),
        }
        match c[32] {
            Cell::Filled(v1) => {
                if v1.get() != 4 && v1.get() != 7 && v1.get() != 9 {
                    return false;
                }
            }
            _ => (),
        }
        match c[36] {
            Cell::Filled(v1) => {
                if v1.get() != 4 && v1.get() != 6 {
                    return false;
                }
            }
            _ => (),
        }
        match c[37] {
            Cell::Filled(v1) => {
                if v1.get() != 4 && v1.get() != 6 {
                    return false;
                }
            }
            _ => (),
        }
        match c[38] {
            Cell::Filled(v1) => {
                if v1.get() != 5 {
                    return false;
                }
            }
            _ => (),
        }
        match c[39] {
            Cell::Filled(v1) => {
                if v1.get() != 1 && v1.get() != 3 {
                    return false;
                }
            }
            _ => (),
        }
        match c[40] {
            Cell::Filled(v1) => {
                if v1.get() != 8 {
                    return false;
                }
            }
            _ => (),
        }
        match c[41] {
            Cell::Filled(v1) => {
                if v1.get() != 7 && v1.get() != 9 {
                    return false;
                }
            }
            _ => (),
        }
        match c[45] {
            Cell::Filled(v1) => {
                if v1.get() != 7 && v1.get() != 9 {
                    return false;
                }
            }
            _ => (),
        }
        match c[46] {
            Cell::Filled(v1) => {
                if v1.get() != 8 {
                    return false;
                }
            }
            _ => (),
        }
        match c[47] {
            Cell::Filled(v1) => {
                if v1.get() != 7 && v1.get() != 9 {
                    return false;
                }
            }
            _ => (),
        }
        match c[48] {
            Cell::Filled(v1) => {
                if v1.get() != 1 && v1.get() != 3 {
                    return false;
                }
            }
            _ => (),
        }
        match c[49] {
            Cell::Filled(v1) => {
                if v1.get() != 2 && v1.get() != 6 {
                    return false;
                }
            }
            _ => (),
        }
        match c[50] {
            Cell::Filled(v1) => {
                if v1.get() != 2 && v1.get() != 6 {
                    return false;
                }
            }
            _ => (),
        }
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
