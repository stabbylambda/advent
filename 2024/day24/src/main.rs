use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum GateKind {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Gate<'a> {
    input1: &'a str,
    input2: &'a str,
    kind: GateKind,
    output: &'a str,
}

impl<'a> Gate<'a> {
    fn is_lsb(&self) -> bool {
        (self.input1 == "x00" || self.input1 == "y00")
            || (self.input2 == "x00" || self.input2 == "y00")
    }

    fn is_inner_gate(&self) -> bool {
        let i1 = ['x', 'y', 'z'].contains(&self.input1.chars().next().unwrap());
        let i2 = ['x', 'y', 'z'].contains(&self.input2.chars().next().unwrap());
        let o = ['x', 'y', 'z'].contains(&self.output.chars().next().unwrap());

        !i1 && !i2 && !o
    }
}

impl Display for Gate<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self.kind {
            GateKind::And => "AND",
            GateKind::Or => "OR",
            GateKind::Xor => "XOR",
        };
        write!(
            f,
            "{} {} {} -> {}",
            self.input1, kind, self.input2, self.output
        )
    }
}

fn parse(input: &str) -> Input<'_> {
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

        // check if we have any z gates left
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

fn problem2(input: &Input) -> String {
    let gates = &input.1;

    let wrong_zout = gates
        .iter()
        .filter(|g| g.output.starts_with("z"))
        .filter(|g| g.output != "z45")
        .filter(|g| g.kind != GateKind::Xor)
        .collect_vec();

    let more_wrong_xors = gates
        .iter()
        .filter(|g| g.kind == GateKind::Xor)
        .filter(|g| {
            // check the subgates for the wrong pattern
            let subgates_wrong = gates
                .iter()
                .filter(|sg| sg.kind == GateKind::Or)
                .any(|sg| sg.input1 == g.output || sg.input2 == g.output);

            g.is_inner_gate() || subgates_wrong
        })
        .collect_vec();

    let wrong_ands = gates
        .iter()
        .filter(|g| !g.is_lsb())
        .filter(|g| g.kind == GateKind::And)
        .filter(|g| {
            !gates
                .iter()
                .filter(|sg| sg.kind == GateKind::Or)
                .any(|sg| sg.input1 == g.output || sg.input2 == g.output)
        })
        .collect_vec();

    let mut swapped: HashSet<&Gate> = HashSet::new();
    swapped.extend(&wrong_zout);
    swapped.extend(&more_wrong_xors);
    swapped.extend(&wrong_ands);

    swapped
        .iter()
        .map(|g| g.output)
        .sorted()
        .join(",")
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
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
}
