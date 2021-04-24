use itertools::Itertools;
use sudoku::create::create_puzzle_solution;
use sudoku::rules::ParityMask;

fn main() {
    // let parity: ParityMask =
    //     "212111122111212221221212111222112111121212121111121222212221111121121212112121212"
    //         .parse()
    //         .unwrap();
    // dbg!(create_puzzle_solution(&parity));
    let a = [3, 4, 2, 1, 5, 6, 7, 8, 9];
    //let a = [1, 2, 3];
    let min = 1;
    let max = a.len();
    for p in a.iter().permutations(a.len()) {
        let mut sandwich = 0;
        let mut in_sandwich = false;
        for &&i in p.iter() {
            let is_boundary = i == min || i == max;
            if in_sandwich {
                if is_boundary {
                    break;
                } else {
                    sandwich += i;
                }
            } else {
                if is_boundary {
                    in_sandwich = true;
                }
            }
        }
        let num = p[0];
        let left_x_sum: usize = p.iter().take(*num).copied().sum();
        let num = p[p.len() - 1];
        let right_x_sum: usize = p.iter().rev().take(*num).copied().sum();
        if left_x_sum == right_x_sum && sandwich == right_x_sum {
            println!("{:?}", p);
            println!("{}", sandwich);
        }
    }
}
