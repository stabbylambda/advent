use std::collections::{BTreeMap, BTreeSet};

use common::{grid::Grid, nom::parse_grid};
use nom::{branch::alt, character::complete::char, combinator::map, IResult, Parser};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input, 200);
    println!("problem 2 answer: {answer}");
}

type Input = Grid<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('#'), |_| Tile::Bug),
        map(char('.'), |_| Tile::Space),
    ))).parse(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Bug,
    Space,
    Level,
}

impl Tile {
    fn tick(&self, neighbor_bugs: usize) -> Tile {
        match (self, neighbor_bugs) {
            (Tile::Bug, 1) => Tile::Bug,
            (Tile::Bug, _) => Tile::Space,
            (Tile::Space, 1) | (Tile::Space, 2) => Tile::Bug,
            _ => Tile::Space,
        }
    }
}

fn tick(map: &Grid<Tile>) -> Grid<Tile> {
    let mut new_map = Grid::new(vec![vec![Tile::Space; 5]; 5]);
    for tile in map.iter() {
        let neighbor_bug_count = tile
            .neighbors()
            .iter()
            .filter(|x| matches!(x.data, Tile::Bug))
            .count();

        let new_tile = tile.data.tick(neighbor_bug_count);

        new_map.set(tile.coords, new_tile);
    }
    new_map
}

fn biodiversity(map: &Grid<Tile>) -> u32 {
    map.iter().enumerate().fold(0, |acc, (idx, x)| {
        let base = match x.data {
            Tile::Bug => 1,
            Tile::Space => 0,
            _ => unreachable!(),
        };

        acc | (base << idx)
    })
}

fn problem1(input: &Input) -> u32 {
    let mut seen: BTreeSet<u32> = BTreeSet::new();
    let mut map = input.clone();

    loop {
        let key = biodiversity(&map);
        if !seen.insert(key) {
            return key;
        }

        map = tick(&map);
    }
}

type ErisCoord = (isize, usize, usize);
struct Eris {
    tiles: BTreeMap<ErisCoord, Tile>,
}

impl Eris {
    fn new(input: &Input) -> Self {
        let initial = input.clone();
        let tiles: BTreeMap<ErisCoord, Tile> = initial
            .iter()
            .map(|t| {
                let data = if t.coords == (2, 2) {
                    Tile::Level
                } else {
                    *t.data
                };
                ((0, t.coords.1, t.coords.0), data)
            })
            .collect();

        Self { tiles }
    }

    fn get_tile(&self, coords: ErisCoord) -> Tile {
        if let Some(tile) = self.tiles.get(&coords) {
            *tile
        } else {
            Tile::Space
        }
    }

    fn tick_tile(&self, tile: Tile, (level, y, x): ErisCoord) -> Option<Tile> {
        let up = level + 1;
        let down = level - 1;
        let top_neighbors = match (y, x) {
            (0, _) => vec![self.get_tile((down, 1, 2))], // A - E => 8
            (3, 2) => (0..5).map(|x| self.get_tile((up, 4, x))).collect(), // 18 => U - Y
            (_, _) => vec![self.get_tile((level, y - 1, x))], // normal
        };

        let left_neighbors = match (y, x) {
            (_, 0) => vec![self.get_tile((down, 2, 1))], // A, F, K, P, U => 12
            (2, 3) => (0..5).map(|y| self.get_tile((up, y, 4))).collect(), // 14 => E, J, O, T, Y
            (_, _) => vec![self.get_tile((level, y, x - 1))], // normal
        };

        let right_neighbors = match (y, x) {
            (_, 4) => vec![self.get_tile((down, 2, 3))], // E, J, O, T, Y => 14
            (2, 1) => (0..5).map(|y| self.get_tile((up, y, 0))).collect(), // 12 => A, F, K, P, U
            (_, _) => vec![self.get_tile((level, y, x + 1))], // normal
        };

        let bottom_neighbors = match (y, x) {
            (4, _) => vec![self.get_tile((down, 3, 2))], // U - Y => 18
            (1, 2) => (0..5).map(|x| self.get_tile((up, 0, x))).collect(), // 8 => A - E
            (_, _) => vec![self.get_tile((level, y + 1, x))], // normal
        };

        let neighbors = [
            top_neighbors,
            left_neighbors,
            right_neighbors,
            bottom_neighbors,
        ];

        let neighbor_bugs = neighbors
            .iter()
            .flatten()
            .filter(|x| **x == Tile::Bug)
            .count();

        match tile.tick(neighbor_bugs) {
            Tile::Bug => Some(Tile::Bug),
            _ => None,
        }
    }

    fn level_range(&self) -> (isize, isize) {
        let min = self.tiles.keys().min().unwrap().0;
        let max = self.tiles.keys().max().unwrap().0;

        (min, max)
    }

    fn all_coords(&self) -> impl Iterator<Item = ErisCoord> {
        let (min, max) = self.level_range();
        (min - 1..=max + 1)
            .flat_map(|level| (0..5).flat_map(move |y| (0..5).map(move |x| (level, y, x))))
    }

    fn tick(&self) -> Self {
        let new_tiles = self
            .all_coords()
            .filter_map(|(level, y, x)| {
                let tile = self.get_tile((level, y, x));
                let new_tile = match tile {
                    Tile::Level => None,
                    _ if x == 2 && y == 2 => None,
                    _ => self.tick_tile(tile, (level, y, x)),
                };

                // we only need to insert into the map if we've got a bug
                new_tile.map(|new_tile| ((level, y, x), new_tile))
            })
            .collect();

        Eris { tiles: new_tiles }
    }

    fn bug_count(&self) -> usize {
        self.tiles.values().filter(|x| **x == Tile::Bug).count()
    }
}
fn problem2(input: &Input, ticks: usize) -> usize {
    let eris = (0..ticks).fold(Eris::new(input), |eris, _n| eris.tick());
    eris.bug_count()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 2129920)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input, 10);
        assert_eq!(result, 99)
    }
}
