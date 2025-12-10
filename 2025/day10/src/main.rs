use core::panic;
use std::collections::BinaryHeap;

use common::{answer, read_input};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::tag,
    character::complete::{newline, u64, usize},
    combinator::{complete, map},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use z3::{ast::Int, Optimize, SatResult};

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
    buttons_encoded: Vec<u64>,
    button_groups: Vec<Vec<usize>>,
    joltage: Vec<u64>,
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
    fn configure_joltage(&self) -> u64 {
        let solution = Optimize::new();

        let button_presses = self
            .button_groups
            .iter()
            .enumerate()
            .map(|(idx, _x)| Int::fresh_const(&format!("button_{idx}")))
            .collect_vec();

        let joltage_constraints = self
            .joltage
            .iter()
            .enumerate()
            .map(|(target_idx, &target)| {
                let terms = self
                    .button_groups
                    .iter()
                    .enumerate()
                    .filter_map(|(button_idx, button)| {
                        button
                            .contains(&target_idx)
                            .then_some(button_presses[button_idx].clone())
                    })
                    .collect_vec();

                let sum = Int::add(&terms);
                let target = Int::from_u64(target);

                sum.eq(target)
            })
            .collect_vec();

        // now add the constraints
        for x in &button_presses {
            solution.assert(&x.ge(0));
        }

        for x in joltage_constraints {
            solution.assert(&x);
        }

        let total = Int::fresh_const("total");
        solution.assert(&total.eq(Int::add(&button_presses)));
        solution.minimize(&total);

        assert_eq!(solution.check(&[]), SatResult::Sat);

        solution
            .get_model()
            .and_then(|m| m.eval(&total, true))
            .and_then(|x| x.as_u64())
            .unwrap()
    }

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

            for x in &self.buttons_encoded {
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

    let buttons = map(
        separated_list1(
            tag(" "),
            delimited(
                tag("("),
                map(separated_list1(tag(","), usize), |bits| {
                    let encoded = bits.iter().fold(0u64, |acc, &b| acc | (1 << b));
                    (encoded, bits)
                }),
                tag(")"),
            ),
        ),
        |x| {
            (
                x.iter().cloned().map(|x| x.0).collect_vec(),
                x.iter().cloned().map(|x| x.1).collect_vec(),
            )
        },
    );
    let joltage = delimited(tag("{"), separated_list1(tag(","), u64), tag("}"));

    let result: IResult<&str, Input> = complete(separated_list1(
        newline,
        map(
            (
                terminated(lights, tag(" ")),
                buttons,
                preceded(tag(" "), joltage),
            ),
            |(lights, (buttons_encoded, button_groups), joltage)| MachineSpec {
                lights,
                buttons_encoded,
                button_groups,
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

fn problem2(x: &Input) -> u64 {
    x.iter().map(|x| x.configure_joltage()).sum()
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
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 33)
    }
}
