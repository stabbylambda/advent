use common::math::lcm;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap},
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

enum Tile {
    Wall,
    Empty,
    Blizzard(Direction),
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Point = (i64, i64, i64);
type Input = Vec<Vec<Tile>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        many1(alt((
            map(char('#'), |_| Tile::Wall),
            map(char('^'), |_| Tile::Blizzard(Direction::Up)),
            map(char('v'), |_| Tile::Blizzard(Direction::Down)),
            map(char('<'), |_| Tile::Blizzard(Direction::Left)),
            map(char('>'), |_| Tile::Blizzard(Direction::Right)),
            map(char('.'), |_| Tile::Empty),
        ))),
    )(input);
    result.unwrap().1
}

struct Valley {
    points: BTreeSet<Point>,
    height: i64,
    width: i64,
    cycle: i64,
}

impl Valley {
    fn get_start(&self) -> (i64, i64) {
        (1, 0)
    }

    fn get_end(&self) -> (i64, i64) {
        (self.width - 2, self.height - 1)
    }

    fn is_free(&self, p @ (x, y, _z): &(i64, i64, i64)) -> bool {
        *x >= 0 && *y >= 0 && !self.points.contains(p)
    }

    fn new(input: &Input) -> Valley {
        let height = input.len() as i64;
        let width = input.first().unwrap().len() as i64;

        // the blizzards are periodic, so we only need to generate up to a certain size
        let cycle = lcm(height, width);

        let mut points: BTreeSet<Point> = BTreeSet::new();
        // we're going to create a 3D map where the third dimension is time

        for t in 0..cycle {
            // the start and end are always available
            points.insert((1, -1, t));
            points.insert((width - 2, height, t));

            for (y, row) in input.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let x = x as i64;
                    let y = y as i64;
                    match cell {
                        // walls are always walls
                        Tile::Wall => points.insert((x, y, t)),

                        // insert the positions of where the blizzards will be at all points in time
                        Tile::Blizzard(Direction::Up) => {
                            points.insert((x, (y - 1 - t).rem_euclid(height - 2) + 1, t))
                        }
                        Tile::Blizzard(Direction::Down) => {
                            points.insert((x, (y - 1 + t).rem_euclid(height - 2) + 1, t))
                        }
                        Tile::Blizzard(Direction::Left) => {
                            points.insert(((x - 1 - t).rem_euclid(width - 2) + 1, y, t))
                        }
                        Tile::Blizzard(Direction::Right) => {
                            points.insert(((x - 1 + t).rem_euclid(width - 2) + 1, y, t))
                        }

                        // we don't really need to do anything for the empty tiles
                        Tile::Empty => false,
                    };
                }
            }
        }

        Valley {
            points,
            height,
            width,
            cycle,
        }
    }

    fn travel_time(&self, start: (i64, i64, i64), (tx, ty): (i64, i64)) -> Option<i64> {
        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::new();

        distances.insert(start, 0);
        queue.push((Reverse(0), start));

        while let Some((Reverse(distance), (x, y, t))) = queue.pop() {
            // have we reached the end? if so, our third dimension is the travel time here
            if (x, y) == (tx, ty) {
                return Some(distance);
            }

            // get our neighbors in spacetime
            let next_time = (t + 1) % self.cycle;
            [
                (x, y, next_time),     // wait
                (x - 1, y, next_time), // west
                (x + 1, y, next_time), // east
                (x, y - 1, next_time), // north
                (x, y + 1, next_time), // south
            ]
            .into_iter()
            .filter(|x| self.is_free(x))
            .for_each(|neighbor| {
                let neighbor_distance = distances.entry(neighbor).or_insert(i64::MAX);

                if *neighbor_distance > distance + 1 {
                    *neighbor_distance = distance + 1;
                    queue.push((Reverse(*neighbor_distance), neighbor));
                }
            });
        }
        None
    }
}

fn problem1(input: &Input) -> i64 {
    let valley = Valley::new(input);
    valley.travel_time((1, 0, 0), valley.get_end()).unwrap()
}

fn problem2(input: &Input) -> i64 {
    let valley = Valley::new(input);
    let start = valley.get_start();
    let end = valley.get_end();

    let t1 = valley
        .travel_time((start.0, start.1, 0), valley.get_end())
        .unwrap();
    let t2 = valley
        .travel_time((end.0, end.1, t1), valley.get_start())
        .unwrap();
    let t3 = valley
        .travel_time((start.0, start.1, t1 + t2), valley.get_end())
        .unwrap();

    t1 + t2 + t3
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 18)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 54)
    }
}
