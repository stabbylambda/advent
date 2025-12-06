use common::{answer, read_input};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = (Rules, Vec<Manual>);

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_pair(
        map(
            separated_list1(newline, separated_pair(u32, tag("|"), u32)),
            Rules::new,
        ),
        tag("\n\n"),
        separated_list1(newline, map(separated_list1(tag(","), u32), Manual::new)),
    ).parse(input);

    result.unwrap().1
}

struct Manual {
    pages: Vec<u32>,
}

impl Manual {
    fn new(pages: Vec<u32>) -> Self {
        Self { pages }
    }

    fn middle(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }

    fn reorder(&self, rules: &Rules) -> Self {
        Manual {
            pages: self
                .pages
                .iter()
                .sorted_by(|a, b| rules.cmp(a, b))
                .cloned()
                .collect(),
        }
    }
}

struct Rules {
    rules: HashMap<u32, Vec<u32>>,
}

impl Rules {
    fn new(v: Vec<(u32, u32)>) -> Self {
        let rules = v.into_iter().map(|(a, b)| (b, a)).into_group_map();

        Self { rules }
    }

    fn cmp(&self, a: &u32, b: &u32) -> Ordering {
        self.rules
            .get(a)
            .and_then(|x| x.contains(b).then_some(Ordering::Less))
            .unwrap_or(Ordering::Greater)
    }

    fn is_valid(&self, manual: &Manual) -> bool {
        manual
            .pages
            .is_sorted_by(|a, b| self.cmp(a, b) == Ordering::Greater)
    }
}

fn problem1(input: &Input) -> u32 {
    let (rules, manuals) = input;

    manuals
        .iter()
        .filter(|x| rules.is_valid(x))
        .map(|x| x.middle())
        .sum()
}

fn problem2(input: &Input) -> u32 {
    let (rules, manuals) = input;

    manuals
        .iter()
        .filter(|x| !rules.is_valid(x))
        .map(|x| x.reorder(rules))
        .map(|x| x.middle())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn one() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = input.0.is_valid(input.1.first().unwrap());
        assert!(result)
    }

    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 143)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 123)
    }
}
