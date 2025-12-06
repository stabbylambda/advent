use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64 as nom_i64, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input<'a> = HashMap<(&'a str, &'a str), i64>;

fn parse(input: &str) -> Input<'_> {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            map(
                (
                    terminated(alpha1, tag(" would ")),
                    alt((map(tag("gain "), |_| 1), map(tag("lose "), |_| -1))),
                    nom_i64,
                    delimited(
                        tag(" happiness units by sitting next to "),
                        alpha1,
                        tag("."),
                    ),
                ),
                |(name, mul, num, other)| ((name, other), mul * num),
            ),
        ),
        |v| v.into_iter().collect(),
    )
    .parse(input);

    result.unwrap().1
}

fn score_seating(input: &Input) -> i64 {
    let names: HashSet<&str> = input.keys().flat_map(|(x, y)| vec![*x, *y]).collect();
    names
        .iter()
        .permutations(names.len())
        .map(|p| {
            let mut v = p.clone();
            // add the first to the end so the tuple pairs work out
            v.extend(vec![p.first().unwrap()]);

            let happiness: i64 = v
                .iter()
                .tuple_windows()
                .map(|(&&x, &&y)| input[&(x, y)] + input[&(y, x)])
                .sum();

            happiness
        })
        .max()
        .unwrap()
}

fn problem1(input: &Input) -> i64 {
    score_seating(input)
}

fn problem2(input: &Input) -> i64 {
    let mut with_me = input.clone();
    let names: HashSet<&str> = input.keys().flat_map(|(x, y)| vec![*x, *y]).collect();

    for name in names {
        with_me.insert(("me", name), 0);
        with_me.insert((name, "me"), 0);
    }

    score_seating(&with_me)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 330)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 286)
    }
}
