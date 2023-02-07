use common::get_raw_input;
use nom::character::complete::char;
use nom::combinator::map;
use nom::{branch::alt, multi::many1, IResult};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<i32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        many1(alt((map(char('('), |_| 1), map(char(')'), |_| -1))))(input);

    result.unwrap().1
}

fn problem1(input: &[i32]) -> i32 {
    input.iter().sum()
}

fn problem2(input: &[i32]) -> usize {
    let mut floor = 0;
    for (step, dir) in input.iter().enumerate() {
        floor += dir;
        if floor == -1 {
            return step + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let tests = [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (input, expected) in tests {
            assert_eq!(problem1(&parse(input)[..]), expected)
        }
    }

    #[test]
    fn second() {
        let tests = [(")", 1), ("()())", 5)];
        for (input, expected) in tests {
            assert_eq!(problem2(&parse(input)[..]), expected)
        }
    }
}
