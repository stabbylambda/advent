use common::extensions::RangeExt;
use std::ops::RangeInclusive;

use nom::{
    character::complete::{char, u32 as nom_u32},
    combinator::map,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let lines = common::read_input!();
    let assignments = parse_assignments(lines);

    let answer = problem1(&assignments);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&assignments);
    println!("problem 2 answer: {answer}");
}

fn parse_range(s: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(
        separated_pair(nom_u32, char('-'), nom_u32),
        |(start, end)| start..=end,
    ).parse(s)
}
#[derive(Debug)]
struct Assignment {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}
impl Assignment {
    fn is_full_overlap(&self) -> bool {
        self.first.fully_contains(&self.second) || self.second.fully_contains(&self.first)
    }

    fn is_any_overlap(&self) -> bool {
        self.first.partially_contains(&self.second) || self.second.partially_contains(&self.first)
    }

    fn parse(s: &str) -> IResult<&str, Assignment> {
        map(
            separated_pair(parse_range, char(','), parse_range),
            |(first, second)| Assignment { first, second },
        ).parse(s)
    }
}

fn parse_assignments(input: &str) -> Vec<Assignment> {
    input
        .lines()
        .map(|s| Assignment::parse(s).unwrap().1)
        .collect()
}

fn problem1(assignments: &[Assignment]) -> u32 {
    assignments.iter().filter(|x| x.is_full_overlap()).count() as u32
}

fn problem2(assignments: &[Assignment]) -> u32 {
    assignments.iter().filter(|x| x.is_any_overlap()).count() as u32
}

#[cfg(test)]
mod test {

    use crate::{parse_assignments, problem1, problem2};
    #[test]
    fn first() {
        let lines = include_str!("../test.txt");
        let assignments = parse_assignments(lines);
        let result = problem1(&assignments);
        assert_eq!(result, 2)
    }

    #[test]
    fn second() {
        let lines = include_str!("../test.txt");
        let assignments = parse_assignments(lines);
        let result = problem2(&assignments);
        assert_eq!(result, 4)
    }
}
