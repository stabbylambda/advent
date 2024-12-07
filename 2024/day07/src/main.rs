use std::{collections::VecDeque, time::Instant};

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

    let i = Instant::now();
    let score = problem1(&input);
    let d = i.elapsed();
    println!("problem 1 score: {score} in {d:?}");

    let i = Instant::now();
    let score = problem2(&input);
    let d = i.elapsed();
    println!("problem 2 score: {score} in {d:?}");
}

type Input = Vec<(u64, Vec<u64>)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        separated_pair(u64, tag(": "), separated_list1(tag(" "), u64)),
    )(input);

    result.unwrap().1
}

/// concat two numbers (e.g. concat_numbers(123, 45) == 12345)
fn concat_numbers(a: u64, b: u64) -> u64 {
    let exp = (b as f64).log10().ceil() as u32;
    // multiple a by 10^c to "shift" it over so we can concat with b by adding
    a * 10_u64.pow(exp) + b
}

fn is_valid(test_value: &u64, numbers: &[u64], allow_concat: bool) -> bool {
    let mut v = VecDeque::new();
    v.push_back((0, numbers[0]));

    while let Some((idx, current)) = v.pop_front() {
        // all the operators cause the number to get bigger, bail if we go over
        if current > *test_value {
            continue;
        }

        let new_idx = idx + 1;
        if let Some(new_num) = numbers.get(new_idx) {
            v.push_back((new_idx, current + new_num));
            v.push_back((new_idx, current * new_num));
            if allow_concat {
                v.push_back((new_idx, concat_numbers(current, *new_num)));
            }
        } else if current == *test_value {
            // if we're done with the numbers and we hit the test value, return true
            return true;
        }
    }

    false
}

fn problem1(input: &Input) -> u64 {
    input
        .iter()
        .filter_map(|(test_value, elements)| {
            is_valid(test_value, elements, false).then_some(test_value)
        })
        .sum()
}

fn problem2(input: &Input) -> u64 {
    input
        .iter()
        .filter_map(|(test_value, elements)| {
            is_valid(test_value, elements, true).then_some(test_value)
        })
        .sum()
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
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 11387)
    }
}
