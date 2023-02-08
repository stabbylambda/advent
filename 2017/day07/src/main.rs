use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let root = problem1(&input);
    println!("problem 1 score: {root}");

    let score = problem2(&input, root);
    println!("problem 2 score: {score}");
}

type Input<'a> = HashMap<&'a str, Program<'a>>;

#[derive(Debug)]
struct Program<'a> {
    name: &'a str,
    weight: u32,
    programs: Vec<&'a str>,
}

impl<'a> Display for Program<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.weight)?;
        if !self.programs.is_empty() {
            let programs = self.programs.join(", ");
            write!(f, " -> {programs}")?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            tuple((
                terminated(alpha1, tag(" ")),
                delimited(tag("("), u32, tag(")")),
                opt(preceded(tag(" -> "), separated_list1(tag(", "), alpha1))),
            )),
        ),
        |v| {
            let map: HashMap<_, _> = v
                .into_iter()
                .map(|(name, weight, programs)| {
                    (
                        name,
                        Program {
                            name,
                            weight,
                            programs: programs.unwrap_or_default(),
                        },
                    )
                })
                .collect();

            map
        },
    )(input);

    result.unwrap().1
}

fn problem1<'a>(input: &'a Input) -> &'a str {
    let not_root: HashSet<&str> = input
        .iter()
        .flat_map(|(_name, p)| p.programs.iter().cloned())
        .collect();
    let all: HashSet<&str> = input.iter().map(|(&name, _p)| name).collect();

    all.difference(&not_root).next().unwrap()
}

fn problem2(input: &Input, root: &str) -> u32 {
    let branches: Vec<(&str, u32, u32)> = input[root]
        .programs
        .iter()
        .map(|&name| (name, input[name].weight, get_weight(input, name)))
        .collect();

    let min_stack = branches
        .iter()
        .min_by_key(|(name, self_weight, total_weight)| total_weight)
        .unwrap();
    let max_stack = branches
        .iter()
        .max_by_key(|(name, self_weight, total_weight)| total_weight)
        .unwrap();

    let diff = max_stack.2 - min_stack.2;
    let would_be = max_stack.1 - diff;
    dbg!(&branches, diff, would_be);

    would_be
}

fn get_weight(input: &HashMap<&str, Program>, name: &str) -> u32 {
    let p = &input[name];
    p.programs
        .iter()
        .fold(p.weight, |acc, x| acc + get_weight(input, x))
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
        let result = problem2(&input, "tknk");
        assert_eq!(result, 60)
    }
}
