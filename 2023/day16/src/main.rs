use std::collections::{BTreeSet, VecDeque};

use common::map::{Map, MapSquare};
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

type Input = Map<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(alt((
                map(char('.'), |_| Tile::Empty),
                map(char('/'), |_| Tile::ForwardMirror),
                map(char('\\'), |_| Tile::BackwardMirror),
                map(char('|'), |_| Tile::VerticalSplitter),
                map(char('-'), |_| Tile::HorizontalSplitter),
            ))),
        ),
        Map::new,
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    ForwardMirror,
    BackwardMirror,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn problem1(input: &Input) -> usize {
    use Direction::*;
    use Tile::*;
    let start = ((0, 0), Direction::East);
    let mut queue: VecDeque<((usize, usize), Direction)> = VecDeque::new();
    queue.push_back(start);

    let mut visited: BTreeSet<((usize, usize), Direction)> = BTreeSet::new();

    while let Some(data @ (coords, direction)) = queue.pop_front() {
        let tile = input.get(coords);
        if !visited.insert(data) {
            continue;
        };

        let new_tiles = match (tile.data, direction) {
            (Empty, North)
            | (VerticalSplitter, North)
            | (ForwardMirror, East)
            | (BackwardMirror, West) => {
                vec![tile.neighbors().north.map(|n| (n.coords, North))]
            }
            (Empty, South)
            | (VerticalSplitter, South)
            | (ForwardMirror, West)
            | (BackwardMirror, East) => {
                vec![tile.neighbors().south.map(|n| (n.coords, South))]
            }
            (Empty, East)
            | (HorizontalSplitter, East)
            | (ForwardMirror, North)
            | (BackwardMirror, South) => {
                vec![tile.neighbors().east.map(|n| (n.coords, East))]
            }
            (Empty, West)
            | (HorizontalSplitter, West)
            | (ForwardMirror, South)
            | (BackwardMirror, North) => {
                vec![tile.neighbors().west.map(|n| (n.coords, West))]
            }
            (VerticalSplitter, East) | (VerticalSplitter, West) => {
                vec![
                    tile.neighbors().north.map(|n| (n.coords, North)),
                    tile.neighbors().south.map(|n| (n.coords, South)),
                ]
            }
            (HorizontalSplitter, North) | (HorizontalSplitter, South) => {
                vec![
                    tile.neighbors().east.map(|n| (n.coords, East)),
                    tile.neighbors().west.map(|n| (n.coords, West)),
                ]
            }
        };

        for t in new_tiles.into_iter().flatten() {
            queue.push_back(t);
        }
    }

    let coords_only: BTreeSet<(usize, usize)> = visited.into_iter().map(|(c, _dir)| c).collect();
    coords_only.len()
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
        assert_eq!(result, 46)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
