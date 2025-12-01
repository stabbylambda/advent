use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let root = problem1(&input);
    println!("problem 1 score: {root}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input<'a> = Program<'a>;

#[derive(Debug, PartialEq, Eq)]
struct Program<'a> {
    name: &'a str,
    weight: u32,
    programs: Vec<Program<'a>>,
}
impl<'a> Program<'a> {
    fn get_total_weight(&self) -> u32 {
        self.programs
            .iter()
            .fold(self.weight, |acc, x| acc + x.get_total_weight())
    }

    fn all_children_balanced(&self) -> bool {
        let set: HashSet<u32> = self.programs.iter().map(|x| x.get_total_weight()).collect();
        set.len() == 1
    }
}

impl<'a> PartialOrd for Program<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Program<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_total_weight().cmp(&other.get_total_weight())
    }
}
impl<'a> Display for Program<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.weight)?;
        if !self.programs.is_empty() {
            let programs = self
                .programs
                .iter()
                .map(|x| x.name)
                .collect::<Vec<&str>>()
                .join(", ");
            write!(f, " -> {programs}")?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> Input<'_> {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            (
                terminated(alpha1, tag(" ")),
                delimited(tag("("), u32, tag(")")),
                map(
                    opt(preceded(tag(" -> "), separated_list1(tag(", "), alpha1))),
                    |v| v.unwrap_or_default(),
                ),
            ),
        ),
        |v| {
            let map: HashMap<_, _> = v
                .clone()
                .into_iter()
                .map(|(name, weight, programs)| (name, (weight, programs)))
                .collect();
            let not_root: HashSet<&str> = v
                .iter()
                .flat_map(|(_, _, programs)| programs.iter().cloned())
                .collect();
            let all: HashSet<&str> = v.iter().map(|(name, _, _)| *name).collect();
            let root = *all.difference(&not_root).next().unwrap();

            build_tree(&map, root)
        },
    )
    .parse(input);

    result.unwrap().1
}

fn build_tree<'a>(map: &HashMap<&'a str, (u32, Vec<&'a str>)>, name: &'a str) -> Program<'a> {
    let (weight, children) = &map[name];
    let programs = children.iter().map(|&x| build_tree(map, x)).collect();

    Program {
        name,
        weight: *weight,
        programs,
    }
}

fn problem1<'a>(input: &'a Input) -> &'a str {
    input.name
}

fn problem2(input: &Input) -> u32 {
    let mut current = input;

    let result = loop {
        let max_child = current.programs.iter().max().unwrap();
        let min_child = current.programs.iter().min().unwrap();

        // the current node is the problem
        if max_child.all_children_balanced() {
            let diff = max_child.get_total_weight() - min_child.get_total_weight();
            break max_child.weight - diff;
        } else {
            current = max_child;
        }
    };

    result
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, "tknk")
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 60)
    }
}
