use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    ops::RangeInclusive,
};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Notes;

fn parse(input: &str) -> Input {
    let ticket = |s| map(separated_list1(tag(","), u64), Ticket)(s);

    let range = |s| map(separated_pair(u64, tag("-"), u64), |(l, h)| l..=h)(s);

    let rule = |s| {
        map(
            separated_pair(
                map(take_until(":"), |s: &str| s.to_string()),
                tag(": "),
                separated_pair(range, tag(" or "), range),
            ),
            |(name, (range1, range2))| Rule {
                name,
                range1,
                range2,
            },
        )(s)
    };

    let result: IResult<&str, Input> = map(
        tuple((
            terminated(separated_list1(newline, rule), tag("\n\n")),
            delimited(tag("your ticket:\n"), ticket, tag("\n\n")),
            preceded(tag("nearby tickets:\n"), separated_list1(newline, ticket)),
        )),
        |(rules, ticket, nearby)| Notes {
            rules,
            ticket,
            nearby,
        },
    )(input);

    result.unwrap().1
}

struct Rule {
    name: String,
    range1: RangeInclusive<u64>,
    range2: RangeInclusive<u64>,
}

impl Rule {
    fn is_valid(&self, field: u64) -> bool {
        let valid1 = self.range1.contains(&field);
        let valid2 = self.range2.contains(&field);

        valid1 || valid2
    }
}

struct Ticket(Vec<u64>);

impl Ticket {
    fn is_valid(&self, rules: &[Rule]) -> bool {
        self.invalid_fields(rules).is_empty()
    }

    fn invalid_fields(&self, rules: &[Rule]) -> Vec<u64> {
        self.0
            .iter()
            .filter(|field| !rules.iter().any(|rule| rule.is_valid(**field)))
            .copied()
            .collect()
    }
}

impl Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.0)
    }
}

struct Notes {
    rules: Vec<Rule>,
    ticket: Ticket,
    nearby: Vec<Ticket>,
}

fn problem1(input: &Input) -> u64 {
    input
        .nearby
        .iter()
        .flat_map(|ticket| ticket.invalid_fields(&input.rules))
        .sum()
}

fn problem2(input: &Input) -> u64 {
    let valid_tickets: Vec<&Ticket> = input
        .nearby
        .iter()
        .filter(|ticket| ticket.is_valid(&input.rules))
        .collect();

    let mut possibles: Vec<(String, BTreeSet<usize>)> = input
        .rules
        .iter()
        .map(|rule| {
            let possible_columns = (0..input.rules.len())
                .filter(|i| valid_tickets.iter().all(|t| rule.is_valid(t.0[*i])))
                .collect();
            (rule.name.to_string(), possible_columns)
        })
        .collect();

    // sort by least matches
    possibles.sort_by_key(|x| x.1.len());

    // pull off each of the matched columns and associate them with a name
    let mut matches: BTreeMap<String, usize> = BTreeMap::new();
    for (name, possible) in &possibles {
        let already_matched = matches.values().copied().collect();
        if let Some(unmatched) = possible.difference(&already_matched).next() {
            matches.insert(name.to_string(), *unmatched);
        };
    }

    // multiply all the fields that start with product
    matches
        .iter()
        .filter_map(|(name, index)| {
            name.starts_with("departure")
                .then_some(input.ticket.0[*index])
        })
        .product()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 71)
    }

    #[test]
    fn second() {
        let input = include_str!("../input.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 4381476149273)
    }
}
