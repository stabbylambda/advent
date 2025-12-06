use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Card>;
type CardId = u32;

#[derive(Debug)]
struct Card {
    id: CardId,
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl Card {
    /* Get the count of all matching cards */
    fn matching(&self) -> u32 {
        let winning: HashSet<u32> = HashSet::from_iter(self.winning.iter().cloned());
        let have: HashSet<u32> = HashSet::from_iter(self.have.iter().cloned());
        winning.intersection(&have).count() as u32
    }

    /* Score for part 1 */
    fn score(&self) -> u32 {
        let count = self.matching();

        if count == 0 {
            0
        } else {
            2u32.pow(count - 1)
        }
    }
}

fn parse(input: &str) -> Input {
    let card = map(
        (
            terminated(
                delimited(terminated(tag("Card"), space1), u32, tag(":")),
                space1,
            ),
            terminated(
                separated_list1(space1, u32),
                preceded(space1, terminated(tag("|"), space1)),
            ),
            separated_list1(space1, u32),
        ),
        |(id, winning, have)| Card { id, winning, have },
    );

    let result: IResult<&str, Input> = separated_list1(newline, card).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    input.iter().map(|x| x.score()).sum()
}

fn problem2(input: &Input) -> u32 {
    // seed the map with 1 of each card we have
    let mut card_count: HashMap<CardId, u32> = input.iter().map(|x| (x.id, 1)).collect();

    for card in input {
        // how many copies of this card do we have already?
        let copies_of_current = *card_count.get(&card.id).unwrap();

        // how many new cards do we get?
        for subsequent_copy in (card.id + 1)..=(card.id + card.matching()) {
            // update the existing subsequent card with 1 new copy for each copy of the current card
            if let Some(e) = card_count.get_mut(&subsequent_copy) {
                *e += copies_of_current;
            }
        }
    }

    card_count.values().sum()
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
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 30)
    }
}
