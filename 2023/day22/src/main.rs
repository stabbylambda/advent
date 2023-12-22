use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display},
};

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
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

type Input = Vec<Brick>;

fn parse(input: &str) -> Input {
    let mut idx = 0usize;
    let triple = |s| {
        map(
            tuple((terminated(u32, tag(",")), terminated(u32, tag(",")), u32)),
            |(x, y, z)| Triple::new(z, y, x),
        )(s)
    };
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(separated_pair(triple, tag("~"), triple), |(start, end)| {
            idx += 1;
            Brick::new(idx, start, end)
        }),
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Triple {
    z: u32,
    y: u32,
    x: u32,
}

impl Triple {
    fn new(z: u32, y: u32, x: u32) -> Self {
        Self { z, y, x }
    }

    fn below(&self) -> Self {
        Self {
            z: self.z - 1,
            y: self.y,
            x: self.x,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Brick {
    id: usize,
    min: Triple,
    max: Triple,
    cubes: BTreeSet<Triple>,
}

impl Brick {
    fn new(id: usize, start: Triple, end: Triple) -> Self {
        let min = start.min(end);
        let max = start.max(end);
        let cubes = Self::cubes(min, max);
        Self {
            id,
            min,
            max,
            cubes,
        }
    }

    fn on_ground(&self) -> bool {
        self.min.z == 1
    }

    fn move_down(&mut self) {
        self.min.z -= 1;
        self.max.z -= 1;
        // this is probably inefficient?
        self.cubes = Self::cubes(self.min, self.max);
    }

    // generate all the cubes for this brick
    fn cubes(min: Triple, max: Triple) -> BTreeSet<Triple> {
        (min.z..=max.z)
            .flat_map(|z| {
                (min.y..=max.y)
                    .flat_map(move |y| (min.x..=max.x).map(move |x| Triple::new(z, y, x)))
            })
            .collect()
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {},{},{}~{},{},{}",
            self.id, self.min.z, self.min.y, self.min.x, self.max.z, self.max.y, self.max.x
        )
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min.cmp(&other.min).then(self.max.cmp(&other.max))
    }
}

fn problem1(input: &Input) -> usize {
    // sort the bricks by z index
    let mut bricks = input.clone();
    bricks.sort();

    // The 3D tower, with brick id being the value
    let mut settled: BTreeMap<Triple, usize> = BTreeMap::new();

    // the "graph" indicating which bricks support other bricks
    let mut holding_up: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut sitting_on: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for brick in bricks.iter_mut() {
        // set up our graph from here
        holding_up.insert(brick.id, vec![]);
        sitting_on.insert(brick.id, vec![]);

        // move down until we find something below us
        let bricks_below = loop {
            // if we hit the ground, return nothing
            if brick.on_ground() {
                break vec![];
            }

            let bricks_below: Vec<usize> = brick
                .cubes
                .iter()
                .filter_map(|c| settled.get(&c.below()))
                .cloned()
                .collect();

            // if there are bricks below us, return their IDs
            if !bricks_below.is_empty() {
                break bricks_below;
            }

            brick.move_down();
        };

        // now that we dropped all the way, set the id where we landed
        settled.extend(brick.cubes.iter().map(|c| (*c, brick.id)));

        // set up the graph for each of the bricks
        for below in bricks_below {
            holding_up.entry(below).or_default().push(brick.id);
            sitting_on.entry(brick.id).or_default().push(below);
        }
    }

    holding_up
        .into_iter()
        .filter(|(_brick, above)| above.iter().all(|a| sitting_on[&a].len() != 1))
        .count()
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
        assert_eq!(result, 5)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
