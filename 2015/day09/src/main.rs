use advent_2015_09::{parse, problem1};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let (min, max) = problem1(&input);
    println!("problem 1 score: {min}");

    println!("problem 2 score: {max}");
}
