use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    fmt::Display,
};

use common::{
    dijkstra::{shortest_path, Edge},
    map::{Map, MapSquare},
};
use nom::{
    character::complete::{anychar, newline},
    combinator::{map, map_opt},
    multi::{many1, separated_list1},
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

type Input = Map<Tile>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Space,
    Entrance,
    Key(char),
    Door(char),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => '#',
                Tile::Space => '.',
                Tile::Entrance => '@',
                Tile::Key(x) => *x,
                Tile::Door(x) => x.to_ascii_uppercase(),
            }
        )
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(map_opt(anychar, |x| match x {
                '@' => Some(Tile::Entrance),
                '#' => Some(Tile::Wall),
                '.' => Some(Tile::Space),
                x if x.is_lowercase() => Some(Tile::Key(x)),
                x if x.is_uppercase() => Some(Tile::Door(x.to_ascii_lowercase())),
                _ => None,
            })),
        ),
        Map::new,
    )(input);

    result.unwrap().1
}

fn get_edges(maze: &Map<Tile>, keys: &BTreeSet<char>) -> Vec<Vec<Edge>> {
    // could probably cut out a bunch of tiles if we used edge weights equal to the number of spaces between nodes instead of
    // just having everything be neighbors with an edge weight of 1
    let is_open = |t: &Tile| {
        match t {
            // walls have no edges
            Tile::Wall => false,
            // doors that we don't have the key for also have no edges
            Tile::Door(c) if !keys.contains(c) => false,
            _ => true,
        }
    };

    maze.into_iter()
        .map(|square| {
            if is_open(square.data) {
                square
                    .neighbors()
                    .into_iter()
                    .filter_map(|n| is_open(n.data).then(|| Edge::from_map_square(&n)))
                    .collect()
            } else {
                vec![]
            }
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    steps: usize,
    position: (usize, usize),
    keys: BTreeSet<char>,
}

impl State {
    fn get_key(&self, steps: usize, position: (usize, usize), key: char) -> State {
        let mut new_state = self.clone();
        new_state.steps += steps;
        new_state.position = position;
        new_state.keys.insert(key);

        new_state
    }

    fn to_cache_key(&self) -> ((usize, usize), BTreeSet<char>) {
        (self.position, self.keys.clone())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

fn problem1(input: &Input) -> usize {
    println!("{input}");
    let start = input
        .into_iter()
        .find(|x| x.data == &Tile::Entrance)
        .unwrap();
    let keys: Vec<MapSquare<Tile>> = input
        .into_iter()
        .filter(|x| matches!(x.data, Tile::Key(..)))
        .collect();

    // we're starting at the entrance with no keys
    let state = State {
        steps: 0,
        position: start.coords,
        keys: BTreeSet::new(),
    };

    let mut seen: BTreeMap<((usize, usize), BTreeSet<char>), usize> = BTreeMap::new();
    let mut adjacencies: BTreeMap<BTreeSet<char>, Vec<Vec<Edge>>> = BTreeMap::new();

    let mut queue = BinaryHeap::new();
    queue.push(state);

    while let Some(state) = queue.pop() {
        // we've got all the keys, so we're done!
        if state.keys.len() == keys.len() {
            return state.steps;
        }

        // have we already been here before?
        if let Some(previous_steps) = seen.insert(state.to_cache_key(), state.steps) {
            if state.steps >= previous_steps {
                continue;
            }
        }

        println!("{state:?}");

        // get the edges given the keys that we have right now (but cache the adjacency list because it won't change)
        let edges = adjacencies
            .entry(state.keys.clone())
            .or_insert_with(|| get_edges(input, &state.keys));

        let new_states: Vec<State> = keys
            .iter()
            .filter_map(|x| {
                // only keep keys we don't already have
                let Tile::Key(c) = x.data else { return None };
                if state.keys.contains(c) {
                    return None;
                }

                let position = x.get_grid_index();
                shortest_path(edges, input.get_grid_index(state.position), position)
                    .map(|cost| state.get_key(cost, x.coords, *c))
            })
            .collect();

        // go after each of the keys
        queue.extend(new_states);
    }

    0
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
        assert_eq!(result, 136)
    }

    #[test]
    #[ignore]
    fn input() {
        let input = include_str!("../input.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 5198)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
