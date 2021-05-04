use sudoku::{rules::Miracle, solve::solve_one};
use sudoku::{
    rules::{count_10s, ClassicSudoku},
    Board,
};

fn main() {
    let mut board: Board<81, 9, 3, 3> =
        "964728153318495726752631948427183695185946372639572814243817569591364287876259431"
            .parse()
            .unwrap();
    let solution = solve_one(&mut board, &ClassicSudoku {}).unwrap();
    dbg!(&solution);
    dbg!(count_10s(&solution));
}

fn fun_name() -> ! {
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
