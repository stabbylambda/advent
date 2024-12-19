use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashMap, mem, time::Instant};

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

fn is_valid<'a>(towel: &'a str, designs: &[&str], memo: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(valid) = memo.get(towel) {
        return *valid;
    }

    designs.iter().any(|design| {
        towel
            .strip_prefix(design)
            .map(|rest| {
                let valid = is_valid(rest, designs, memo);
                memo.insert(rest, valid);
                valid
            })
            .unwrap_or_default()
    })
}

fn problem1(input: &Input) -> usize {
    let (designs, towels) = input;

    // start a dynamic programming cache with the empty string being valid
    let mut memo: HashMap<&str, bool> = HashMap::new();
    memo.insert("", true);

    towels
        .iter()
        .filter(|x| is_valid(x, &designs[..], &mut memo))
        .count()
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
        assert_eq!(result, 6)
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
