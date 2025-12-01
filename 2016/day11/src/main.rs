use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::map,
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let mut input = input;

    for element in ["elerium", "dilithium"] {
        let generator = Item::Generator(element.to_string());
        let chip = Item::Microchip(element.to_string());
        input.floors[0].items.extend([chip, generator]);
    }

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Elevator;

fn parse(input: &str) -> Input {
    let generator = map(
        delimited(alt((tag("and a "), tag("a "))), alpha1, tag(" generator")),
        |x: &str| Item::Generator(x.to_string()),
    );

    let microchip = map(
        delimited(
            alt((tag("and a "), tag("a "))),
            alpha1,
            tag("-compatible microchip"),
        ),
        |x: &str| Item::Microchip(x.to_string()),
    );

    let items = alt((
        separated_list1(alt((tag(", "), tag(" and "))), alt((generator, microchip))),
        map(tag("nothing relevant"), |_| vec![]),
    ));

    let floor_name = delimited(tag("The "), alpha1, tag(" floor contains "));

    let result: IResult<&str, Input> = map(
        separated_list1(
            tag(".\n"),
            map((floor_name, items), |(name, items)| Floor {
                name: name.to_string(),
                items,
            }),
        ),
        Elevator::new,
    ).parse(input);

    result.unwrap().1
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Item {
    Generator(String),
    Microchip(String),
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Floor {
    name: String,
    items: Vec<Item>,
}

impl Floor {
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn generator_count(&self) -> usize {
        self.items
            .iter()
            .filter(|x| matches!(x, Item::Generator(..)))
            .count()
    }

    fn microchip_count(&self) -> usize {
        self.items
            .iter()
            .filter(|x| matches!(x, Item::Microchip(..)))
            .count()
    }

    fn fried_items(&self) -> bool {
        let contains_generators = self.generator_count() > 0;

        self.items.iter().any(|x| {
            // only consider microchips
            let Item::Microchip(name) = x else {
                return false;
            };

            // check if we have a matching generator
            if self.items.contains(&Item::Generator(name.to_string())) {
                return false;
            }

            // if there are generators, then this chip is fried
            contains_generators
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Elevator {
    floors: Vec<Floor>,
    current_floor: usize,
    moves: u32,
}

impl Elevator {
    fn new(floors: Vec<Floor>) -> Self {
        Elevator {
            floors,
            current_floor: 0,
            moves: 0,
        }
    }

    fn available_items(&self) -> Vec<(Vec<Item>, bool)> {
        self.floors[self.current_floor]
            .items
            .iter()
            .powerset()
            .filter(|x| x.len() == 1 || x.len() == 2)
            .map(|v| v.iter().map(|&x| x.clone()).collect_vec())
            .cartesian_product(vec![true, false])
            .collect_vec()
    }

    fn can_move(&self, up: bool) -> bool {
        // can't go up from 3 or down from 4
        let up_from_three = up && self.current_floor == 3;
        let down_from_zero = !up && self.current_floor == 0;

        !(up_from_three || down_from_zero)
    }

    fn move_items(&self, items: &[Item], up: bool) -> Option<Elevator> {
        if !self.can_move(up) {
            return None;
        }

        let mut new = self.clone();
        new.moves += 1;

        // update the current floor
        if up {
            new.current_floor += 1;
        } else {
            new.current_floor -= 1;
        }

        // move all the items
        for item in items {
            new.floors[self.current_floor].items.retain(|x| x != item);
            new.floors[new.current_floor].items.push(item.clone());
        }

        // don't return scenarios that result in fried items
        if new.fried_items() {
            return None;
        }

        Some(new)
    }

    fn ready_to_assemble(&self) -> bool {
        // only the fourth floor has all the items
        self.floors[0].is_empty()
            && self.floors[1].is_empty()
            && self.floors[2].is_empty()
            && !self.floors[3].is_empty()
    }

    fn fried_items(&self) -> bool {
        self.floors.iter().any(|f| f.fried_items())
    }

    fn get_fingerprint(&self) -> ElevatorFingerprint {
        ElevatorFingerprint {
            current_floor: self.current_floor,
            counts: self
                .floors
                .iter()
                .map(|v| (v.generator_count(), v.microchip_count()))
                .collect(),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct ElevatorFingerprint {
    current_floor: usize,
    counts: Vec<(usize, usize)>,
}

impl PartialOrd for Elevator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elevator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // flipped so that we can use a min-heap, then compare each of the floors to see which one has more items up high
        other
            .moves
            .cmp(&self.moves)
            .then_with(|| self.floors[3].items.len().cmp(&other.floors[3].items.len()))
            .then_with(|| self.floors[2].items.len().cmp(&other.floors[2].items.len()))
            .then_with(|| self.floors[1].items.len().cmp(&other.floors[1].items.len()))
    }
}

fn simulate(elevator: &Elevator) -> u32 {
    let mut best_so_far = u32::MAX;
    let mut compared = 0;

    let mut seen: HashSet<ElevatorFingerprint> = HashSet::new();
    let mut priority_queue: BinaryHeap<Elevator> = BinaryHeap::new();
    priority_queue.push(elevator.clone());

    while let Some(elevator) = priority_queue.pop() {
        compared += 1;

        if compared % 1000 == 0 {
            println!("Compared {compared}");
        }

        // if all the items are on the fourth floor, we've found a new potential minimum
        if elevator.ready_to_assemble() {
            best_so_far = best_so_far.min(elevator.moves);
            println!(
                "Found a solution with {} steps after {} comparisons.",
                best_so_far, compared
            );
            continue;
        }

        // if we've already gone too many moves, just bail
        if elevator.moves >= best_so_far {
            continue;
        }

        // get all the permutations of the items on the current floor and try taking them to the other floor
        for (items, up) in elevator.available_items() {
            // if moving the items results in a valid scenario
            let Some(next) = elevator.move_items(&items, up) else {
                continue;
            };

            // and we haven't seen this fingerprint before
            if seen.insert(next.get_fingerprint()) {
                // queue it up
                priority_queue.push(next);
            }
        }
    }

    best_so_far
}
fn problem1(input: &Input) -> u32 {
    simulate(input)
}

fn problem2(input: &Input) -> u32 {
    simulate(input)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 11)
    }
}
