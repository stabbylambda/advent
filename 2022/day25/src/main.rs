use std::fmt::{Debug, Display};

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = Snafu::parse_all(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");
}

type Input = Vec<Snafu>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum SnafuNumber {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl From<&SnafuNumber> for i64 {
    fn from(val: &SnafuNumber) -> Self {
        match val {
            SnafuNumber::Two => 2,
            SnafuNumber::One => 1,
            SnafuNumber::Zero => 0,
            SnafuNumber::Minus => -1,
            SnafuNumber::DoubleMinus => -2,
        }
    }
}

impl Debug for SnafuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnafuNumber::Two => write!(f, "2"),
            SnafuNumber::One => write!(f, "1"),
            SnafuNumber::Zero => write!(f, "0"),
            SnafuNumber::Minus => write!(f, "-"),
            SnafuNumber::DoubleMinus => write!(f, "="),
        }
    }
}

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Snafu {
    numbers: Vec<SnafuNumber>,
}

impl Snafu {
    fn parse(s: &str) -> IResult<&str, Snafu> {
        map(
            many1(alt((
                map(char('2'), |_| SnafuNumber::Two),
                map(char('1'), |_| SnafuNumber::One),
                map(char('0'), |_| SnafuNumber::Zero),
                map(char('-'), |_| SnafuNumber::Minus),
                map(char('='), |_| SnafuNumber::DoubleMinus),
            ))),
            |numbers| Snafu { numbers },
        )(s)
    }
    fn parse_all(input: &str) -> Input {
        let result: IResult<&str, Input> = separated_list1(newline, Snafu::parse)(input);
        result.unwrap().1
    }
}

impl From<&Snafu> for i64 {
    fn from(value: &Snafu) -> Self {
        let mut result = 0;
        for (i, n) in value.numbers.iter().rev().enumerate() {
            let power = i64::pow(5, i as u32);
            let num: i64 = n.into();

            result += num * power;
        }

        result
    }
}

impl From<i64> for Snafu {
    fn from(val: i64) -> Self {
        let mut numbers: Vec<SnafuNumber> = vec![];
        let mut n = val;
        while n != 0 {
            let remaining: i64 = n % 5;
            let digit = match remaining {
                2 => SnafuNumber::Two,
                1 => SnafuNumber::One,
                0 => SnafuNumber::Zero,
                4 => SnafuNumber::Minus,
                3 => SnafuNumber::DoubleMinus,
                _ => unreachable!(),
            };

            numbers.push(digit);
            n = (n + 2) / 5;
        }

        numbers.reverse();

        Snafu { numbers }
    }
}

impl From<&str> for Snafu {
    fn from(value: &str) -> Self {
        Snafu::parse(value).unwrap().1
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for num in self.numbers.iter() {
            write!(f, "{num}")?;
        }

        write!(f, "")
    }
}

fn problem1(input: &Input) -> Snafu {
    let n: i64 = input
        .iter()
        .map(|s| {
            let i: i64 = s.into();
            i
        })
        .sum();

    n.into()
}

#[cfg(test)]
mod test {

    use crate::{problem1, Snafu};

    #[test]
    fn examples() {
        let data = [
            (1, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (10, "20"),
            (11, "21"),
            (12, "22"),
            (13, "1=="),
            (14, "1=-"),
            (15, "1=0"),
            (15, "1=0"),
            (20, "1-0"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ];

        for (dec, snafu) in data {
            let result: Snafu = dec.into();
            let expected: Snafu = snafu.into();

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = Snafu::parse_all(input);
        let result = problem1(&input);
        let expected: Snafu = 4890.into();
        assert_eq!(result, expected)
    }
}
