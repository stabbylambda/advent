use std::collections::HashMap;
use std::{fmt::Debug, usize};

use std::fmt::Display;

use common::extensions::vecvec::VecVec;

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
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

type Input = Platform;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(alt((
                map(char('#'), |_| Tile::CubeRock),
                map(char('O'), |_| Tile::RoundedRock),
                map(char('.'), |_| Tile::Empty),
            ))),
        ),
        Platform::new,
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/** Represents the tiles on the platform. Ordering is important because "tilting" is based on sort order */
enum Tile {
    Empty,
    RoundedRock,
    CubeRock,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoundedRock => write!(f, "O"),
            Self::Empty => write!(f, "."),
            Self::CubeRock => write!(f, "#"),
        }
    }
}

#[derive(Clone)]
struct Platform {
    platform: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Platform {
    fn new(v: Vec<Vec<Tile>>) -> Self {
        let height = v.len();
        let width = v[0].len();

        let mut s = Self {
            platform: v,
            height,
            width,
        };

        // rotate so that the end of each row is "north"
        s.rotate();
        s
    }

    fn load(&self) -> usize {
        self.platform
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter_map(|(weight, tile)| (*tile == Tile::RoundedRock).then_some(weight + 1))
                    .sum::<usize>()
            })
            .sum()
    }

    fn rotate(&mut self) {
        let platform = self.platform.rotate();

        self.width = platform[0].len();
        self.height = platform.len();
        self.platform = platform;
    }

    fn spin_cycle(&mut self) {
        // go through an entire cycle
        for _dir in 0..4 {
            self.tilt();
            self.rotate();
        }
    }

    /** Tilt the platform. We always orient so that the direction we're tilting is row-wise to the end. That way we don't
     * have to deal with stupid columnar math. Should also make Vec operations faster since we have better memory-locality?
     */
    fn tilt(&mut self) {
        let sorted: Vec<Vec<Tile>> = self
            .platform
            .iter()
            .map(|slice| {
                slice
                    // split at the cube rocks, inclusive, so we get slices that end in a rock
                    .split_inclusive(|x| *x == Tile::CubeRock)
                    .flat_map(|s| {
                        let mut s1 = s.to_vec().clone();
                        // sort them in place, this will get all the empty spaces, then the boulders, then the cube rocks
                        // which is neat because then we basically have "falling" for free
                        s1.sort();
                        s1
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect();

        self.platform = sorted;
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = self;

        for x in 0..p.width {
            for y in 0..p.height {
                write!(f, "{:?}", p.platform[x][y])?;
            }

            writeln!(f)?;
        }

        writeln!(f)?;

        Ok(())
    }
}

fn problem1(input: &Input) -> usize {
    let mut platform: Platform = input.clone();
    platform.tilt();
    platform.load()
}

fn problem2(input: &Input) -> usize {
    let mut platform: Platform = input.clone();
    // keep a cache of the boards and the last cycle we saw that configuration
    let mut platform_cache: HashMap<String, usize> = HashMap::new();

    let limit = 1_000_000_000;

    // start on cycle 1
    let mut current = 1;
    let mut load = 0;

    while current < limit {
        // do the spin cycle and get the load
        platform.spin_cycle();
        load = platform.load();

        // insert the current state of the board and get the last time we saw it
        if let Some(previous) = platform_cache.insert(platform.to_string(), current) {
            // if we've seen it before, then we need to do some cool cycle math from 2022 day 17
            let cycle_size = previous.abs_diff(current);
            let skip_count = (limit - current) / cycle_size;
            let skipped = skip_count * cycle_size;

            println!(
                "Found cycle at {current}. Skipping {skip_count} cycles of {cycle_size} size for {skipped}"
            );

            // skip that count
            current += skipped;

            // don't let it find a new cycle on the next line, just blow away everything
            platform_cache.clear();
        } else {
            current += 1;
        };
    }

    load
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn load_test() {
        let input = include_str!("../count_test.txt");
        let input = parse(input);
        let result = input.load();
        assert_eq!(result, 136)
    }
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 136)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 64)
    }
}
