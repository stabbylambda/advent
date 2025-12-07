use std::{collections::BTreeMap, fmt::Debug};

use common::{answer, read_input};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));

    let input = read_input!();
    let input = input
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31");
    let input = parse(&input);

    answer!(problem2(&input));
}

type Input = (BTreeMap<u32, Rule>, Vec<String>);

#[derive(Debug, PartialEq, Eq)]
enum Rule {
    Literal(char),
    String(Vec<u32>),
    Or(Vec<u32>, Vec<u32>),
}

impl Rule {
    fn matches<'a>(
        &self,
        rules: &'a BTreeMap<u32, Rule>,
        remainder: &'a [char],
    ) -> Vec<&'a [char]> {
        if remainder.is_empty() {
            return vec![];
        }

        match self {
            // "Consume" one char and move on with the parsing
            Rule::Literal(c) if remainder[0] == *c => vec![&remainder[1..]],
            // bail on this branch
            Rule::Literal(_) => vec![],
            Rule::String(keys) if keys.len() == 1 => {
                let a = &rules[&keys[0]];

                a.matches(rules, remainder)
            }
            Rule::String(keys) if keys.len() == 2 => {
                let a = &rules[&keys[0]];
                let b = &rules[&keys[1]];

                a.matches(rules, remainder)
                    .into_iter()
                    .flat_map(|x| b.matches(rules, x))
                    .collect()
            }
            Rule::String(keys) if keys.len() == 3 => {
                let a = &rules[&keys[0]];
                let b = &rules[&keys[1]];
                let c = &rules[&keys[2]];

                a.matches(rules, remainder)
                    .into_iter()
                    .flat_map(|x| b.matches(rules, x))
                    .flat_map(|x| c.matches(rules, x))
                    .collect()
            }
            Rule::Or(a, b) => {
                let mut result = vec![];
                let a_results = Rule::String(a.clone()).matches(rules, remainder);
                let b_results = Rule::String(b.clone()).matches(rules, remainder);

                result.extend(a_results);
                result.extend(b_results);

                result
            }

            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Input {
    fn rule(s: &str) -> IResult<&str, Rule> {
        alt((
            delimited(tag("\""), map(anychar, Rule::Literal), tag("\"")),
            map(
                separated_pair(
                    separated_list1(tag(" "), u32),
                    tag(" | "),
                    separated_list1(tag(" "), u32),
                ),
                |(a, b)| Rule::Or(a, b),
            ),
            map(separated_list1(tag(" "), u32), Rule::String),
        )).parse(s)
    }

    let rules = |s| {
        map(
            separated_list1(newline, separated_pair(u32, tag(": "), rule)),
            |x| x.into_iter().collect(),
        ).parse(s)
    };
    let messages = |s| separated_list1(newline, map(take_until("\n"), |s: &str| s.to_string())).parse(s);
    let result: IResult<&str, Input> = separated_pair(rules, tag("\n\n"), messages).parse(input);

    result.unwrap().1
}

fn problem1((rules, messages): &Input) -> usize {
    let root = rules.get(&0).unwrap();

    messages
        .iter()
        .filter(|x| {
            let cs = x.chars().collect::<Vec<char>>();
            root.matches(rules, &cs).iter().any(|m| m.is_empty())
        })
        .count()
}

fn problem2((rules, messages): &Input) -> usize {
    let root = rules.get(&0).unwrap();

    messages
        .iter()
        .filter(|x| {
            let cs = x.chars().collect::<Vec<char>>();
            root.matches(rules, &cs).iter().any(|m| m.is_empty())
        })
        .count()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 2)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = input
            .replace("8: 42", "8: 42 | 42 8")
            .replace("11: 42 31", "11: 42 31 | 42 11 31");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 12)
    }
}
