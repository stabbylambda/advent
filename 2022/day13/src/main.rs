use std::fmt::Debug;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Scalar(u32),
    List(Vec<Self>),
}

impl Packet {
    fn as_slice(&self) -> &[Self] {
        match self {
            x @ Packet::Scalar(_) => std::slice::from_ref(x),
            Packet::List(l) => l.as_slice(),
        }
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(arg0) => write!(f, "{arg0}"),
            Self::List(arg0) => {
                write!(f, "[")?;
                for x in arg0 {
                    write!(f, "{x:?},")?;
                }
                write!(f, "]")
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Scalar(x), Packet::Scalar(y)) => x.cmp(y),
            _ => self.as_slice().cmp(other.as_slice()),
        }
    }
}
type Input = Vec<(Packet, Packet)>;

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    map(
        delimited(
            tag("["),
            separated_list0(tag(","), alt((map(nom_u32, Packet::Scalar), parse_packet))),
            tag("]"),
        ),
        Packet::List,
    )(input)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list0(
        tag("\n\n"),
        separated_pair(parse_packet, newline, parse_packet),
    )(input);

    result.unwrap().1
}

fn problem1(pairs: &Input) -> usize {
    pairs
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (left, right))| {
            acc + if left <= right { idx + 1 } else { 0 }
        })
}

fn problem2(pairs: &Input) -> usize {
    let mut signals: Vec<&Packet> = pairs.iter().flat_map(|(p1, p2)| vec![p1, p2]).collect();

    let divider_2 = &parse_packet("[[2]]").unwrap().1;
    let divider_6 = &parse_packet("[[6]]").unwrap().1;

    signals.push(divider_2);
    signals.push(divider_6);

    signals.sort();

    signals.iter().enumerate().fold(1, |acc, (idx, x)| {
        acc * if *x == divider_2 || *x == divider_6 {
            idx + 1
        } else {
            1
        }
    })
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 13)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 140)
    }
}
