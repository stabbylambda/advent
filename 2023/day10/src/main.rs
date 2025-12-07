use std::fmt::Display;

use common::{
    answer,
    grid::{Grid, GridSquare, Neighbors},
    nom::parse_grid,
    read_input,
};
use nom::{branch::alt, character::complete::char, combinator::map, IResult, Parser};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Grid<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('|'), |_| Tile::Vertical),
        map(char('-'), |_| Tile::Horizontal),
        map(char('L'), |_| Tile::BottomLeft),
        map(char('J'), |_| Tile::BottomRight),
        map(char('7'), |_| Tile::TopRight),
        map(char('F'), |_| Tile::TopLeft),
        map(char('.'), |_| Tile::Ground),
        map(char('S'), |_| Tile::StartingPosition),
    ))).parse(input);

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
fn connecting_neighbors<'a>(map_square: &GridSquare<'a, Tile>) -> Neighbors<'a, Tile> {
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
    current: (usize, usize),
    /** we keep track of from to ensure we don't go backward */
    from: Vec<(usize, usize)>,
}

fn find_path(input: &Input) -> Path {
    // figure out where we start
    let start = input
        .iter()
        .find(|x| x.data == &Tile::StartingPosition)
        .unwrap();

    // find both connecting neighbors and pick one to travel down
    let mut path = connecting_neighbors(&start)
        .iter()
        .map(|x| Path {
            from: vec![start.coords],
            current: x.coords,
        })
        .next()
        .unwrap();

    // keep going until we find the starting position again
    loop {
        let current = input.get(path.current);
        if current.data == &Tile::StartingPosition {
            return path;
        }

        // pick the first neighbor we haven't been to
        if let Some(coords) = connecting_neighbors(&current)
            .iter()
            .find(|x| x.coords != *path.from.last().unwrap())
            .map(|x| x.coords)
        {
            // update the path
            path.from.push(path.current);
            path.current = coords;
        }
    }
}

fn problem1(input: &Input) -> usize {
    let path = find_path(input);
    // obviously we only need the halfway point to find the farthest
    path.from.len() / 2
}

fn problem2(input: &Input) -> i64 {
    // get the path from part1 and make it a closed loop by pushing the starting location
    let mut path = find_path(input).from;
    path.push(*path.first().unwrap());

    // Get all the points into i64s so we can do the math correctly (you know, to avoid underflow)
    let points: Vec<(i64, i64)> = path.iter().map(|(x, y)| (*x as i64, *y as i64)).collect();

    // Do the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula#Shoelace_formula)
    let area: i64 = points
        .windows(2)
        .map(|w| {
            let (x1, y1) = w[0];
            let (x2, y2) = w[1];

            (y1 + y2) * (x2 - x1)
        })
        .sum();

    // And apply [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem)
    area.abs() / 2 - (path.len() as i64) / 2 + 1
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
    fn basic_area() {
        let input = include_str!("../basic_area.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 4)
    }

    #[test]
    fn larger_area() {
        let input = include_str!("../larger_area.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 10)
    }
}
