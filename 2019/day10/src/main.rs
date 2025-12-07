use common::{answer, math::gcd, read_input};
use itertools::Itertools;
use std::{collections::HashSet, f64::consts::PI};

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Asteroids;

#[derive(Debug, Clone, Copy)]
struct Slope(i64, i64);

impl Slope {
    fn get_angle(&self) -> f64 {
        let (x, y) = (self.0 as f64, self.1 as f64);

        let d = y.atan2(x) + PI / 2.0;
        if d < 0.0 {
            2.0 * PI + d
        } else {
            d
        }
    }
}

#[derive(Debug, Clone)]
struct Asteroids {
    asteroids: HashSet<(i64, i64)>,
    lines: Vec<Slope>,
    height: i64,
    width: i64,
}

impl Asteroids {
    fn new(input: Vec<Vec<bool>>) -> Self {
        let height = input.len() as i64;
        let width = input[0].len() as i64;
        let coords = Self::get_coords(input);
        Asteroids {
            asteroids: coords,
            lines: Self::lines(width, height),
            height,
            width,
        }
    }

    fn get_coords(map: Vec<Vec<bool>>) -> HashSet<(i64, i64)> {
        map.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, cell)| cell.then_some((x as i64, y as i64)))
            })
            .collect()
    }

    fn lines(width: i64, height: i64) -> Vec<Slope> {
        let (x, y) = (width - 1, height - 1);

        (-x..x)
            .cartesian_product(-y..y)
            .filter_map(|p| (gcd(p.0, p.1) == 1).then_some(Slope(p.0, p.1)))
            .sorted_by(|p1, p2| p1.get_angle().partial_cmp(&p2.get_angle()).unwrap())
            .collect()
    }

    fn get_asteroid_in_view(
        &self,
        (mut x, mut y): (i64, i64),
        Slope(dx, dy): Slope,
    ) -> Option<(i64, i64)> {
        while x > 0 && x < self.width && y > 0 && y < self.height {
            x += dx;
            y += dy;

            if self.asteroids.contains(&(x, y)) {
                return Some((x, y));
            }
        }

        None
    }

    fn get_station(&self) -> ((i64, i64), usize) {
        self.asteroids
            .iter()
            .map(|asteroid| {
                let hit_count = self
                    .lines
                    .iter()
                    .filter_map(|line| self.get_asteroid_in_view(*asteroid, *line))
                    .count();
                (*asteroid, hit_count)
            })
            .max_by_key(|(_asteroid, hit_count)| *hit_count)
            .unwrap()
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(alt((map(char('#'), |_| true), map(char('.'), |_| false)))),
        ),
        Asteroids::new,
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    input.get_station().1
}

fn problem2(input: &Input) -> i64 {
    let mut input = (*input).clone();
    let mut num_hit = 0;
    let station = input.get_station().0;
    // gotta keep going around
    for line in input.lines.iter().cycle() {
        if let Some(asteroid) = input.get_asteroid_in_view(station, *line) {
            // kill it with the laser
            input.asteroids.remove(&asteroid);
            num_hit += 1;

            if num_hit == 200 {
                return asteroid.0 * 100 + asteroid.1;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 210)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 802)
    }
}
