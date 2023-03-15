use std::{collections::HashMap, fmt::Display};

use common::nom::usize;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    sequence::{preceded, separated_pair},
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

type Input = Cave;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_pair(
            preceded(tag("depth: "), usize),
            newline,
            preceded(tag("target: "), separated_pair(usize, tag(","), usize)),
        ),
        |(depth, target)| Cave::new(depth, target),
    )(input);

    result.unwrap().1
}

#[derive(Clone)]
struct Cave {
    depth: usize,
    target: (usize, usize),
    erosion_cache: HashMap<(usize, usize), usize>,
}

impl Cave {
    fn new(depth: usize, target: (usize, usize)) -> Self {
        Cave {
            depth,
            target,
            erosion_cache: HashMap::new(),
        }
    }

    fn get_erosion_values(&self) -> Vec<usize> {
        self.erosion_cache.values().copied().collect()
    }

    fn geologic_index(&mut self, (x, y): (usize, usize)) -> usize {
        match (x, y) {
            (0, 0) => 0,
            (_, 0) => x * 16807,
            (0, _) => y * 48271,
            _ if x == self.target.0 && y == self.target.1 => 0,
            _ => self.erosion_level((x - 1, y)) * self.erosion_level((x, y - 1)),
        }
    }

    fn erosion_level(&mut self, (x, y): (usize, usize)) -> usize {
        if let Some(erosion) = self.erosion_cache.get(&(x, y)) {
            *erosion
        } else {
            let erosion = (self.geologic_index((x, y)) + self.depth) % 20183;
            self.erosion_cache.insert((x, y), erosion);
            erosion
        }
    }

    fn get_type(&self, (x, y): (usize, usize)) -> RegionType {
        match self.erosion_cache.get(&(x, y)).unwrap() % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => unreachable!(),
        }
    }

    fn explore(&mut self, padding: usize) {
        let (tx, ty) = self.target;
        for y in 0..=ty + padding {
            for x in 0..=tx + padding {
                self.erosion_level((x, y));
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegionType::Rocky => '.',
                RegionType::Wet => '=',
                RegionType::Narrow => '|',
            }
        )
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (tx, ty) = self.target;
        for y in 0..=ty {
            for x in 0..=tx {
                match (x, y) {
                    (0, 0) => write!(f, "M")?,
                    _ if x == tx && y == ty => write!(f, "T")?,
                    _ => write!(f, "{}", self.get_type((x, y)))?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn problem1(input: &Input) -> usize {
    //Explore the cave. This primes the erosion cache, which we need to calculate
    let mut cave = input.clone();
    cave.explore(0);

    cave.get_erosion_values().iter().map(|x| x % 3).sum()
}

fn problem2(input: &Input) -> u32 {
    let mut cave = input.clone();
    // explore the cave and add some padding because we might need to
    // travel past the target and double back for time
    cave.explore(100);

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
        assert_eq!(result, 114)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
