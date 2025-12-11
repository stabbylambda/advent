use std::{collections::HashMap, hash::Hash};

use common::{answer, read_input};
use nom::{
    bytes::complete::tag,
    character::{complete::newline, streaming::alpha1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse<'a>(input: &'a str) -> Input<'a> {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1)),
        ),
        |x| HashMap::from_iter(x),
    )
    .parse(input);

    result.unwrap().1
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State<'a> {
    node: &'a str,
    dac: bool,
    fft: bool,
}

impl<'a> State<'a> {
    fn new(node: &'a str) -> State<'a> {
        State {
            node,
            dac: false,
            fft: false,
        }
    }

    fn move_to(&self, node: &'a str) -> State<'a> {
        State {
            node,
            dac: self.dac || node == "dac",
            fft: self.fft || node == "fft",
        }
    }
}

fn paths<'a, F>(
    state: State<'a>,
    to: &'a str,
    x: &'a Input,
    cache: &mut HashMap<State<'a>, u64>,
    f: F,
) -> u64
where
    F: Copy + Clone + Fn(&State) -> bool,
{
    if *state.node == *to {
        return f(&state).into();
    }

    let result = match cache.get(&state) {
        Some(x) => *x,
        None => {
            let result = x
                .get(state.node)
                .unwrap_or(&vec![])
                .iter()
                .map(|n| paths(state.move_to(n), to, x, cache, f))
                .sum();
            cache.insert(state, result);
            result
        }
    };

    result
}

fn problem1(x: &Input) -> u64 {
    paths(State::new("you"), "out", x, &mut HashMap::new(), |_| true)
}

fn problem2(x: &Input) -> u64 {
    paths(State::new("svr"), "out", x, &mut HashMap::new(), |s| {
        s.dac && s.fft
    })
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 5);
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 2)
    }
}
