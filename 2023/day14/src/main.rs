use std::{fmt::Debug, usize};

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
enum Tile {
    RoundedRock,
    Empty,
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

        Self {
            platform: v,
            height,
            width,
        }
    }
    fn get_column(&self, i: usize) -> Vec<Tile> {
        self.platform.iter().map(|row| row[i]).collect()
    }

    fn load(&self) -> usize {
        (0..self.width)
            .map(|x| {
                self.get_column(x)
                    .iter()
                    .enumerate()
                    .filter_map(|(row_num, tile)| {
                        (*tile == Tile::RoundedRock).then_some(self.height - row_num)
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn tilt_north(&mut self) {
        for i in 0..self.width {
            let col = self.get_column(i);
            let sorted = col
                // split at the rocks, inclusive
                .split_inclusive(|x| *x == Tile::CubeRock)
                .flat_map(|s| {
                    // sort them, because of the way the sort is defined, we'll get all the boulders first, then empty, then the cube rock
                    let mut s1 = s.to_vec().clone();
                    s1.sort();
                    s1
                })
                .collect::<Vec<Tile>>();

            // rearrange everything
            for (row_idx, new_tile) in sorted.iter().enumerate() {
                self.platform[row_idx][i] = *new_tile;
            }
        }
    }
}

fn problem1(input: &Input) -> usize {
    let mut platform: Platform = input.clone();
    platform.tilt_north();
    platform.load()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
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
        assert_eq!(result, 0)
    }
}
