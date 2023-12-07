use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of, u32},
    combinator::map,
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Hand>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            separated_pair(
                count(map(one_of("AKQJT98765432"), Card::new), 5),
                tag(" "),
                u32,
            ),
            |(cards, bid)| Hand { cards, bid },
        ),
    )(input);

    result.unwrap().1
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32) -> Self {
        Self { cards, bid }
    }

    fn get_type(&self) -> HandType {
        let grouped = self.cards.iter().counts();
        let counts = grouped.values().sorted().rev().copied().collect_vec();

        if counts == vec![5] {
            HandType::FiveOfAKind
        } else if counts == vec![4, 1] {
            HandType::FourOfAKind
        } else if counts == vec![3, 2] {
            HandType::FullHouse
        } else if counts == vec![3, 1, 1] {
            HandType::ThreeOfAKind
        } else if counts == vec![2, 2, 1] {
            HandType::TwoPair
        } else if counts == vec![2, 1, 1, 1] {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_type()
            .cmp(&other.get_type())
            .then(self.cards.cmp(&other.cards))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn problem1(input: &Input) -> u32 {
    input
        .iter()
        .sorted()
        .enumerate()
        .map(|(rank, hand)| ((rank + 1) as u32) * hand.bid)
        .sum()
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
        assert_eq!(result, 6440)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
