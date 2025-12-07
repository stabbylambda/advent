use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
};

use ansi_term::Colour::{Green, Red};

use common::{answer, read_input};

fn main() {
    let input = read_input!();
    let input = input
        .lines()
        .map(|s| s.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let map = Map::new(input);

    answer!(map.risk());
    answer!(map.all_basins());
}

struct Map {
    points: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

type Coord = (usize, usize);

impl Map {
    fn new(points: Vec<Vec<u32>>) -> Map {
        let height = points.len();
        let width = points[0].len();
        Map {
            points,
            height,
            width,
        }
    }

    fn get(&self, (x, y): Coord) -> u32 {
        self.points[y][x]
    }

    fn neighbors(&self, (x, y): Coord) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        if y != 0 {
            v.push((x, y - 1));
        }

        if x != 0 {
            v.push((x - 1, y));
        }

        if y != self.height - 1 {
            v.push((x, y + 1));
        }

        if x != self.width - 1 {
            v.push((x + 1, y));
        }
        v
    }

    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = (x, y);
                let v = self.get(coord);
                let lowest = self.neighbors(coord).iter().all(|c| v < self.get(*c));

                // a 9 is the max, so it can never be 9
                if v == 9 {
                    print!("{}", Red.paint(v.to_string()));
                } else if lowest {
                    print!("{}", Green.paint(v.to_string()));
                    result.push((x, y));
                } else {
                    print!("{}", v);
                }
            }
            println!()
        }
        result
    }
    fn risk(&self) -> u32 {
        let points = self.low_points();
        points.iter().fold(0, |acc, c| acc + self.get(*c) + 1)
    }

    fn basin_size(&self, low_point: Coord) -> usize {
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(low_point);

        while let Some(n) = queue.pop_front() {
            if visited.contains(&n) {
                continue;
            }

            visited.insert(n);

            let neighbors = self.neighbors(n);
            for neighbor in neighbors {
                if self.get(neighbor) != 9 {
                    queue.push_back(neighbor);
                }
            }
        }

        visited.len()
    }

    fn all_basins(&self) -> u32 {
        let points = self.low_points();
        let mut basins: Vec<u32> = points.iter().map(|p| self.basin_size(*p) as u32).collect();
        basins.sort_by_key(|x| Reverse(*x));
        basins.iter().take(3).product()
    }
}

#[test]
fn first() {
    let input = include_str!("../test.txt");
    let input = input
        .lines()
        .map(|s| s.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let map = Map::new(input);

    let risk = map.risk();
    assert_eq!(15, risk)
}

#[test]
fn basins() {
    let input = include_str!("../test.txt");
    let input = input
        .lines()
        .map(|s| s.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let map = Map::new(input);

    let lows = [(1, 0, 3), (9, 0, 9), (2, 2, 14), (6, 4, 9)];
    for (x, y, expected) in lows {
        assert_eq!(expected, map.basin_size((x, y)));
    }
    assert_eq!(map.all_basins(), 1134);
}
