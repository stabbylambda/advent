use std::collections::{HashMap, HashSet};

use common::get_raw_input;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64 as nom_i64, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input<'a> = HashMap<(&'a str, &'a str), i64>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            map(
                tuple((
                    terminated(alpha1, tag(" would ")),
                    alt((map(tag("gain "), |_| 1), map(tag("lose "), |_| -1))),
                    nom_i64,
                    delimited(
                        tag(" happiness units by sitting next to "),
                        alpha1,
                        tag("."),
                    ),
                )),
                |(name, mul, num, other)| ((name, other), mul * num),
            ),
        ),
        |v| v.into_iter().collect(),
    )(input);

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
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 0)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
