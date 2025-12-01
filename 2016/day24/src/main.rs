use std::collections::HashMap;

use itertools::Itertools;

use common::{
    dijkstra::{shortest_path, Edge},
    grid::Grid,
    nom::parse_grid,
};
use nom::{branch::alt, bytes::complete::tag, character::complete::u32, combinator::map, IResult, Parser};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Grid<Tile>;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Space,
    Goal(u32),
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(tag("#"), |_| Tile::Wall),
        map(tag("."), |_| Tile::Space),
        map(u32, Tile::Goal),
    ))).parse(input);

    result.unwrap().1
}

fn get_edges(maze: &Grid<Tile>) -> Vec<Vec<Edge>> {
    maze.iter()
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

                    Some(Edge::from_map_square(n))
                })
                .collect()
        })
        .collect()
}

fn problem(m: &Input, back_to_start: bool) -> usize {
    let edges = get_edges(m);

    let points: Vec<(u32, usize)> = m
        .iter()
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

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 14)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 20)
    }
}
