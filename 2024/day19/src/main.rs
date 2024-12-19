use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashMap, time::Instant};

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

type Input<'a> = (Vec<&'a str>, Vec<&'a str>);

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_pair(
        separated_list1(tag(", "), alpha1),
        tag("\n\n"),
        separated_list1(newline, alpha1),
    )(input);

    result.unwrap().1
}

fn count_combos<'a>(towel: &'a str, designs: &[&str], memo: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(valid) = memo.get(towel) {
        return *valid;
    }

    designs
        .iter()
        .flat_map(|design| towel.strip_prefix(design))
        .map(|rest| {
            let valid = count_combos(rest, designs, memo);
            memo.insert(rest, valid);
            valid
        })
        .sum()
}

fn solve(input: &Input) -> Vec<u64> {
    let (designs, towels) = input;

    // start a dynamic programming cache with the empty string being valid
    let mut memo: HashMap<&str, u64> = HashMap::new();
    memo.insert("", 1);

    towels
        .iter()
        .map(|x| count_combos(x, &designs[..], &mut memo))
        .collect()
}

fn problem1(input: &Input) -> usize {
    solve(input).iter().filter(|x| **x > 0).count()
}

fn problem2(input: &Input) -> u64 {
    solve(input).iter().sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 6)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 16)
    }
}
