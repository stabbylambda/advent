use std::collections::{BTreeSet, VecDeque};

use common::nom::usize;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Hand = VecDeque<usize>;
type Input = (Hand, Hand);

fn parse(input: &str) -> Input {
    let hand = |s| {
        map(
            separated_pair(
                delimited(tag("Player "), u32, tag(":")),
                newline,
                map(separated_list1(newline, usize), VecDeque::from_iter),
            ),
            |(_id, hand)| hand,
        ).parse(s)
    };
    let result: IResult<&str, Input> = separated_pair(hand, tag("\n\n"), hand).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let (mut p1, mut p2) = input.clone();
    let winner = loop {
        if let Some((c1, c2)) = p1.pop_front().zip(p2.pop_front()) {
            if c1 > c2 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }

        if p1.is_empty() {
            break p2;
        }

        if p2.is_empty() {
            break p1;
        }
    };

    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, x)| x * (idx + 1))
        .sum()
}

enum GameResult {
    Player(VecDeque<usize>),
    Crab(VecDeque<usize>),
}

fn game((mut p1, mut p2): Input) -> GameResult {
    let mut rounds: BTreeSet<(VecDeque<usize>, VecDeque<usize>)> = BTreeSet::new();

    loop {
        // Did we already have a round exactly like this?
        if !rounds.insert((p1.clone(), p2.clone())) {
            break GameResult::Player(p1.clone());
        }

        //  the players begin the round by each drawing the top card of their deck as normal
        if let Some((c1, c2)) = p1.pop_front().zip(p2.pop_front()) {
            let recurse = (c1 <= p1.len()) && (c2 <= p2.len());

            let result = if recurse {
                // take the right number of cards
                let sub_p1 = p1.iter().take(c1).copied().collect();
                let sub_p2 = p2.iter().take(c2).copied().collect();

                // play a sub game
                game((sub_p1, sub_p2))
            } else if c1 > c2 {
                GameResult::Player(p1.clone())
            } else {
                GameResult::Crab(p2.clone())
            };

            match result {
                GameResult::Player(_) => {
                    p1.push_back(c1);
                    p1.push_back(c2);
                }
                GameResult::Crab(_) => {
                    p2.push_back(c2);
                    p2.push_back(c1);
                }
            }
        }

        if p1.is_empty() {
            break GameResult::Crab(p2.clone());
        }

        if p2.is_empty() {
            break GameResult::Player(p1.clone());
        }
    }
}

fn problem2(input: &Input) -> usize {
    match game(input.clone()) {
        GameResult::Player(x) | GameResult::Crab(x) => x
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, x)| x * (idx + 1))
            .sum(),
    }
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 306)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 291)
    }
}
