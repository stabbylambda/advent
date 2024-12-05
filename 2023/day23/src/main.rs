use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};

use common::{
    grid::{CardinalDirection, Coord, Grid},
    nom::parse_grid,
};
use nom::{branch::alt, character::complete::char, combinator::map, IResult};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Grid<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('#'), |_| Tile::Forest),
        map(char('.'), |_| Tile::Path),
        map(char('^'), |_| Tile::SlopeNorth),
        map(char('v'), |_| Tile::SlopeSouth),
        map(char('<'), |_| Tile::SlopeWest),
        map(char('>'), |_| Tile::SlopeEast),
    )))(input);

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

const DIRECTIONS: [CardinalDirection; 4] = [
    CardinalDirection::North,
    CardinalDirection::West,
    CardinalDirection::East,
    CardinalDirection::South,
];

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

fn get_longest_path<F>(input: &Input, valid: F) -> usize
where
    F: Copy + Fn(Tile, CardinalDirection, Tile) -> bool,
{
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
        .map(|&i| (i, get_edges(input, i, &intersections, valid)))
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
    valid: impl Fn(Tile, CardinalDirection, Tile) -> bool,
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

        let current = input.get(current);
        let neighbors = current.neighbors();

        let valid_neighbors = DIRECTIONS
            .iter()
            .filter_map(|d| neighbors.get(*d).map(|n| (d, n)));

        for (d, n) in valid_neighbors {
            if valid(*current.data, *d, *n.data) && visited.insert(n.coords) {
                queue.push_back((n.coords, cost + 1));
            }
        }
    }

    results
}

fn get_intersections(input: &Input, start: Coord, end: Coord) -> BTreeSet<(usize, usize)> {
    let mut intersections: BTreeSet<Coord> = input
        .iter()
        .filter(|x| x.data != &Tile::Forest)
        .filter_map(|x| {
            // we only care about non-forest tiles
            let valid_neighbors = x
                .neighbors()
                .iter()
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

fn problem1(input: &Input) -> usize {
    let valid = |current, dir, next| match (current, dir, next) {
        // can't go into the forest
        (_, _, Tile::Forest) => false,

        // can only go down the corresponding slope
        (Tile::SlopeNorth, d, _) => d == CardinalDirection::North,
        (Tile::SlopeSouth, d, _) => d == CardinalDirection::South,
        (Tile::SlopeEast, d, _) => d == CardinalDirection::East,

        // can't go up the opposing slope
        (_, CardinalDirection::North, Tile::SlopeSouth) => false,
        (_, CardinalDirection::South, Tile::SlopeNorth) => false,
        (_, CardinalDirection::East, Tile::SlopeWest) => false,
        (_, CardinalDirection::West, Tile::SlopeEast) => false,

        _ => true,
    };

    get_longest_path(input, valid)
}

fn problem2(input: &Input) -> usize {
    let valid = |current, dir, next| match (current, dir, next) {
        // can't go into the forest
        (_, _, Tile::Forest) => false,

        _ => true,
    };

    get_longest_path(input, valid)
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
