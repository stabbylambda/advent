use core::num;
use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<(u64, Vec<u64>)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        separated_pair(u64, tag(": "), separated_list1(tag(" "), u64)),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u64 {
    input
        .iter()
        .filter_map(|(test_value, elements)| is_valid(test_value, elements).then_some(test_value))
        .sum()
}

fn is_valid(test_value: &u64, numbers: &[u64]) -> bool {
    let mut v = VecDeque::new();
    v.push_back((0, numbers[0]));

    while let Some((idx, current)) = v.pop_front() {
        let new_idx = idx + 1;
        if let Some(new_num) = numbers.get(new_idx) {
            v.push_back((new_idx, current + new_num));
            v.push_back((new_idx, current * new_num));
        } else if current == *test_value {
            return true;
        }
    }

    false
}

fn problem2(_input: &Input) -> u64 {
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
        assert_eq!(result, 3749)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
