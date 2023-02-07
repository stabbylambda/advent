use advent_2015_10::*;
use common::get_raw_input;

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {}", score.len());

    let score = problem2(&score);
    println!("problem 2 score: {}", score.len());
}
