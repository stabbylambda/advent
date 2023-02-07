use advent_2015_09::{parse, problem1};
use common::get_raw_input;

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let (min, max) = problem1(&input);
    println!("problem 1 score: {min}");

    println!("problem 2 score: {max}");
}
