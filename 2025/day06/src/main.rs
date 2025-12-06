use std::hint::unreachable_unchecked;

use nom::{
    branch::alt,
    character::complete::{char, newline, space0, space1, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Column>;

#[derive(Debug)]
struct Column {
    numbers: Vec<u64>,
    operation: char,
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_pair(
            separated_list1(newline, many1(delimited(space0, u64, space0))),
            newline,
            separated_list1(space0, alt((char('+'), char('*')))),
        ),
        |(rows, operations)| {
            operations
                .iter()
                .enumerate()
                .map(|(idx, &operation)| Column {
                    numbers: rows.iter().map(|r| r[idx]).collect(),
                    operation,
                })
                .collect()
        },
    )
    .parse(input);

    result.unwrap().1
}

fn problem1(x: &Input) -> u64 {
    x.iter()
        .map(|c| {
            c.numbers
                .iter()
                .cloned()
                .reduce(|acc, x| match c.operation {
                    '+' => acc + x,
                    '*' => acc * x,
                    _ => unreachable!(),
                })
                .unwrap()
        })
        .sum()
}

fn problem2(x: &Input) -> u32 {
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
        assert_eq!(result, 4277556);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
