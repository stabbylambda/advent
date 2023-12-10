use std::fmt::Display;

use common::map::{Map, MapSquare, Neighbors};
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(&input);

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
                map(char('|'), |_| Tile::Vertical),
                map(char('-'), |_| Tile::Horizontal),
                map(char('L'), |_| Tile::BottomLeft),
                map(char('J'), |_| Tile::BottomRight),
                map(char('7'), |_| Tile::TopRight),
                map(char('F'), |_| Tile::TopLeft),
                map(char('.'), |_| Tile::Ground),
                map(char('S'), |_| Tile::StartingPosition),
            ))),
        ),
        Map::new,
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
    Ground,
    StartingPosition,
}

impl Tile {
    fn connects_north(&self) -> bool {
        matches!(
            self,
            Tile::StartingPosition | Tile::Vertical | Self::BottomLeft | Tile::BottomRight
        )
    }

    fn connects_south(&self) -> bool {
        matches!(
            self,
            Tile::StartingPosition | Tile::Vertical | Self::TopLeft | Tile::TopRight
        )
    }
    fn connects_east(&self) -> bool {
        matches!(
            self,
            Tile::StartingPosition | Tile::Horizontal | Self::TopLeft | Tile::BottomLeft
        )
    }
    fn connects_west(&self) -> bool {
        matches!(
            self,
            Tile::StartingPosition | Tile::Horizontal | Self::TopRight | Tile::BottomRight
        )
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Vertical => '|',
            Tile::Horizontal => '-',
            Tile::TopLeft => 'F',
            Tile::BottomLeft => 'L',
            Tile::TopRight => '7',
            Tile::BottomRight => 'J',
            Tile::Ground => '.',
            Tile::StartingPosition => 'S',
        };
        write!(f, "{c}")
    }
}

/**  figure out if this tile connects to its neighbors based on the neighboring tile type */
fn connecting_neighbors<'a>(map_square: &MapSquare<'a, Tile>) -> Neighbors<'a, Tile> {
    let n = map_square.neighbors();

    match map_square.data {
        Tile::Vertical => Neighbors {
            north: n.north.filter(|x| x.data.connects_south()),
            south: n.south.filter(|x| x.data.connects_north()),
            east: None,
            west: None,
        },
        Tile::Horizontal => Neighbors {
            east: n.east.filter(|x| x.data.connects_west()),
            west: n.west.filter(|x| x.data.connects_east()),
            north: None,
            south: None,
        },
        Tile::TopLeft => Neighbors {
            south: n.south.filter(|x| x.data.connects_north()),
            east: n.east.filter(|x| x.data.connects_west()),
            north: None,
            west: None,
        },
        Tile::BottomLeft => Neighbors {
            north: n.north.filter(|x| x.data.connects_south()),
            east: n.east.filter(|x| x.data.connects_west()),
            south: None,
            west: None,
        },
        Tile::TopRight => Neighbors {
            south: n.south.filter(|x| x.data.connects_north()),
            west: n.west.filter(|x| x.data.connects_east()),
            north: None,
            east: None,
        },
        Tile::BottomRight => Neighbors {
            north: n.north.filter(|x| x.data.connects_south()),
            west: n.west.filter(|x| x.data.connects_east()),
            south: None,
            east: None,
        },
        Tile::Ground => Neighbors {
            north: None,
            south: None,
            east: None,
            west: None,
        },
        Tile::StartingPosition => Neighbors {
            north: n.north.filter(|x| x.data.connects_south()),
            south: n.south.filter(|x| x.data.connects_north()),
            east: n.east.filter(|x| x.data.connects_west()),
            west: n.west.filter(|x| x.data.connects_east()),
        },
    }
}

// Data to keep track of while searching
struct Path {
    steps: u32,
    current: (usize, usize),
    /** we keep track of from to ensure we don't go backward */
    from: (usize, usize),
}

fn problem1(input: &Input) -> u32 {
    // figure out where we start
    let start = input
        .into_iter()
        .find(|x| x.data == &Tile::StartingPosition)
        .unwrap();

    // find both connecting neighbors and pick one to travel down
    let mut path = connecting_neighbors(&start)
        .into_iter()
        .map(|x| Path {
            steps: 1,
            from: start.coords,
            current: x.coords,
        })
        .next()
        .unwrap();

    // keep going until we find the starting position again
    let path_length = loop {
        let current = input.get(path.current);
        if current.data == &Tile::StartingPosition {
            break path.steps;
        }

        // pick the first neighbor we haven't been to
        if let Some(coords) = connecting_neighbors(&current)
            .into_iter()
            .find(|x| x.coords != path.from)
            .map(|x| x.coords)
        {
            // update the path
            path = Path {
                steps: path.steps + 1,
                from: path.current,
                current: coords,
            }
        }
    };

    // obviously we only need the halfway point to find the farthest
    path_length / 2
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn simple() {
        let input = include_str!("../simple.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 4)
    }

    #[test]
    fn complex() {
        let input = include_str!("../complex.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 8)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
