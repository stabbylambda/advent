use std::fmt::Display;

use common::{answer, read_input};
use itertools::Itertools;
fn main() {
    let input = read_input!();
    let input = parse(input);

    let ans1 = problem1(&input);
    answer!(ans1.len());

    let ans2 = problem2(&ans1);
    answer!(ans2.len());
}

#[derive(Debug)]
pub struct Digit {
    digit: u32,
    count: u32,
}

impl Digit {
    fn to_vec(&self) -> Vec<u32> {
        vec![self.count, self.digit]
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.count, self.digit)
    }
}

pub type Input = Vec<u32>;

fn translate_digits(input: Vec<Digit>) -> Vec<u32> {
    input.iter().flat_map(|x| x.to_vec()).collect_vec()
}

fn group_digits(input: &[u32]) -> Vec<Digit> {
    input
        .iter()
        .chunk_by(|x| **x)
        .into_iter()
        .map(|(digit, g)| {
            let count = g.count() as u32;

            Digit { digit, count }
        })
        .collect()
}

fn parse(input: &str) -> Input {
    input.chars().filter_map(|c| c.to_digit(10)).collect_vec()
}

fn problem1(input: &Input) -> Vec<u32> {
    let mut digits = input.to_owned();
    for _n in 0..40 {
        let x = group_digits(&digits[..]);
        digits = translate_digits(x);
    }
    digits
}

fn problem2(input: &Input) -> Vec<u32> {
    // we've already done 40 steps in problem 1, so we only need 10 more here
    let mut digits = input.to_owned();
    for _n in 0..10 {
        let x = group_digits(&digits[..]);
        digits = translate_digits(x);
    }
    digits
}

#[cfg(test)]
mod test {

    use itertools::Itertools;

    use crate::{group_digits, parse, Digit};
    fn digits_to_string(input: Vec<Digit>) -> String {
        input.iter().map(|x| x.to_string()).join("")
    }
    #[test]
    fn first() {
        let results = [
            ("1", "11"),
            ("11", "21"),
            ("21", "1211"),
            ("1211", "111221"),
            ("111221", "312211"),
        ];

        for (input, expected) in results {
            let input = parse(input);
            let result = group_digits(&input);
            let result = digits_to_string(result);

            assert_eq!(result, expected);
        }
    }
}
