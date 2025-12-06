use crate::{part1::problem1, part2::problem2};

pub mod part1;
pub mod part2;

fn main() {
    let input = common::read_input!();

    let answer = problem1(input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(input);
    println!("problem 2 answer: {answer}");
}
