use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
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

type Input = Vec<Card>;

#[derive(Debug)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let winning: HashSet<u32> = HashSet::from_iter(self.winning.iter().cloned());
        let have: HashSet<u32> = HashSet::from_iter(self.have.iter().cloned());
        let count = winning.intersection(&have).count() as u32;

        if count == 0 {
            0
        } else {
            2u32.pow(count - 1)
        }
    }
}

fn parse(input: &str) -> Input {
    let card = map(
        tuple((
            terminated(
                delimited(terminated(tag("Card"), space1), u32, tag(":")),
                space1,
            ),
            terminated(
                separated_list1(space1, u32),
                preceded(space1, terminated(tag("|"), space1)),
            ),
            separated_list1(space1, u32),
        )),
        |(id, winning, have)| Card { id, winning, have },
    );

    let result: IResult<&str, Input> = separated_list1(newline, card)(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    input.iter().map(|x| x.score()).sum()
}

fn problem2(_input: &Input) -> u32 {
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
        assert_eq!(result, 13)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
