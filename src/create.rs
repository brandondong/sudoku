use crate::rules::PuzzleRules;
use crate::solve::solve;
use crate::solve::SolveResult;
use crate::Board;
use crate::Cell;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PuzzleCreateError {
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

pub fn create_puzzle_from(
    board: &mut Board,
    rules: &impl PuzzleRules,
) -> Result<(), PuzzleCreateError> {
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
