use std::time::Instant;

use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
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

type Input = Vec<Machine>;

fn parse(input: &str) -> Input {
    let xy_pair = |x| {
        separated_pair(
            preceded(preceded(tag("X"), one_of("+=")), i64),
            tag(", "),
            preceded(preceded(tag("Y"), one_of("+=")), i64),
        )(x)
    };

    let result: IResult<&str, Input> = separated_list1(
        tag("\n\n"),
        map(
            tuple((
                delimited(tag("Button A: "), xy_pair, newline),
                delimited(tag("Button B: "), xy_pair, newline),
                preceded(tag("Prize: "), xy_pair),
            )),
            |(a, b, prize)| Machine { a, b, prize },
        ),
    )(input);

    result.unwrap().1
}

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn solve(&self) -> Option<i64> {
        // We're going to apply Cramer's rule on:
        // ia + kb = m
        // ja + lb = n
        let (i, j) = self.a;
        let (k, l) = self.b;
        let (m, n) = self.prize;

        let determinant = i * l - k * j;
        if determinant == 0 {
            return None;
        }

        let a = divrem(m * l - k * n, determinant);
        let b = divrem(i * n - m * j, determinant);

        a.zip(b).map(|(a, b)| a * 3 + b)
    }
}

fn divrem(a: i64, b: i64) -> Option<i64> {
    // they have to be whole button presses, no fractions
    (a.rem_euclid(b) == 0).then(|| a.div_euclid(b))
}

fn problem1(input: &Input) -> i64 {
    input.iter().flat_map(|x| x.solve()).sum()
}

fn problem2(input: &Input) -> i64 {
    input
        .iter()
        .map(|m| {
            let (px, py) = m.prize;
            let prize = (px + 10000000000000, py + 10000000000000);
            Machine { prize, ..*m }
        })
        .flat_map(|x| x.solve())
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
        assert_eq!(result, 480)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 875318608908)
    }
}
