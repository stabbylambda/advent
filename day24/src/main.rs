use std::collections::HashMap;

use itertools::Itertools;

use common::{
    dijkstra::{shortest_path, Edge},
    get_raw_input,
    map::Map,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Map<Tile>;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Space,
    Goal(u32),
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(alt((
                map(tag("#"), |_| Tile::Wall),
                map(tag("."), |_| Tile::Space),
                map(u32, Tile::Goal),
            ))),
        ),
        |input| Map::new(input.to_vec()),
    )(input);

    result.unwrap().1
}

fn get_edges(maze: &Map<Tile>) -> Vec<Vec<Edge>> {
    maze.into_iter()
        .map(|square| {
            // large nodes are walls and have no edges
            if square.data == &Tile::Wall {
                return vec![];
            }

            square
                .neighbors()
                .iter()
                .filter_map(|n| {
                    if n.data == &Tile::Wall {
                        return None;
                    }

                    Some(Edge {
                        node: n.get_grid_index(),
                        cost: 1,
                    })
                })
                .collect()
        })
        .collect()
}

fn problem(m: &Input, back_to_start: bool) -> usize {
    let edges = get_edges(m);

    let points: Vec<(u32, usize)> = m
        .into_iter()
        .filter_map(|s| match s.data {
            Tile::Wall => None,
            Tile::Space => None,
            Tile::Goal(n) => Some((*n, s.get_grid_index())),
        })
        .collect();

    let pairs: HashMap<(u32, u32), usize> = points
        .iter()
        .tuple_combinations()
        .flat_map(|((a, ai), (b, bi))| {
            let length = shortest_path(&edges, *ai, *bi).unwrap();
            vec![((*a, *b), length), ((*b, *a), length)]
        })
        .collect();

    points
        .iter()
        .permutations(points.len())
        .filter(|p| p.first().unwrap().0 == 0)
        .map(|v| {
            let travel = v
                .iter()
                .tuple_windows()
                .fold(0, |acc, (a, b)| acc + pairs[&(a.0, b.0)]);

            if back_to_start {
                let last = v.last().unwrap().0;
                travel + pairs[&(last, 0)]
            } else {
                travel
            }
        })
        .min()
        .unwrap()
}

fn problem1(input: &Input) -> usize {
    problem(input, false)
}
fn problem2(input: &Input) -> usize {
    problem(input, true)
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 14)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 20)
    }
}
