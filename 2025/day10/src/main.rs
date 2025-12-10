use std::collections::BinaryHeap;

use common::{answer, read_input};
use nom::{
    branch::alt,
    bytes::tag,
    character::complete::{newline, usize},
    combinator::{complete, map},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<MachineSpec>;

#[derive(Debug)]
struct MachineSpec {
    lights: u64,
    buttons: Vec<u64>,
    joltage: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    presses: u64,
    lights: u64,
    last: u64,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.presses.cmp(&self.presses)
    }
}

impl MachineSpec {
    fn configure_lights(&self) -> u64 {
        let mut queue: BinaryHeap<_> = BinaryHeap::new();

        queue.push(State {
            presses: 0,
            lights: 0,
            last: 0,
        });
        while let Some(state) = queue.pop() {
            // did we activate the lights correctly?
            if state.lights == self.lights {
                return state.presses;
            }

            for x in &self.buttons {
                // don't just press the same button again
                if state.last == *x {
                    continue;
                }

                queue.push(State {
                    presses: state.presses + 1,
                    lights: state.lights ^ x,
                    last: *x,
                });
            }
        }

        unreachable!()
    }
}

fn parse(input: &str) -> Input {
    let lights = map(
        delimited(
            tag("["),
            many1(alt((map(tag("."), |_| false), map(tag("#"), |_| true)))),
            tag("]"),
        ),
        |b| {
            // transform it into a bit representation
            b.iter()
                .rev()
                .fold(0u64, |acc, &b| (acc << 1) | if b { 1 } else { 0 })
        },
    );

    let buttons = separated_list1(
        tag(" "),
        delimited(
            tag("("),
            map(separated_list1(tag(","), usize), |bits| {
                bits.iter().fold(0u64, |acc, &b| acc | (1 << b))
            }),
            tag(")"),
        ),
    );
    let joltage = delimited(tag("{"), separated_list1(tag(","), usize), tag("}"));

    let result: IResult<&str, Input> = complete(separated_list1(
        newline,
        map(
            (
                terminated(lights, tag(" ")),
                buttons,
                preceded(tag(" "), joltage),
            ),
            |(lights, buttons, joltage)| MachineSpec {
                lights,
                buttons,
                joltage,
            },
        ),
    ))
    .parse(input);

    result.unwrap().1
}

fn problem1(x: &Input) -> u64 {
    x.iter().map(|x| x.configure_lights()).sum()
}

fn problem2(x: &Input) -> u32 {
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
        assert_eq!(result, 7);
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
