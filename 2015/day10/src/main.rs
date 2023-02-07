use advent_2015_10::*;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {}", score.len());

    let score = problem2(&score);
    println!("problem 2 score: {}", score.len());
}
