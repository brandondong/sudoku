use std::num::NonZeroU8;

#[derive(Clone)]
struct Board {
    // Rows are read from left to right and then top to bottom.
    cells: [Cell; 81],
}

#[derive(Clone, Copy)]
enum Cell {
    Unfilled,
    Filled(NonZeroU8),
}

enum SolveResult {
    NoSolution,
    UniqueSolution(Board),
    MultipleSolutions(Board),
}

fn main() {
    let mut board = Board {
        cells: [Cell::Filled(NonZeroU8::new(1).unwrap()); 81],
    };
    solve(&mut board);
}

fn solve(board: &mut Board) -> SolveResult {
    if !is_valid(&board) {
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

fn is_valid(_board: &Board) -> bool {
    true
}
