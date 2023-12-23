use std::collections::{BTreeSet, BinaryHeap};

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
    fn move_to(&self, position: Coord) -> Option<Self> {
        let mut visited = self.visited.clone();
        visited.insert(position).then_some(Self {
            steps: self.steps + 1,
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
            Tile::SlopeNorth => state.move_to(neighbors.north.unwrap().coords),
            Tile::SlopeSouth => state.move_to(neighbors.south.unwrap().coords),
            Tile::SlopeEast => state.move_to(neighbors.east.unwrap().coords),
            Tile::SlopeWest => state.move_to(neighbors.west.unwrap().coords),
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
                            let neighbor = state.move_to(n.coords);
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
        assert_eq!(result, 94)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
