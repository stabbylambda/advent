use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    time::Instant,
};

use common::{
    grid::{Coord, Grid},
    nom::{parse_grid, single_digit},
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

type Input = Grid<u32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        parse_grid(alt((single_digit, map(char('.'), |_| 5))))(input);

    result.unwrap().1
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    start: Coord,
    current: Coord,
    height: u32,
}

impl Path {
    fn new(c: Coord) -> Self {
        Self {
            start: c,
            current: c,
            height: 0,
        }
    }

    fn move_to(&self, c: Coord) -> Self {
        Self {
            start: self.start,
            current: c,
            height: self.height + 1,
        }
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.height.cmp(&other.height)
    }
}

fn find_trails(input: &Input) -> BTreeMap<Coord, Vec<Coord>> {
    let mut trailheads: BTreeMap<Coord, Vec<Coord>> = input
        .iter()
        .filter_map(|x| (*x.data == 0).then_some((x.coords, vec![])))
        .collect();

    let mut paths: BinaryHeap<Path> = trailheads.keys().map(|c| Path::new(*c)).collect();

    while let Some(path) = paths.pop() {
        let current = input.get(path.current);
        // did we find the end?
        if *current.data == 9 {
            trailheads.entry(path.start).and_modify(|c| {
                c.push(current.coords);
            });
            continue;
        }

        // walk to the neighbors that are only a jump of 1
        for n in current
            .neighbors()
            .iter()
            .filter(|x| *x.data == path.height + 1)
        {
            paths.push(path.move_to(n.coords));
        }
    }

    trailheads
}

fn problem1(input: &Input) -> u32 {
    // only find unique solutions
    find_trails(input)
        .values()
        .map(|x| BTreeSet::from_iter(x.iter()).len() as u32)
        .sum()
}

fn problem2(input: &Input) -> u32 {
    // find and sum all solutions
    find_trails(input).values().map(|x| x.len() as u32).sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 36)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 81)
    }
}
