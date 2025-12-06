use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    sequence::separated_pair,
    IResult, Parser,
};
fn main() {
    let lines = common::read_input!();
    let input = parse(lines);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}
type Instruction = ((char, char), char);
struct Input {
    polymer: Vec<char>,
    instructions: BTreeMap<(char, char), char>,
}

fn parse(lines: &str) -> Input {
    let lines: Vec<&str> = lines.lines().collect();
    let polymer: Vec<char> = lines[0].chars().collect();
    let instructions: BTreeMap<(char, char), char> = lines[2..]
        .iter()
        .map(|i| parse_instruction(i).unwrap().1)
        .collect();

    Input {
        polymer,
        instructions,
    }
}
fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    separated_pair((anychar, anychar), tag(" -> "), anychar).parse(s)
}

fn problem1(input: &Input) -> u64 {
    problem(input, 10)
}

fn problem2(input: &Input) -> u64 {
    problem(input, 40)
}

fn problem(input: &Input, times: u32) -> u64 {
    let Input {
        polymer,
        instructions,
    } = input;

    let mut map: BTreeMap<(char, char), u64> = BTreeMap::new();
    let mut letter_count: BTreeMap<char, u64> = BTreeMap::new();

    // seed the map the first time
    polymer.windows(2).for_each(|window| {
        map.entry((window[0], window[1]))
            .and_modify(|x| *x += 1)
            .or_insert(1);
    });

    for c in polymer {
        letter_count.entry(*c).and_modify(|x| *x += 1).or_insert(1);
    }

    for _n in 0..times {
        let mut result = BTreeMap::new();
        for (pair @ (c1, c2), score) in map {
            let new_char = instructions[&pair];
            result
                .entry((c1, new_char))
                .and_modify(|x| *x += score)
                .or_insert(score);

            result
                .entry((new_char, c2))
                .and_modify(|x| *x += score)
                .or_insert(score);

            letter_count
                .entry(new_char)
                .and_modify(|x| *x += score)
                .or_insert(score);
        }

        map = result;
    }

    let scores: Vec<u64> = letter_count.values().copied().collect();
    let max = scores.iter().max().unwrap();
    let min = scores.iter().min().unwrap();

    max - min
}
#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 1588)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 2188189693529)
    }
}
