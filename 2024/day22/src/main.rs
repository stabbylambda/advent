use itertools::{iterate, Itertools};
use nom::{
    character::complete::{newline, u64},
    multi::separated_list1,
    IResult, Parser,
};
use std::{collections::HashMap, time::Instant};

fn main() {
    let input = common::read_input!();
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
    let result: IResult<&str, Input> = separated_list1(newline, u64).parse(input);

    result.unwrap().1
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn next(current: &u64) -> u64 {
    let current = mix(*current, current * 64);
    let current = prune(current);
    let current = mix(current, current / 32);
    let current = prune(current);
    let current = mix(current, current * 2048);

    prune(current)
}

fn problem1(input: &Input) -> u64 {
    input
        .iter()
        .flat_map(|buyer| iterate(*buyer, next).nth(2000))
        .sum()
}

fn get_prices(secret: u64) -> Vec<i8> {
    iterate(secret, next)
        .take(2001)
        .map(|x| (x % 10) as i8)
        .collect()
}

fn get_deltas(prices: &[i8]) -> HashMap<(i8, i8, i8, i8), u64> {
    prices
        .iter()
        .tuple_windows()
        .map(|(a, b, c, d, e)| ((b - a, c - b, d - c, e - d), e))
        .fold(HashMap::new(), |mut acc, (k, v)| {
            // only keep the first time this has been seen
            acc.entry(k).or_insert(*v as u64);
            acc
        })
}

fn problem2(input: &Input) -> u64 {
    *input
        .iter()
        .map(|x| get_prices(*x))
        .flat_map(|v| get_deltas(&v))
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).and_modify(|x| *x += v).or_insert(v);
            acc
        })
        .values()
        .max()
        .unwrap()
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
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 23)
    }
}
