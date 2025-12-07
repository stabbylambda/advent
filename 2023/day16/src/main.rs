use std::collections::{BTreeSet, VecDeque};

use common::{answer, grid::Grid, nom::parse_grid, read_input};
use nom::{branch::alt, character::complete::char, combinator::map, IResult, Parser};
use Direction::*;
use Tile::*;

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Grid<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('.'), |_| Tile::Empty),
        map(char('/'), |_| Tile::ForwardMirror),
        map(char('\\'), |_| Tile::BackwardMirror),
        map(char('|'), |_| Tile::VerticalSplitter),
        map(char('-'), |_| Tile::HorizontalSplitter),
    ))).parse(input);

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

type Key = ((usize, usize), Direction);

fn fire_beam(input: &Input, start: Key) -> usize {
    // We'll need to manage multiple beams per start
    let mut queue: VecDeque<Key> = VecDeque::new();
    queue.push_back(start);

    // keep track of where we've been and what direction we were going at the time
    let mut visited: BTreeSet<Key> = BTreeSet::new();

    while let Some(data @ (coords, direction)) = queue.pop_front() {
        // if we've already been here and going this direction, bail
        if !visited.insert(data) {
            continue;
        };

        // figure out where to go from here
        let tile = input.get(coords);
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

    // only consider unique coords, we don't care about the direction we were going when we were there
    let coords_only: BTreeSet<&(usize, usize)> = visited.iter().map(|(c, _dir)| c).collect();
    coords_only.len()
}

fn problem1(input: &Input) -> usize {
    fire_beam(input, ((0, 0), East))
}

fn problem2(input: &Input) -> usize {
    input
        .iter()
        .flat_map(|t| {
            let c @ (x, y) = t.coords;
            let top = y == 0;
            let left = x == 0;
            let right = x == input.width - 1;
            let bottom = y == input.height - 1;

            // only look at the edges, the corners get two directions
            if top && left {
                vec![(c, East), (c, South)]
            } else if top && right {
                vec![(c, West), (c, South)]
            } else if bottom && left {
                vec![(c, North), (c, East)]
            } else if bottom && right {
                vec![(c, North), (c, West)]
            } else if top {
                vec![(c, South)]
            } else if left {
                vec![(c, East)]
            } else if bottom {
                vec![(c, North)]
            } else if right {
                vec![(c, West)]
            } else {
                vec![]
            }
        })
        .map(|s| fire_beam(input, s))
        .max()
        .unwrap()
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
        assert_eq!(result, 51)
    }
}
