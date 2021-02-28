use sudoku::rules::ClassicSudoku;
use sudoku::rules::Miracle;
use sudoku::{create::create_puzzle_solution, rules::PuzzleRules};

fn main() {
    for _i in 0..100 {
        match create_puzzle_solution(&Miracle {}) {
            None => println!("No solution"),
            Some(board) => {
                if !(ClassicSudoku {}).is_valid(&board) {
                    dbg!(board);
                    return;
                }
            }
        }
    }
}
