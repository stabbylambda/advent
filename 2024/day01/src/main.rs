use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = (Vec<u32>, Vec<u32>);

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(newline, separated_pair(u32, tag("   "), u32)),
        |x| x.into_iter().unzip(),
    ).parse(input);

    result.unwrap().1
}

fn problem1((left, right): &Input) -> u32 {
    let diff_sum = left
        .iter()
        .sorted()
        .zip(right.iter().sorted())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    diff_sum
}

fn problem2((left, right): &Input) -> u32 {
    let counts = right.iter().counts();

    left.iter()
        .map(|l| {
            let times = *counts.get(l).unwrap_or(&0);

            l * (times as u32)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 31)
    }
}
