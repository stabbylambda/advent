use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    fmt::Display,
    time::{Duration, Instant},
};

use common::{
    answer,
    dijkstra::{shortest_path, Edge},
    grid::{Grid, GridSquare},
    nom::parse_grid,
    read_input,
};
use nom::{character::complete::anychar, combinator::map_opt, IResult, Parser};

fn main() {
    let input = read_input!();
    let input = parse(input);
    answer!(problem(&input));

    let input = include_str!("../input2.txt");
    let input = parse(input);
    answer!(problem(&input));
}

type Input = Grid<Tile>;

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
    let result: IResult<&str, Input> = parse_grid(map_opt(anychar, |x| match x {
        '@' => Some(Tile::Entrance),
        '#' => Some(Tile::Wall),
        '.' => Some(Tile::Space),
        x if x.is_lowercase() => Some(Tile::Key(x)),
        x if x.is_uppercase() => Some(Tile::Door(x.to_ascii_lowercase())),
        _ => None,
    })).parse(input);

    result.unwrap().1
}

// build an adjacency list for the map
fn get_edges(maze: &Grid<Tile>) -> Vec<Vec<Edge>> {
    let is_open = |t: &Tile| {
        match t {
            // walls have no edges
            Tile::Wall => false,
            _ => true,
        }
    };
    maze.iter()
        .map(|square| {
            // walls have no edges
            if !is_open(square.data) {
                return vec![];
            }

            square
                .neighbors()
                .iter()
                .filter_map(|n| {
                    if !is_open(n.data) {
                        return None;
                    }

                    Some(Edge::from_map_square(n))
                })
                .collect()
        })
        .collect()
}

/// Remove any edges from the adjacency list that go through a door we don't have the key for
fn strip_edges(
    edges: &[Vec<Edge>],
    doors: &[(usize, char)],
    keys: &BTreeSet<char>,
) -> Vec<Vec<Edge>> {
    let mut edges = edges.to_vec();

    for (grid_index, c) in doors {
        // if we don't have the key
        if !keys.contains(c) {
            // remove any edges through that door
            edges[*grid_index].clear()
        }
    }

    edges
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    steps: usize,
    positions: Vec<Coord>,
    keys: BTreeSet<char>,
    key_count: usize,
}

impl State {
    fn get_key(&self, steps: usize, position_idx: usize, position: Coord, key: char) -> State {
        let mut new_state = self.clone();
        new_state.steps += steps;
        new_state.positions[position_idx] = position;
        new_state.keys.insert(key);
        new_state.key_count += 1;

        new_state
    }

    fn to_cache_key(&self) -> (Vec<Coord>, BTreeSet<char>) {
        (self.positions.clone(), self.keys.clone())
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
            .then(self.key_count.cmp(&other.key_count))
    }
}

type Coord = (usize, usize);
fn problem(input: &Input) -> usize {
    println!("{input}");
    let start: Vec<Coord> = input
        .iter()
        .filter_map(|x| (x.data == &Tile::Entrance).then_some(x.coords))
        .collect();
    let keys: Vec<GridSquare<Tile>> = input
        .iter()
        .filter(|x| matches!(x.data, Tile::Key(..)))
        .collect();
    let doors: Vec<(usize, char)> = input
        .iter()
        .filter_map(|x| match x.data {
            Tile::Door(c) => Some((x.get_grid_index(), *c)),
            _ => None,
        })
        .collect();

    // we're starting at the entrance with no keys
    let state = State {
        steps: 0,
        positions: start,
        keys: BTreeSet::new(),
        key_count: 0,
    };

    let mut seen: BTreeMap<(Vec<Coord>, BTreeSet<char>), usize> = BTreeMap::new();
    let mut adjacencies: BTreeMap<BTreeSet<char>, Vec<Vec<Edge>>> = BTreeMap::new();

    let mut queue = BinaryHeap::new();
    queue.push(state);

    let mut elapsed = Duration::ZERO;

    let edges = get_edges(input);

    let mut considered = 1;
    let start = Instant::now();
    while let Some(state) = queue.pop() {
        considered += 1;
        // we've got all the keys, so we're done!
        if state.keys.len() == keys.len() {
            let total_time = start.elapsed();
            println!("Considered {considered} states. Dijkstra took {elapsed:?}. Total time {total_time:?}");
            return state.steps;
        }

        // have we already been here before?
        if let Some(previous_steps) = seen.insert(state.to_cache_key(), state.steps) {
            if state.steps >= previous_steps {
                continue;
            }
        }

        // get the edges given the keys that we have right now (but cache the adjacency list because it won't change)
        let edges = adjacencies
            .entry(state.keys.clone())
            .or_insert_with(|| strip_edges(&edges, &doors, &state.keys));

        let new_states: Vec<State> = state
            .positions
            .iter()
            .cloned()
            .enumerate()
            .flat_map(|(position_idx, position)| {
                keys.iter()
                    .filter_map(|x| {
                        // only keep keys we don't already have
                        let Tile::Key(c) = x.data else { return None };
                        if state.keys.contains(c) {
                            return None;
                        }

                        let start = input.get_grid_index(position);
                        let goal = x.get_grid_index();

                        let now = Instant::now();
                        shortest_path(edges, start, goal).map(|cost| {
                            elapsed += now.elapsed();
                            state.get_key(cost, position_idx, x.coords, *c)
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // go after each of the keys
        queue.extend(new_states);
    }

    0
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    #[ignore]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem(&input);
        assert_eq!(result, 136)
    }

    #[test]
    #[ignore]
    fn input() {
        let input = common::read_input!();
        let input = parse(input);
        let result = problem(&input);
        assert_eq!(result, 5198)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem(&input);
        assert_eq!(result, 72)
    }
}
