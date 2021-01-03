use sudoku::create::create_puzzle_solution;
use sudoku::rules::ParityMask;

fn main() {
    let parity: ParityMask =
        "212111122111212221221212111222112111121212121111121222212221111121121212112121212"
            .parse()
            .unwrap();
    dbg!(create_puzzle_solution(&parity));
}
