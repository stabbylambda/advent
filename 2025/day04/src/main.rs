use std::fmt::Debug;

use nom::{branch::alt, character::complete::char, combinator::map, IResult, Parser};

use common::grid::{Grid, GridSquare};
use common::nom::parse_grid;

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Roll,
    Empty,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Roll => write!(f, "@"),
            Self::Empty => write!(f, "."),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Roll => write!(f, "@"),
            Tile::Empty => write!(f, "."),
        }
    }
}

type Input = Grid<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('@'), |_| Tile::Roll),
        map(char('.'), |_| Tile::Empty),
    )))
    .parse(input);

    result.unwrap().1
}

fn removable<'a>(grid: &'a Grid<Tile>) -> Vec<GridSquare<'a, Tile>> {
    let v = grid
        .iter()
        .filter(|x| x.data == &Tile::Roll)
        .filter(|x| {
            let rolls = x
                .all_neighbors()
                .iter()
                .filter(|n| n.data == &Tile::Roll)
                .count();
            rolls < 4
        })
        .collect::<Vec<_>>();

    v
}

fn problem1(x: &Input) -> usize {
    removable(x).len()
}

fn remove_rolls(count: usize, grid: &Grid<Tile>) -> usize {
    let r = removable(grid);
    let new_count = r.len();
    if new_count == 0 {
        return count;
    }

    let mut new_grid = grid.clone();
    for x in r {
        new_grid.set(x.coords, Tile::Empty);
    }

    remove_rolls(count + new_count, &new_grid)
}

fn problem2(x: &Input) -> usize {
    remove_rolls(0, x)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 43)
    }
}
