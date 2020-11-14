use sudoku::create::create_puzzle_solution;
use sudoku::rules::Test;
use sudoku::Board;

fn main() {
    dbg!(create_puzzle_solution(&Test {}).unwrap());
}
