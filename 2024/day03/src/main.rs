use common::nom::drop_till;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i32,
    combinator::map,
    multi::many1,
    sequence::{delimited, separated_pair},
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

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Enable,
    Disable,
}

type Input = Vec<Instruction>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = many1(drop_till(alt((
        map(
            delimited(tag("mul("), separated_pair(i32, tag(","), i32), tag(")")),
            |(a, b)| Instruction::Mul(a, b),
        ),
        map(tag("do()"), |_| Instruction::Enable),
        map(tag("don't()"), |_| Instruction::Disable),
    ))))(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> i32 {
    input
        .iter()
        .map(|x| match *x {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

fn problem2(input: &Input) -> i32 {
    let (_, result) = input.iter().fold((true, 0), |(enabled, acc), x| match *x {
        Instruction::Mul(a, b) if enabled => (enabled, acc + a * b),
        Instruction::Enable => (true, acc),
        Instruction::Disable => (false, acc),
        _ => (enabled, acc),
    });
    result
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 161)
    }

    #[test]
    fn second() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 48)
    }
}
