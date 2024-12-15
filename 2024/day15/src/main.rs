use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::Display,
    time::Instant,
};

use common::{
    grid::{CardinalDirection, Coord, Grid},
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
            Self::Space => write!(f, "."),
            Self::Robot => write!(f, "@"),
            Self::Wall => write!(f, "#"),
            Self::Box => write!(f, "O"),
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

fn simulate_moves(grid: &Grid<Tile>, robot: Coord, dir: CardinalDirection) -> Vec<(Coord, Tile)> {
    // find the first free space in the direction we're moving
    let mut queue = VecDeque::new();
    queue.push_back((robot, Tile::Robot));

    let mut result: Vec<(Coord, Tile)> = vec![];
    while let Some((current, tile)) = queue.pop_front() {
        // if there's no neighbor, we're off the grid (which is impossible, but whatever)
        let Some(next) = grid.get_neighbor(current, dir) else {
            continue;
        };

        match *next.data {
            Tile::Space => {
                result.push((next.coords, tile));
            }
            Tile::Wall => {
                // we hit a wall, clear the result and the queue
                result.clear();
                queue.clear();
            }
            _ => {
                result.push((current, *next.data));
                queue.push_back((next.coords, *next.data));
            }
        }
    }
    result
}

fn problem1(input: &Input) -> usize {
    let (mut grid, directions) = input.clone();
    let mut robot = grid
        .iter()
        .find_map(|x| (x.data == &Tile::Robot).then_some(x.coords))
        .unwrap();

    for d in directions {
        let (data, coords) = {
            let n = grid.get_neighbor(robot, d).unwrap();
            let data = n.data;
            let coords = n.coords;

            (*data, coords)
        };

        match data {
            // if it's a free space, move into it
            Tile::Space => {
                grid.set(robot, Tile::Space);
                grid.set(coords, Tile::Robot);
                robot = coords;
            }
            // if it's a box, we need to figure out if it can move into a free space
            // in that direction
            Tile::Box => {
                let mut moves = simulate_moves(&grid, robot, d);
                if moves.is_empty() {
                    continue;
                }

                while let Some((free_space, tile)) = moves.pop() {
                    grid.set(free_space, tile);
                }

                grid.set(coords, Tile::Robot);
                grid.set(robot, Tile::Space);
                robot = coords;
            }
            // if it's anything else (wall or...robot?), do nothing
            _ => {}
        }
    }

    // get the gps of all the boxes
    grid.iter()
        .filter(|x| x.data == &Tile::Box)
        .map(|x| x.coords.1 * 100 + x.coords.0)
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum WideTile {
    Space,
    Robot,
    Wall,
    SmallBox,
    BoxLeft,
    BoxRight,
}

impl Display for WideTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space => write!(f, "."),
            Self::Robot => write!(f, "@"),
            Self::Wall => write!(f, "#"),
            Self::BoxLeft => write!(f, "["),
            Self::BoxRight => write!(f, "]"),
            Self::SmallBox => write!(f, "O"),
        }
    }
}

fn make_wide(grid: Grid<Tile>) -> Grid<WideTile> {
    let wide_grid: Vec<Vec<WideTile>> = grid
        .points
        .into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|t| match t {
                    Tile::Space => vec![WideTile::Space, WideTile::Space],
                    Tile::Robot => vec![WideTile::Robot, WideTile::Space],
                    Tile::Wall => vec![WideTile::Wall, WideTile::Wall],
                    Tile::Box => vec![WideTile::BoxLeft, WideTile::BoxRight],
                })
                .collect::<Vec<WideTile>>()
        })
        .collect();

    Grid::new(wide_grid)
}

fn simulate_moves_wide(
    grid: &Grid<WideTile>,
    robot: Coord,
    dir: CardinalDirection,
) -> BTreeMap<Coord, WideTile> {
    // find the first free space in the direction we're moving
    let mut seen: BTreeSet<Coord> = BTreeSet::new();
    let mut result: BTreeMap<Coord, WideTile> = BTreeMap::new();

    let mut queue = VecDeque::from([robot]);
    while let Some(current) = queue.pop_front() {
        // we've already looked at this point
        if !seen.insert(current) {
            continue;
        }

        // if there's no neighbor, we're off the grid (which is impossible, but whatever)
        let Some(next) = grid.get_neighbor(current, dir) else {
            continue;
        };

        match *next.data {
            // we hit a wall, clear the result and the queue
            WideTile::Wall => {
                seen.clear();
                queue.clear();
            }
            WideTile::SmallBox => {
                queue.push_back(next.coords);
            }
            WideTile::BoxLeft => {
                queue.push_back(next.coords);
                queue.push_back((next.coords.0 + 1, next.coords.1));
            }
            WideTile::BoxRight => {
                queue.push_back(next.coords);
                queue.push_back((next.coords.0 - 1, next.coords.1));
            }
            _ => {}
        }
    }

    let boxes = seen
        .into_iter()
        .sorted_by_key(|(x, y)| Reverse((x.abs_diff(robot.0), y.abs_diff(robot.1))));
    for b in boxes {
        let n = grid.get_neighbor(b, dir).unwrap();
        let c = grid.get_opt(b).unwrap();

        result.insert(n.coords, *c.data);
        result.insert(c.coords, WideTile::Space);
    }

    result
}

fn problem2(input: &Input) -> usize {
    let (grid, directions) = input.clone();
    let mut grid = make_wide(grid);
    let mut robot = grid
        .iter()
        .find_map(|x| (x.data == &WideTile::Robot).then_some(x.coords))
        .unwrap();

    for d in directions {
        let (data, coords) = {
            let n = grid.get_neighbor(robot, d).unwrap();
            let data = n.data;
            let coords = n.coords;

            (*data, coords)
        };

        match data {
            // if it's a free space, move into it
            WideTile::Space => {
                grid.set(robot, WideTile::Space);
                grid.set(coords, WideTile::Robot);
                robot = coords;
            }
            // if it's a box, we need to figure out if it can move into a free space
            // in that direction
            WideTile::BoxLeft | WideTile::BoxRight => {
                let moves = simulate_moves_wide(&grid, robot, d);
                if moves.is_empty() {
                    continue;
                }

                for (free_space, tile) in moves {
                    grid.set(free_space, tile);
                }

                grid.set(coords, WideTile::Robot);
                grid.set(robot, WideTile::Space);
                robot = coords;
            }
            // if it's anything else (wall or...robot?), do nothing
            _ => {}
        }
    }

    // get the gps of all the boxes
    grid.iter()
        .filter(|x| x.data == &WideTile::BoxLeft)
        .map(|x| x.coords.1 * 100 + x.coords.0)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn small() {
        let input = include_str!("../test_small.txt");
        let input = parse(input);

        let result = problem1(&input);
        assert_eq!(result, 2028)
    }

    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);

        let result = problem1(&input);
        assert_eq!(result, 10092)
    }

    #[test]
    fn hallway_left() {
        let input = "#######
#..OO@#
#######

<<<<";
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 206)
    }

    #[test]
    fn hallway_right() {
        let input = "#######
#@OO..#
#######

>>>>";
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 216)
    }

    #[test]
    fn small_vertical_hallway() {
        let input = "###
#.#
#.#
#.#
#.#
#O#
#O#
#@#
###

>>v>^^^";
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 504)
    }
    #[test]
    fn vertical_hallway() {
        let input = "#..#..#
#.....#
#.....#
#.....#
#.....#
#..O..#
#.@O..#
#.....#
#..#..#

>>v>^^^";
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 513)
    }

    #[test]
    fn second_small() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 9021)
    }
}
