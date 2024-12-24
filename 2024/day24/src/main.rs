use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use std::{
    cmp::Reverse,
    collections::{HashMap, VecDeque},
    time::Instant,
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

type Input<'a> = (HashMap<&'a str, u64>, Vec<Gate<'a>>);

#[derive(Clone, Copy, Debug)]
enum GateKind {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy, Debug)]
struct Gate<'a> {
    input1: &'a str,
    input2: &'a str,
    kind: GateKind,
    output: &'a str,
}

fn parse(input: &str) -> Input {
    let gate = |s| {
        map(
            tuple((
                terminated(alphanumeric1, tag(" ")),
                alt((
                    map(tag("AND "), |_| GateKind::And),
                    map(tag("OR "), |_| GateKind::Or),
                    map(tag("XOR "), |_| GateKind::Xor),
                )),
                terminated(alphanumeric1, tag(" -> ")),
                alphanumeric1,
            )),
            |(input1, kind, input2, output)| Gate {
                input1,
                input2,
                kind,
                output,
            },
        )(s)
    };

    let result: IResult<&str, Input> = separated_pair(
        map(
            separated_list1(newline, separated_pair(alphanumeric1, tag(": "), u64)),
            |x| x.into_iter().collect(),
        ),
        tag("\n\n"),
        separated_list1(newline, gate),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u64 {
    let (mut wires, gates) = input.clone();
    let mut gates = VecDeque::from_iter(&gates);

    while let Some(gate) = gates.pop_front() {
        let Some(i1) = wires.get(gate.input1) else {
            gates.push_back(gate);
            continue;
        };

        let Some(i2) = wires.get(gate.input2) else {
            gates.push_back(gate);
            continue;
        };

        let output = match gate.kind {
            GateKind::And => i1 & i2,
            GateKind::Or => i1 | i2,
            GateKind::Xor => i1 ^ i2,
        };

        wires.insert(gate.output, output);

        if !gates.iter().any(|x| x.output.starts_with("z")) {
            break;
        }
    }

    let result = wires
        .iter()
        .filter(|x| x.0.starts_with("z"))
        .sorted_by_key(|x| x.0)
        .enumerate()
        .fold(0, |acc, (s, (_k, v))| acc | (v << s));

    result
}

fn problem2(input: &Input) -> u64 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 4)
    }

    #[test]
    fn first_larger() {
        let input = include_str!("../test_larger.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 2024)
    }

    #[test]
    #[ignore]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
