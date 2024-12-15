use std::{fmt::Display, time::Instant};

use common::{
    grid::{CardinalDirection, Grid},
    nom::parse_grid,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let i = Instant::now();
    let score = problem1(&input);
    let d = i.elapsed();
    println!("problem 1 score: {score} in {d:?}");

    let i = Instant::now();
    let score = problem2(&input);
    let d = i.elapsed();
    println!("problem 2 score: {score} in {d:?}");
}

type Input = (Grid<Tile>, Vec<CardinalDirection>);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Space,
    Robot,
    Wall,
    Box,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Space => write!(f, "."),
            Tile::Robot => write!(f, "@"),
            Tile::Wall => write!(f, "#"),
            Tile::Box => write!(f, "O"),
        }
    }
}

fn parse(input: &str) -> Input {
    let tile = parse_grid(alt((
        map(char('@'), |_| Tile::Robot),
        map(char('.'), |_| Tile::Space),
        map(char('O'), |_| Tile::Box),
        map(char('#'), |_| Tile::Wall),
    )));

    let directions = map(
        separated_list1(
            newline,
            many1(alt((
                map(char('^'), |_| CardinalDirection::North),
                map(char('v'), |_| CardinalDirection::South),
                map(char('<'), |_| CardinalDirection::West),
                map(char('>'), |_| CardinalDirection::East),
            ))),
        ),
        |x| x.into_iter().flatten().collect::<Vec<CardinalDirection>>(),
    );

    let result: IResult<&str, Input> = separated_pair(tile, tag("\n\n"), directions)(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let (mut grid, directions) = input.clone();
    let mut robot = grid
        .iter()
        .find(|x| x.data == &Tile::Robot)
        .map(|x| x.coords)
        .unwrap();

    for d in directions {
        let (data, coords) = {
            let n = grid.get_neighbor(robot, d).unwrap();
            let data = n.data;
            let coords = n.coords;

            (data, coords)
        };

        match *data {
            // if it's a free space, move into it
            Tile::Space => {
                grid.set(robot, Tile::Space);
                grid.set(coords, Tile::Robot);
                robot = coords;
            }
            // if it's a box, we need to figure out if it can move into a free space
            // in that direction
            Tile::Box => {
                // find the first free space in the direction we're moving
                let mut current = coords;
                let mut free = None;
                while let Some(next) = grid.get_neighbor(current, d) {
                    if next.data == &Tile::Space {
                        free = Some(next.coords);
                        break;
                    } else if next.data == &Tile::Wall {
                        break;
                    } else {
                        current = next.coords;
                    }
                }

                if let Some(free_space) = free {
                    grid.set(free_space, Tile::Box);
                    grid.set(coords, Tile::Robot);
                    grid.set(robot, Tile::Space);
                    robot = coords;
                }
            }
            // if it's anything else (wall or...robot?), do nothing
            _ => {}
        }
    }

    grid.iter()
        .filter(|x| x.data == &Tile::Box)
        .map(|x| x.coords.1 * 100 + x.coords.0)
        .sum()
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
        assert_eq!(result, 10092)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
