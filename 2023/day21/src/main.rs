use std::collections::{BTreeSet, VecDeque};

use common::map::{Coord, Map};
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input, 64);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Map<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(alt((
                map(char('#'), |_| Tile::Rock),
                map(char('.'), |_| Tile::Garden),
                map(char('S'), |_| Tile::StartingPosition),
            ))),
        ),
        Map::new,
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Rock,
    Garden,
    StartingPosition,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    steps_left: u32,
    position: Coord,
}

fn problem1(input: &Input, step_goal: u32) -> usize {
    let starting_position = input
        .into_iter()
        .find_map(|x| (x.data == &Tile::StartingPosition).then_some(x.coords))
        .unwrap();

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(State {
        steps_left: step_goal,
        position: starting_position,
    });

    let mut ending: BTreeSet<Coord> = BTreeSet::new();
    let mut seen: BTreeSet<State> = BTreeSet::new();

    while let Some(state) = queue.pop_front() {
        // if we're done on this path, just quit
        if state.steps_left == 0 {
            ending.insert(state.position);
            continue;
        }

        for n in input
            .neighbors(state.position)
            .into_iter()
            .filter(|x| x.data != &Tile::Rock)
        {
            let state = State {
                steps_left: state.steps_left - 1,
                position: n.coords,
            };

            // only explore the space if we haven't been here before with the exact same steps left
            if seen.insert(state) {
                queue.push_back(state);
            }
        }
    }

    ending.len()
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
        let result = problem1(&input, 6);
        assert_eq!(result, 16)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
