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

    let score = problem2(&input, 26501365);
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

fn infinite_neighbors((x, y): (isize, isize)) -> Vec<(isize, isize)> {
    [(0, -1), (0, 1), (-1, 0), (1, 0)]
        .into_iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct InfiniteState {
    steps: u32,
    position: (isize, isize),
}

impl PartialOrd for InfiniteState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InfiniteState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.position.cmp(&other.position))
    }
}

fn find_cycles(input: &Input, step_goal: usize) -> Vec<usize> {
    let starting_position = input
        .into_iter()
        .find_map(|x| (x.data == &Tile::StartingPosition).then_some(x.coords))
        .map(|(x, y)| (x as isize, y as isize))
        .unwrap();

    let mut queue: VecDeque<InfiniteState> = VecDeque::new();
    queue.push_back(InfiniteState {
        steps: 0,
        position: starting_position,
    });

    let mut ending: BTreeSet<(isize, isize)> = BTreeSet::new();
    let mut seen: BTreeSet<InfiniteState> = BTreeSet::new();

    let mut last = 1usize;
    let mut cycles = vec![];
    while let Some(state) = queue.pop_front() {
        ending.insert(state.position);

        // when we move up into a higher step size, we need to check if we're at the right point for a quadratic fit
        if last != state.steps as usize {
            if last % input.width == step_goal % input.width {
                cycles.push(ending.len());

                // once we have three terms, we're good
                if cycles.len() == 3 {
                    return cycles;
                }
            }
            ending.clear();

            last = state.steps as usize;
        }

        for neighbor @ (nx, ny) in infinite_neighbors(state.position) {
            let x = (nx.rem_euclid(input.width as isize)) as usize;
            let y = (ny.rem_euclid(input.height as isize)) as usize;

            if let Tile::Rock = input.get((x, y)).data {
                continue;
            }

            let state = InfiniteState {
                steps: state.steps + 1,
                position: neighbor,
            };

            // only explore the space if we haven't been here before with the exact same steps left
            if seen.insert(state) {
                queue.push_back(state);
            }
        }
    }

    unreachable!()
}

fn problem2(input: &Input, step_goal: usize) -> usize {
    let terms = find_cycles(input, step_goal);

    let x = step_goal / input.width;

    let b0 = terms[0];
    let b1 = terms[1] - terms[0];
    let b2 = terms[2] - terms[1];

    b0 + b1 * x + (x * (x - 1) / 2) * (b2 - b1)
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
        assert_eq!(problem2(&input, 6), 16);
        assert_eq!(problem2(&input, 10), 50);
        assert_eq!(problem2(&input, 50), 1594);
        assert_eq!(problem2(&input, 100), 6536);
    }
}
