use sudoku::{rules::count_10s, Board};
use sudoku::{rules::Miracle, solve::solve_one};

fn main() {
    loop {
        let mut board: Board<81, 9, 3, 3> =
            "173000000495000000628000000364000000817000000259000000931000000586000000742000000"
                .parse()
                .unwrap();
        // let mut board: Board<81, 9, 3, 3> = Board::unfilled();
        let solution = solve_one(&mut board, &Miracle {}).unwrap();
        dbg!(&solution);
        dbg!(count_10s(&solution));
    }
}
