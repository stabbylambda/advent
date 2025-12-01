use common::math::chinese_remainder;
use nom::{
    branch::alt,
    character::complete::{char, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = (u32, Vec<Option<u32>>);

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_pair(
        u32,
        newline,
        separated_list1(char(','), alt((map(u32, Some), map(char('x'), |_| None)))),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    let (timestamp, busses) = input;

    let (wait, id) = busses
        .iter()
        .flatten()
        .map(|t| (t, t - (timestamp % t)))
        .min_by_key(|x| x.1)
        .unwrap();

    id * wait
}

fn problem2(input: &Input) -> i64 {
    let (_timestamp, busses) = input;
    let pairs: Vec<(i64, i64)> = busses
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|b| (b as i64, b as i64 - i as i64)))
        .collect();

    let busses: Vec<i64> = pairs.iter().map(|x| x.0).collect();
    let offsets: Vec<i64> = pairs.iter().map(|x| x.1).collect();

    chinese_remainder(&offsets, &busses).unwrap()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 295)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1068781)
    }
}
