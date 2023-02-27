use std::collections::HashSet;

use nom::{
    branch::alt,
    character::complete::{char, i32, newline},
    multi::separated_list1,
    sequence::preceded,
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

type Input = Vec<i32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, alt((preceded(char('+'), i32), i32)))(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> i32 {
    input.iter().sum()
}

fn problem2(input: &Input) -> i32 {
    let mut frequency: i32 = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    for x in input.iter().cycle() {
        frequency += x;

        if !seen.insert(frequency) {
            return frequency;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 3)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 2)
    }
}
