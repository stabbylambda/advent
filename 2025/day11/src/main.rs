use std::collections::{HashMap, VecDeque};

use common::{answer, read_input, to_number};
use nom::{
    bytes::complete::tag,
    character::{
        complete::{i32, newline},
        streaming::alpha1,
    },
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(input: &str) -> Input {
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

fn problem1(x: &Input) -> u32 {
    let state = "you";
    let mut queue = VecDeque::new();
    queue.push_back(state);
    let mut count = 0;

    while let Some(state) = queue.pop_front() {
        if state == "out" {
            count += 1;
            continue;
        }

        for next in &x[state] {
            queue.push_back(next);
        }
    }

    count
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
        assert_eq!(result, 5);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
