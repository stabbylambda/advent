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

fn parse(input: &str) -> Input {
    let tile = parse_grid(alt((
        map(char('@'), |_| Tile::Robot),
        map(char('.'), |_| Tile::Space),
        map(char('O'), |_| Tile::SmallBox),
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Space,
    Robot,
    Wall,
    SmallBox,
    BoxLeft,
    BoxRight,
}

impl Display for Tile {
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

fn make_wide(grid: Grid<Tile>) -> Grid<Tile> {
    let wide_grid: Vec<Vec<Tile>> = grid
        .points
        .into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|t| match t {
                    Tile::Space => vec![Tile::Space, Tile::Space],
                    Tile::Robot => vec![Tile::Robot, Tile::Space],
                    Tile::Wall => vec![Tile::Wall, Tile::Wall],
                    Tile::SmallBox => vec![Tile::BoxLeft, Tile::BoxRight],
                    _ => unreachable!(),
                })
                .collect::<Vec<Tile>>()
        })
        .collect();

    Grid::new(wide_grid)
}

fn simulate_moves_wide(
    grid: &Grid<Tile>,
    robot: Coord,
    dir: CardinalDirection,
) -> BTreeMap<Coord, Tile> {
    // find the first free space in the direction we're moving
    let mut seen: BTreeSet<Coord> = BTreeSet::new();
    let mut result: BTreeMap<Coord, Tile> = BTreeMap::new();

    let mut queue = VecDeque::new();
    queue.push_back(robot);

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
            Tile::Wall => {
                seen.clear();
                queue.clear();
            }
            Tile::SmallBox => {
                queue.push_back(next.coords);
            }
            Tile::BoxLeft => {
                queue.push_back(next.coords);
                queue.push_back((next.coords.0 + 1, next.coords.1));
            }
            Tile::BoxRight => {
                queue.push_back(next.coords);
                queue.push_back((next.coords.0 - 1, next.coords.1));
            }
            _ => {}
        }
    }

    let boxes = seen
        .into_iter()
        .sorted_by_key(|(x, y)| Reverse((x.abs_diff(robot.0), y.abs_diff(robot.1))))
        .map(|b| {
            let n = grid.get_neighbor(b, dir).unwrap();
            let c = grid.get_opt(b).unwrap();
            (n, c)
        });

    for (n, c) in boxes {
        result.insert(n.coords, *c.data);
        result.insert(c.coords, Tile::Space);
    }

    result
}

fn execute_instructions(mut grid: Grid<Tile>, directions: &[CardinalDirection]) -> Grid<Tile> {
    let mut robot = grid
        .iter()
        .find_map(|x| (x.data == &Tile::Robot).then_some(x.coords))
        .unwrap();

    for &d in directions {
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
            Tile::SmallBox | Tile::BoxLeft | Tile::BoxRight => {
                let moves = simulate_moves_wide(&grid, robot, d);
                if moves.is_empty() {
                    continue;
                }

                for (free_space, tile) in moves {
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
    grid
}

fn problem1(input: &Input) -> usize {
    let (grid, directions) = input.clone();
    let grid = execute_instructions(grid, &directions);

    // get the gps of all the boxes
    grid.iter()
        .filter(|x| x.data == &Tile::SmallBox)
        .map(|x| x.coords.1 * 100 + x.coords.0)
        .sum()
}

fn problem2(input: &Input) -> usize {
    let (grid, directions) = input.clone();
    let grid = make_wide(grid);
    let grid = execute_instructions(grid, &directions);

    // get the gps of all the boxes
    grid.iter()
        .filter(|x| x.data == &Tile::BoxLeft)
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
