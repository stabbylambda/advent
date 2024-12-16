use std::{
    collections::{BTreeSet, BinaryHeap},
    time::Instant,
};

use common::{
    grid::{CardinalDirection, Coord, Grid},
    nom::parse_grid,
};
use nom::{branch::alt, character::complete::char, combinator::map, IResult};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let i = Instant::now();
    let score = problem1(&input);
    let d = i.elapsed();
    println!("problem 1 score: {score} in {d:?}");

    let i = Instant::now();
    let score = problem2(&input);
    let d = i.elapsed();
    println!("problem 2 score: {score} in {d:?}");
}

type Input = Grid<Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Space,
    Wall,
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('S'), |_| Tile::Start),
        map(char('E'), |_| Tile::End),
        map(char('.'), |_| Tile::Space),
        map(char('#'), |_| Tile::Wall),
    )))(input);

    result.unwrap().1
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    position: Coord,
    facing: CardinalDirection,
    tiles: Vec<Coord>,
}
impl State {
    fn new(position: Coord) -> Self {
        Self {
            cost: 0,
            position,
            facing: CardinalDirection::East,
            tiles: vec![position],
        }
    }
    fn moves(&self, grid: &Grid<Tile>) -> Vec<State> {
        let mut valid = vec![];
        valid.push(Self {
            facing: self.facing.turn_left(),
            cost: self.cost + 1000,
            tiles: self.tiles.clone(),
            ..*self
        });

        valid.push(Self {
            facing: self.facing.turn_right(),
            cost: self.cost + 1000,
            tiles: self.tiles.clone(),
            ..*self
        });

        if let Some(forward) = grid
            .get_neighbor(self.position, self.facing)
            .filter(|x| matches!(x.data, Tile::Space | Tile::End | Tile::Start))
            .filter(|x| !self.tiles.contains(&x.coords))
        {
            let mut new_tiles = self.tiles.clone();
            new_tiles.push(forward.coords);
            valid.push(Self {
                position: forward.coords,
                cost: self.cost + 1,
                tiles: new_tiles,
                ..*self
            });
        }

        valid
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn problem1(input: &Input) -> usize {
    let position = input
        .iter()
        .find_map(|x| (x.data == &Tile::Start).then_some(x.coords))
        .unwrap();

    let start = State::new(position);

    let mut seen: BTreeSet<(Coord, CardinalDirection)> = BTreeSet::new();

    let mut queue = BinaryHeap::new();
    queue.push(start);

    let mut best: Option<State> = None;
    while let Some(current) = queue.pop() {
        let square = input.get(current.position);

        // have we already been here?
        if !seen.insert((current.position, current.facing)) {
            continue;
        }

        // don't continue if we're already over a previous best
        if let Some(previous_best) = &best {
            if current.cost > previous_best.cost {
                continue;
            }
        }

        // did we hit an end?
        if square.data == &Tile::End {
            if let Some(previous_best) = &best {
                // we already have a previous best that's better than this
                if previous_best.cost != current.cost {
                    continue;
                }
            } else {
                best = Some(current.clone());
            }
        }

        // we're not there yet, keep looking
        for valid_move in &current.moves(input) {
            queue.push(valid_move.clone());
        }
    }

    best.unwrap().cost
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first_small() {
        let input = include_str!("../test_small.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 7036)
    }

    #[test]
    fn first_large() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 11048)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
