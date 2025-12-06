use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of, u32},
    combinator::map,
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();

    let input1 = parse(input, false);
    let score = problem1(&input1);
    println!("problem 1 score: {score}");

    let input2 = parse(input, true);
    let score = problem2(&input2);
    println!("problem 2 score: {score}");
}

type Input = Hands;

fn parse(input: &str, jokers: bool) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            map(
                separated_pair(
                    count(map(one_of("AKQJT98765432"), |c| Card::new(c, jokers)), 5),
                    tag(" "),
                    u32,
                ),
                |(cards, bid)| (Hand { cards }, bid),
            ),
        ),
        |hands| Hands { hands },
    ).parse(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
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
    fn new(c: char, jokers: bool) -> Self {
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
            'J' if jokers => Self::Joker,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!(),
        }
    }
}

struct Hands {
    hands: Vec<(Hand, u32)>,
}

impl Hands {
    fn score(&self) -> u32 {
        // sort all the hands using the ordering we've built for Hand, multiply by corresponding bid
        self.hands
            .iter()
            .sorted_by_key(|x| x.0.clone())
            .map(|x| x.1)
            .enumerate()
            .map(|(rank, bid)| ((rank + 1) as u32) * bid)
            .sum()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let grouped = self.cards.iter().counts();
        let counts = grouped.values().sorted().rev().copied().collect_vec();
        // in problem 1, joker will be zero, because J == Jacks
        let joker_count = *grouped.get(&Card::Joker).unwrap_or(&0);

        // Gotta match all the different ways that jokers can make different combinations
        if counts == vec![5]
            || (counts == vec![4, 1] && joker_count > 0)
            || (counts == vec![3, 2] && joker_count > 0)
        {
            HandType::FiveOfAKind
        } else if counts == vec![4, 1]
            || (counts == vec![3, 1, 1] && joker_count > 0)
            || (counts == vec![2, 2, 1] && joker_count == 2)
        {
            HandType::FourOfAKind
        } else if counts == vec![3, 2] || (counts == vec![2, 2, 1] && joker_count == 1) {
            HandType::FullHouse
        } else if counts == vec![3, 1, 1] || (counts == vec![2, 1, 1, 1] && joker_count > 0) {
            HandType::ThreeOfAKind
        } else if counts == vec![2, 2, 1] {
            HandType::TwoPair
        } else if counts == vec![2, 1, 1, 1] || (counts == vec![1, 1, 1, 1, 1] && joker_count > 0) {
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
        // type first, then natural ordering of the Vec of Card enums
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
    input.score()
}

fn problem2(input: &Input) -> u32 {
    input.score()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input, false);
        let result = problem1(&input);
        assert_eq!(result, 6440)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input, true);
        let result = problem2(&input);
        assert_eq!(result, 5905)
    }
}
