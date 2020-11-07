use sudoku::create::create_puzzle_from;
use sudoku::rules::EvenOddNeighbors;
use sudoku::Board;

fn main() {
    let mut board = Board::new(
        "132547698547698123698123574321456789874931256965872341419765832783214965256389417",
    );
    dbg!(create_puzzle_from(&mut board, &EvenOddNeighbors {}).unwrap());
}
