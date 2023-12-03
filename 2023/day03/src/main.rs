use std::ascii::AsciiExt;

use common::{
    map::{Map, MapSquare},
    nom::single_digit,
};
use nom::{
    branch::alt,
    character::complete::{char, newline, one_of, u32},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

#[derive(Clone, Copy, Debug)]
enum SchematicPart {
    Symbol(char),
    Number(u32),
    Blank,
}

impl std::fmt::Display for SchematicPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchematicPart::Symbol(c) => write!(f, "{c}"),
            SchematicPart::Number(n) => write!(f, "{n}"),
            SchematicPart::Blank => write!(f, "."),
        }
    }
}

type Input = Map<SchematicPart>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(alt((
                map(char('.'), |_| SchematicPart::Blank),
                map(single_digit, SchematicPart::Number),
                map(one_of("!@#$%^&*+-=/\\"), SchematicPart::Symbol),
            ))),
        ),
        Map::new,
    )(input);

    let r = result.unwrap();
    println!("{}", r.0);

    r.1
}

fn next_to_symbol(x: &MapSquare<SchematicPart>) -> bool {
    x.all_neighbors()
        .into_iter()
        .any(|x| matches!(x.data, SchematicPart::Symbol(..)))
}

fn problem1(input: &Input) -> u32 {
    let mut total = 0;
    let mut examined: Vec<(usize, usize)> = vec![];
    for x in input {
        // have we already been here?
        if examined.contains(&x.coords) {
            continue;
        }

        // if we're on a number
        if let SchematicPart::Number(n) = x.data {
            let mut current = x;
            let mut num = *n;
            let mut is_adjacent_to_symbol = next_to_symbol(&x);

            // go right until we don't have a number anymore
            while let Some(SchematicPart::Number(next)) = current.neighbors().east.map(|x| x.data) {
                // are we adjacent to a symbol here?
                num = (num * 10) + *next;
                current = current.neighbors().east.unwrap();
                is_adjacent_to_symbol |= next_to_symbol(&current);

                // mark that we've seen the successor square and don't need to visit it again
                examined.push(current.coords);
            }

            // we know the whole number now
            if is_adjacent_to_symbol {
                total += num;
            }
        }
    }
    total
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 4361)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
