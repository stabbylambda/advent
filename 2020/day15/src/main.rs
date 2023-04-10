use std::collections::BTreeMap;

use nom::{bytes::complete::tag, character::complete::u32, multi::separated_list1, IResult};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(tag(","), u32)(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    solve(input, 2020)
}

fn solve(input: &Input, limit: u32) -> u32 {
    let mut map: BTreeMap<u32, (u32, u32)> = input
        .iter()
        .enumerate()
        .map(|(turn, num)| {
            let turn = (turn + 1) as u32;
            (*num, (turn, turn))
        })
        .collect();

    let mut last = *input.last().unwrap();
    let mut turn = (input.len() + 1) as u32;

    while turn <= limit {
        let last_pair = *map.get(&last).unwrap_or(&(last, last));
        let next = last_pair.0.abs_diff(last_pair.1);

        map.entry(next)
            .and_modify(|(age1, age2)| {
                *age2 = *age1;
                *age1 = turn;
            })
            .or_insert((turn, turn));

        last = next;
        turn += 1;
    }

    last
}

fn problem2(input: &Input) -> u32 {
    solve(input, 30_000_000)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 436)
    }

    #[test]
    #[ignore]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 175594)
    }
}
