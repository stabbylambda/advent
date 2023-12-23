use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    fmt::Binary,
};

use common::map::{Coord, Direction, Map};
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list0},
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

type Input = Map<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list0(
            newline,
            many1(alt((
                map(char('#'), |_| Tile::Forest),
                map(char('.'), |_| Tile::Path),
                map(char('^'), |_| Tile::SlopeNorth),
                map(char('v'), |_| Tile::SlopeSouth),
                map(char('<'), |_| Tile::SlopeWest),
                map(char('>'), |_| Tile::SlopeEast),
            ))),
        ),
        Map::new,
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Forest,
    Path,
    SlopeNorth,
    SlopeSouth,
    SlopeEast,
    SlopeWest,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    steps: usize,
    position: Coord,
    visited: BTreeSet<Coord>,
}

impl State {
    fn move_to(&self, position: Coord, cost: usize) -> Option<Self> {
        let mut visited = self.visited.clone();
        visited.insert(position).then_some(Self {
            steps: self.steps + cost,
            position,
            visited,
        })
    }
}

fn find_longest_path(input: &Input) -> usize {
    let start = (
        input.points[0]
            .iter()
            .position(|x| x == &Tile::Path)
            .unwrap(),
        0,
    );

    let start = State {
        steps: 0,
        position: start,
        visited: BTreeSet::new(),
    };

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(start);

    let mut longest = 0;
    while let Some(state) = queue.pop() {
        if state.position.1 == input.height - 1 {
            // we reached the end
            longest = longest.max(state.steps);
            continue;
        }

        let current = input.get(state.position);
        let neighbors = current.neighbors();
        let forced_neighbor = match current.data {
            // we can only move in the direction of slopes
            Tile::SlopeNorth => state.move_to(neighbors.north.unwrap().coords, 1),
            Tile::SlopeSouth => state.move_to(neighbors.south.unwrap().coords, 1),
            Tile::SlopeEast => state.move_to(neighbors.east.unwrap().coords, 1),
            Tile::SlopeWest => state.move_to(neighbors.west.unwrap().coords, 1),
            _ => None,
        };

        if let Some(new_state) = forced_neighbor {
            queue.push(new_state);
            continue;
        } else {
            for d in [
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::South,
            ] {
                if let Some((d, n)) = neighbors.get(d).map(|n| (d, n)) {
                    match (d, n.data) {
                        (_, Tile::Forest)
                        | (Direction::North, Tile::SlopeSouth)
                        | (Direction::South, Tile::SlopeNorth)
                        | (Direction::West, Tile::SlopeEast)
                        | (Direction::East, Tile::SlopeWest) => continue,
                        _ => {
                            let neighbor = state.move_to(n.coords, 1);
                            if let Some(new_state) = neighbor {
                                queue.push(new_state);
                                continue;
                            }
                        }
                    }
                };
            }
        }
    }
    longest
}

fn problem1(input: &Input) -> usize {
    find_longest_path(input)
}

fn get_intersections(input: &Input, start: Coord, end: Coord) -> BTreeSet<(usize, usize)> {
    let mut intersections: BTreeSet<Coord> = input
        .into_iter()
        .filter(|x| x.data != &Tile::Forest)
        .filter_map(|x| {
            // we only care about non-forest tiles
            let valid_neighbors = x
                .neighbors()
                .into_iter()
                .filter(|x| x.data != &Tile::Forest)
                .count();

            (valid_neighbors > 2).then_some(x.coords)
        })
        .collect();

    // we also care about the start and end
    intersections.insert(start);
    intersections.insert(end);

    intersections
}

fn problem2(input: &Input) -> usize {
    let start = (
        input
            .points
            .first()
            .unwrap()
            .iter()
            .position(|x| x == &Tile::Path)
            .unwrap(),
        0,
    );

    let end = (
        input
            .points
            .last()
            .unwrap()
            .iter()
            .position(|x| x == &Tile::Path)
            .unwrap(),
        input.height - 1,
    );

    let intersections = get_intersections(input, start, end);
    // do some edge compression because the grid has a ton of hallways
    let edges: BTreeMap<Coord, Vec<(Coord, usize)>> = intersections
        .iter()
        .map(|&i| (i, get_edges(input, i, &intersections)))
        .collect();

    let start = State {
        steps: 0,
        position: start,
        visited: BTreeSet::new(),
    };

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(start);

    let mut longest = 0;
    while let Some(state) = queue.pop() {
        if state.position == end {
            longest = longest.max(state.steps);
            continue;
        }

        for &(next, cost) in edges.get(&state.position).unwrap() {
            if let Some(new_state) = state.move_to(next, cost) {
                queue.push(new_state);
            }
        }
    }

    longest
}

fn get_edges(
    input: &Input,
    start: Coord,
    intersections: &BTreeSet<(usize, usize)>,
) -> Vec<(Coord, usize)> {
    let mut results: Vec<(Coord, usize)> = vec![];
    let mut visited: BTreeSet<Coord> = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((current, cost)) = queue.pop_front() {
        if intersections.contains(&current) && current != start {
            results.push((current, cost));
            continue;
        }

        for n in input
            .neighbors(current)
            .into_iter()
            .filter(|x| x.data != &Tile::Forest)
        {
            if visited.insert(n.coords) {
                queue.push_back((n.coords, cost + 1));
            }
        }
    }

    results
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 94)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 154)
    }
}
