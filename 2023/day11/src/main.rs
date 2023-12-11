use std::fmt::Debug;

use common::{extensions::PointExt, map::Map};
use itertools::Itertools;
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

type Input = Vec<Vec<Tile>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        many1(alt((
            map(char('.'), |_| Tile::Empty),
            map(char('#'), |_| Tile::Galaxy),
        ))),
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}

fn expand_universe(galaxy: Vec<Vec<Tile>>) -> Map<Tile> {
    // Expand any row that's all empty
    let mut expanded_rows: Vec<Vec<Tile>> = galaxy
        .into_iter()
        .flat_map(|row| {
            if row.iter().all(|t| *t == Tile::Empty) {
                vec![row.clone(), row]
            } else {
                vec![row]
            }
        })
        .collect();

    // Expand the inner vectors which are the columns (this is super gross, should probably have transposed? I don't know.)
    let mut max_width = expanded_rows[0].len();
    let mut x = 0;

    while x < max_width {
        let empty_colum = expanded_rows.iter().all(|r| r[x] == Tile::Empty);
        if empty_colum {
            (0..expanded_rows.len()).for_each(|y| {
                expanded_rows[y].insert(x, Tile::Empty);
            });
            x += 2;
            max_width += 1;
        } else {
            x += 1;
        }
    }

    Map::new(expanded_rows)
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Galaxy => write!(f, "#"),
            Self::Empty => write!(f, "."),
        }
    }
}

fn problem1(input: &Input) -> usize {
    let universe = expand_universe(input.clone());

    // find the manhattan distances between all the galaxies
    let distances: usize = universe
        .into_iter()
        .filter_map(|x| (*x.data == Tile::Galaxy).then_some(x.coords))
        .permutations(2)
        .map(|x| x[0].manhattan(&x[1]))
        .sum();

    // permutations gives us all pairs twice...so rather than sorting pairs and de-duping...just divide by 2
    distances / 2
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
        assert_eq!(result, 374)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
