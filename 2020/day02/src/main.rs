use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline, not_line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

use common::nom::usize;

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<((usize, usize, char), String)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        separated_pair(
            (
                terminated(usize, tag("-")),
                terminated(usize, tag(" ")),
                anychar,
            ),
            tag(": "),
            map(not_line_ending, |x: &str| x.to_string()),
        ),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    input
        .iter()
        .filter(|((low, high, c), password)| {
            let actual = password.chars().filter(|x| x == c).count();
            *low <= actual && actual <= *high
        })
        .count()
}

fn problem2(input: &Input) -> usize {
    input
        .iter()
        .filter(|((idx1, idx2, c), password)| {
            let first = password.chars().nth(idx1 - 1).unwrap();
            let second = password.chars().nth(idx2 - 1).unwrap();

            (first == *c && second != *c) || (first != *c && second == *c)
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
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1)
    }
}
