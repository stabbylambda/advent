use common::{answer, read_input};
use nom::{
    branch::alt,
    character::complete::{alpha1, newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult, Parser,
};

fn main() {
    let input = read_input!();

    answer!(problem1(&parse(input, false)));
    answer!(problem2(&parse(input, true)));
}

type Input = Vec<Vec<u32>>;

fn parse(input: &str, include_words: bool) -> Input {
    let input = if include_words {
        // this feels so gross, but it's the easiest way to handle overlapping numbers in the input
        input
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .replace("zero", "zero0zero")
    } else {
        input.to_string()
    };

    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            many1(alt((
                map(one_of("1234567890"), |c| c.to_digit(10)),
                // discard any letters
                map(alpha1, |_| None),
            ))),
            |nums| nums.iter().filter_map(|x| *x).collect(),
        ),
    )
    .parse(&input);

    result.unwrap().1
}

fn make_number(digits: &[u32]) -> u32 {
    let first = digits.first();
    let last = digits.last().or(first);

    first.zip(last).map(|(x, y)| x * 10 + y).unwrap()
}

fn problem1(input: &Input) -> u32 {
    input.iter().map(|x| make_number(x)).sum()
}

fn problem2(input: &Input) -> u32 {
    input.iter().map(|x| make_number(x)).sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input, false);
        let result = problem1(&input);
        assert_eq!(result, 142)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input, true);
        let result = problem2(&input);
        assert_eq!(result, 281)
    }
}
