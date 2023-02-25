use std::{collections::BinaryHeap, fmt::Display};

use nom::{
    character::complete::{char, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Piece>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(separated_pair(u32, char('/'), u32), |(x, y)| Piece {
            left: x,
            right: y,
        }),
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Eq)]
struct Piece {
    left: u32,
    right: u32,
}
impl Piece {
    fn strength(&self) -> u32 {
        self.left + self.right
    }

    fn swap(&self) -> Piece {
        Piece {
            left: self.right,
            right: self.left,
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        (self.left == other.left && self.right == other.right)
            || (self.left == other.right && self.right == other.left)
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.left, self.right)
    }
}
#[derive(Clone, PartialEq, Eq)]
struct State {
    pieces: Vec<Piece>,
    bridge: Vec<Piece>,
}

impl State {
    fn strength(&self) -> u32 {
        self.bridge.iter().map(|x| x.strength()).sum()
    }

    fn last_pins(&self) -> u32 {
        self.bridge.last().map(|x| x.right).unwrap_or(0)
    }

    fn next_pieces(&self) -> Vec<Piece> {
        let last = self.last_pins();
        self.pieces
            .clone()
            .into_iter()
            .filter_map(|x| {
                if x.left == last {
                    Some(x)
                } else if x.right == last {
                    Some(x.swap())
                } else {
                    None
                }
            })
            .collect()
    }

    fn use_piece(&self, piece: Piece) -> State {
        let mut bridge = self.bridge.clone();
        bridge.push(piece);

        let pieces = self
            .pieces
            .clone()
            .into_iter()
            .filter(|x| *x != piece)
            .collect();

        State { bridge, pieces }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.strength().cmp(&other.strength())
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Length(State);

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Length {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .bridge
            .len()
            .cmp(&other.0.bridge.len())
            .then(self.0.strength().cmp(&other.0.strength()))
    }
}

fn problem1(input: &Input) -> u32 {
    /* TODO: In this naive version, there's 838,744 states considered. There's definitely a way to
       cut down on the states here by:
        1) Remove dupes (e.g. 2/2). If the max bridge has a number that would have been a dupe,
           just add the number back to the strength
        2) Prune states based on remaining matches. If we've got a number that has no pair, then
           we can delete the state
        3) Maybe keep track of best strength so far and if we're less than that and can't possibly
           make it up, then delete the state?
    */

    let mut states = 0;
    let mut bridges: Vec<State> = vec![];
    let mut priority_queue: BinaryHeap<State> = BinaryHeap::new();
    priority_queue.push(State {
        pieces: input.clone(),
        bridge: vec![],
    });

    while let Some(state) = priority_queue.pop() {
        states += 1;
        bridges.push(state.clone());

        for x in state.next_pieces() {
            priority_queue.push(state.use_piece(x));
        }
    }

    // get the best bridge
    let best = bridges.iter().max().unwrap();

    println!("{states} states considered");
    best.strength()
}

fn problem2(input: &Input) -> u32 {
    let mut bridges: Vec<Length> = vec![];
    let mut priority_queue: BinaryHeap<Length> = BinaryHeap::new();
    priority_queue.push(Length(State {
        pieces: input.clone(),
        bridge: vec![],
    }));

    while let Some(Length(state)) = priority_queue.pop() {
        bridges.push(Length(state.clone()));

        for x in state.next_pieces() {
            priority_queue.push(Length(state.use_piece(x)));
        }
    }

    // get the best bridge
    bridges.iter().max().unwrap().0.strength()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 31)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 19)
    }
}
