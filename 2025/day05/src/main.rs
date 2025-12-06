use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::{
        complete::{char, i32, newline},
        streaming::u64,
    },
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
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

type Input = Inventory;
struct Inventory {
    ranges: Vec<RangeInclusive<u64>>,
    items: Vec<u64>,
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_pair(
            separated_list1(
                newline,
                map(separated_pair(u64, char('-'), u64), |(a, b)| a..=b),
            ),
            tag("\n\n"),
            separated_list1(newline, u64),
        ),
        |(ranges, items)| Inventory { ranges, items },
    )
    .parse(input);

    result.unwrap().1
}

fn problem1(x: &Input) -> usize {
    x.items
        .iter()
        .filter(|i| x.ranges.iter().any(|r| r.contains(i)))
        .count()
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
        assert_eq!(result, 3);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
