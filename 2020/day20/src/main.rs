use std::{collections::BTreeMap, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
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

type Input = Vec<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        tag("\n\n"),
        map(
            separated_pair(
                delimited(tag("Tile "), u64, tag(":")),
                newline,
                separated_list1(
                    newline,
                    many1(alt((map(char('#'), |_| true), map(char('.'), |_| false)))),
                ),
            ),
            |(id, v)| Tile::new(id, v),
        ),
    )(input);

    result.unwrap().1
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Tile {
    id: u64,
    value: Vec<Vec<bool>>,
}
impl Tile {
    fn new(id: u64, v: Vec<Vec<bool>>) -> Self {
        Self { id, value: v }
    }

    fn generate_translations(&self) -> Vec<Self> {
        let original = self.clone();
        let r90 = self.rotate();
        let r180 = r90.rotate();
        let r270 = r180.rotate();
        let flip = self.flip();
        let f90 = flip.rotate();
        let f180 = f90.rotate();
        let f270 = f180.rotate();

        vec![original, r90, r180, r270, flip, f90, f180, f270]
    }

    fn flip(&self) -> Self {
        Self {
            id: self.id,
            value: self.value.clone().into_iter().rev().collect(),
        }
    }

    fn rotate(&self) -> Self {
        let mut matrix = self.value.clone();
        matrix.reverse();
        for i in 0..matrix.len() {
            for j in i..matrix.len() {
                // I'm sure we can use mem::swap here, but that probably involves slicing and isn't as clear
                let x = matrix[i][j];
                let y = matrix[j][i];
                matrix[j][i] = x;
                matrix[i][j] = y;
            }
        }

        Self {
            id: self.id,
            value: matrix,
        }
    }

    fn get_edge_ids(&self) -> Vec<Vec<bool>> {
        let top = self.value.first().cloned().unwrap();
        let left = self
            .value
            .iter()
            .map(|r| r.first().unwrap())
            .cloned()
            .collect();
        let bottom = self.value.last().cloned().unwrap();
        let right = self
            .value
            .iter()
            .map(|r| r.last().unwrap())
            .cloned()
            .collect();
        vec![top, left, bottom, right]
            .iter()
            .map(|e| e.clone().min(e.iter().rev().cloned().collect()))
            .collect()
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

fn sort_tiles(input: &Input) -> Vec<(TileType, Tile)> {
    let mut edges: BTreeMap<Vec<bool>, Vec<u64>> = BTreeMap::new();
    for x in input {
        for e in x.get_edge_ids() {
            edges
                .entry(e)
                .and_modify(|v| v.push(x.id))
                .or_insert(vec![x.id]);
        }
    }

    input
        .iter()
        .filter_map(|x| {
            let matching_edges = x
                .get_edge_ids()
                .iter()
                .filter(|e| edges[*e].len() == 2)
                .count();

            match matching_edges {
                2 => Some((TileType::Corner, x.clone())),
                3 => Some((TileType::Edge, x.clone())),
                4 => Some((TileType::Inner, x.clone())),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[derive(Debug)]
enum TileType {
    Corner,
    Edge,
    Inner,
}

fn problem1(input: &Input) -> u64 {
    let tiles = sort_tiles(input);
    tiles
        .iter()
        .filter(|x| matches!(x.0, TileType::Corner))
        .map(|x| x.1.id)
        .product()
}

fn problem2(input: &Input) -> u32 {
    let side = (input.len() as f64).sqrt() as usize;
    let mut board: Vec<Vec<Option<Tile>>> = vec![vec![None; side]; side];

    let tiles = sort_tiles(input);

    let first = tiles
        .iter()
        .filter(|x| matches!(x.0, TileType::Corner))
        .next()
        .unwrap();

    dbg!(first);

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
        assert_eq!(result, 20899048083289)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
