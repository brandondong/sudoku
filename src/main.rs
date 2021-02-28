use sudoku::Board;
use sudoku::{rules::Miracle, solve::solve_one};

fn main() {
    let mut board: Board<81, 9, 3, 3> = Board::unfilled();
    dbg!(solve_one(&mut board, &Miracle {}).unwrap());
}
