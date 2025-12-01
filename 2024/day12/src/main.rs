use itertools::Itertools;
use std::{
    collections::{BTreeSet, VecDeque},
    time::Instant,
};

use common::{
    grid::{CardinalDirection, Coord, Grid},
    nom::parse_grid,
};
use nom::{character::complete::one_of, IResult, Parser};

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

type Input = Grid<char>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")).parse(input);

    result.unwrap().1
}

struct Region {
    name: char,
    area: usize,
    sides: Vec<Edge>,
}

struct Edge {
    direction: CardinalDirection,
    primary: usize,
    secondary: usize,
}

impl Edge {
    fn new(direction: CardinalDirection, (x, y): Coord) -> Self {
        let (primary, secondary) = match direction {
            CardinalDirection::North | CardinalDirection::South => (y, x),
            CardinalDirection::East | CardinalDirection::West => (x, y),
        };

        Self {
            direction,
            primary,
            secondary,
        }
    }
}

impl Region {
    fn new(name: char) -> Self {
        Self {
            name,
            area: 0,
            sides: vec![],
        }
    }
    fn fences(&self) -> usize {
        self.sides.len()
    }

    fn unique_fences(&self) -> usize {
        // group them by direction -> primary coordinate -> secondary coordinate
        let sides = self
            .sides
            .iter()
            .into_grouping_map_by(|e| (e.direction, e.primary))
            .fold(Vec::new(), |mut acc: Vec<usize>, _key, x| {
                acc.push(x.secondary);
                acc
            });

        sides
            .values()
            .map(|blocks| {
                let mut stack = blocks.iter().sorted().rev().collect_vec();
                // grab the first element out and start a continuous run
                let mut current = stack.pop().unwrap();
                let mut continuous = 1;

                // go over the next elements and when we find a discontinuity, the next fence is a new segment
                while let Some(next) = stack.pop() {
                    if *next != *current + 1 {
                        continuous += 1;
                    }

                    current = next;
                }

                continuous
            })
            .sum()
    }
}

fn get_regions(input: &Input) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];
    let mut seen: BTreeSet<Coord> = BTreeSet::new();

    for x in input.iter() {
        // don't visit squares we've already seen (from another search)
        if !seen.insert(x.coords) {
            continue;
        }

        let mut region = Region::new(*x.data);

        let mut queue = VecDeque::new();
        queue.push_back(x);

        while let Some(current) = queue.pop_front() {
            // are we part of the same region?
            if *current.data == region.name {
                region.area += 1;
            }

            for dir in [
                CardinalDirection::North,
                CardinalDirection::South,
                CardinalDirection::East,
                CardinalDirection::West,
            ] {
                // only visit neighbors within the same region
                let neighbor = current.get_neighbor(dir);
                let region_neighbor = neighbor.filter(|n| n.data == x.data);

                if let Some(n) = region_neighbor {
                    // don't go back to previous neighbors
                    if seen.insert(n.coords) {
                        queue.push_back(n);
                    }
                } else {
                    region.sides.push(Edge::new(dir, current.coords))
                };
            }
        }

        regions.push(region);
    }

    regions
}

fn problem1(input: &Input) -> usize {
    get_regions(input)
        .iter()
        .map(|region| region.area * region.fences())
        .sum()
}

fn problem2(input: &Input) -> usize {
    get_regions(input)
        .iter()
        .map(|region| {
            // dbg!(region.name, region.area, region.unique_fences());

            region.area * region.unique_fences()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 1930)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1206)
    }
}
