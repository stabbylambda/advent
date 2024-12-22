use nom::{
    character::complete::{newline, u64},
    multi::separated_list1,
    IResult,
};
use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let i = Instant::now();
    let score = problem1(&input);
    let d = i.elapsed();
    println!("problem 1 score: {score} in {d:?}");

    let i = Instant::now();
    let score = problem2(&input);
    let d = i.elapsed();
    println!("problem 2 score: {score} in {d:?}");
}

type Input = Vec<u64>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, u64)(input);

    result.unwrap().1
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}
fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn next(current: u64) -> u64 {
    let current = mix(current, current * 64);
    let current = prune(current);
    let current = mix(current, current / 32);
    let current = prune(current);
    let current = mix(current, current * 2048);

    prune(current)
}

fn problem1(input: &Input) -> u64 {
    input
        .iter()
        .map(|buyer| (0..2000).fold(*buyer, |acc, _| next(acc)))
        .sum()
}

fn problem2(input: &Input) -> u64 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 37327623)
    }

    #[test]
    #[ignore]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
