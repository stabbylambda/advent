use common::{answer, read_input};
use nom::{
    character::complete::{newline, u64},
    multi::separated_list1,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    let answer1 = problem1(&input);
    answer!(answer1);
    answer!(problem2(&input, answer1));
}

type Input = Vec<u64>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, u64).parse(input);

    result.unwrap().1
}

fn first_error(input: &[u64], preamble_size: usize) -> u64 {
    // grab the digit after the preamble
    if let Some(digit) = input.get(preamble_size + 1) {
        let preamble = &input[0..=preamble_size];

        // for each digit x in the preamble, check if digit - x  is in the preamble
        let valid = preamble
            .iter()
            .filter_map(|x| digit.checked_sub(*x).filter(|y| x != y))
            .any(|y| preamble.contains(&y));

        return if valid {
            first_error(&input[1..], preamble_size)
        } else {
            *digit
        };
    }

    unreachable!("This code is valid?")
}

fn problem1(input: &Input) -> u64 {
    first_error(input, 25)
}

fn problem2(input: &Input, goal: u64) -> u64 {
    let idx = input.iter().position(|x| *x == goal).unwrap();

    // this is the naive approach, start from the goal and work backwards, taking decreasing slices and summing them
    // it's fast enough though
    for n in (0..idx).rev() {
        for size in 0..n {
            let slice = &input[size..n];
            let summed: u64 = slice.iter().sum();
            if summed == goal {
                return slice.iter().min().unwrap() + slice.iter().max().unwrap();
            }
        }
    }

    0
}

#[cfg(test)]
mod test {
    use crate::{first_error, parse, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = first_error(&input, 5);
        assert_eq!(result, 127)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input, 127);
        assert_eq!(result, 62)
    }
}
