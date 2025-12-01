use common::math::chinese_remainder;
use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Disc>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            (
                preceded(tag("Disc #"), nom_i64),
                delimited(tag(" has "), nom_i64, tag(" positions;")),
                delimited(tag(" at time=0, it is at position "), nom_i64, tag(".")),
            ),
            |(_number, size, start)| Disc { size, start },
        ),
    ).parse(input);

    result.unwrap().1
}
#[derive(Clone)]
struct Disc {
    size: i64,
    start: i64,
}

fn problem(input: &Input) -> i64 {
    let residues: Vec<i64> = input
        .iter()
        .enumerate()
        .map(|(seconds, x)| {
            // adjust for the falling time + modulus
            let position_at_t = x.start + (seconds as i64) + 1;
            x.size - (position_at_t % x.size)
        })
        .collect();

    let modulii: Vec<i64> = input.iter().map(|x| x.size).collect();

    chinese_remainder(&residues, &modulii).unwrap()
}

fn problem1(input: &Input) -> i64 {
    problem(input)
}

fn problem2(input: &Input) -> i64 {
    let mut input = input.clone();
    input.push(Disc { size: 11, start: 0 });

    problem(&input)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 5)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 85)
    }
}
