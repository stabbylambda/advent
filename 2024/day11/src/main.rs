use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

use nom::{bytes::complete::tag, character::complete::u64, multi::separated_list1, IResult};

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
    let result: IResult<&str, Input> = separated_list1(tag(" "), u64)(input);

    result.unwrap().1
}

fn blink_single(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let digits = (stone as f64).log10().floor() as u64 + 1;
    if digits.is_multiple_of(2) {
        let half = 10_f64.powi((digits / 2) as i32);
        let left = stone / half as u64;
        let right = stone % half as u64;
        return vec![left, right];
    }

    vec![2024 * stone]
}

fn blink(stones: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    stones
        .iter()
        .flat_map(|(stone, count)| {
            blink_single(*stone)
                .into_iter()
                .map(|new_stone| (new_stone, *count))
        })
        .into_grouping_map()
        .sum()
}

fn problem1(input: &Input) -> usize {
    let mut stones = input.iter().cloned().counts();
    for _n in 0..25 {
        stones = blink(&stones);
    }

    stones.values().sum()
}

fn problem2(input: &Input) -> usize {
    let mut stones = input.iter().cloned().counts();
    for _n in 0..75 {
        stones = blink(&stones);
    }

    stones.values().sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 55312)
    }
}
