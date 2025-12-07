use itertools::Itertools;
use std::{
    collections::{BTreeSet, VecDeque},
    ops::{Add, Sub},
};

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use common::{answer, read_input};

fn main() {
    let input = read_input!();
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    answer!(answer1);
    answer!(answer2);
}

type Input = Vec<Scanner>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i32, i32, i32);

impl Point {
    fn rotate(&self, rot: u8) -> Self {
        let Point(x, y, z) = *self;
        match rot {
            0 => Point(x, y, z),
            1 => Point(x, z, -y),
            2 => Point(x, -y, -z),
            3 => Point(x, -z, y),
            4 => Point(y, x, -z),
            5 => Point(y, z, x),
            6 => Point(y, -x, z),
            7 => Point(y, -z, -x),
            8 => Point(z, x, y),
            9 => Point(z, y, -x),
            10 => Point(z, -x, -y),
            11 => Point(z, -y, x),
            12 => Point(-x, y, -z),
            13 => Point(-x, z, y),
            14 => Point(-x, -y, z),
            15 => Point(-x, -z, -y),
            16 => Point(-y, x, z),
            17 => Point(-y, z, -x),
            18 => Point(-y, -x, -z),
            19 => Point(-y, -z, x),
            20 => Point(-z, x, -y),
            21 => Point(-z, y, x),
            22 => Point(-z, -x, y),
            23 => Point(-z, -y, -x),
            _ => unreachable!(),
        }
    }

    fn manhattan(&self, other: &Self) -> u32 {
        let Point(x1, y1, z1) = self;
        let Point(x2, y2, z2) = *other;

        x1.abs_diff(x2) + y1.abs_diff(y2) + z1.abs_diff(z2)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        let Point(x1, y1, z1) = self;
        let Point(x2, y2, z2) = rhs;

        Point(x1 - x2, y1 - y2, z1 - z2)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let Point(x, y, z) = self;
        let Point(ox, oy, oz) = rhs;
        Point(x + ox, y + oy, z + oz)
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    id: i32,
    beacons: Vec<Point>,
}
impl Scanner {
    fn new(id: i32, beacons: Vec<Point>) -> Self {
        Scanner { id, beacons }
    }

    fn rotate(&self, rot: u8) -> Self {
        Self {
            id: self.id,
            beacons: self.beacons.iter().map(|x| x.rotate(rot)).collect(),
        }
    }

    fn translate(&self, t: Point) -> Self {
        Self {
            id: self.id,
            beacons: self.beacons.iter().map(|x| *x + t).collect(),
        }
    }

    fn merge(&mut self, other: &Scanner) {
        for beacon in &other.beacons {
            if !self.beacons.contains(beacon) {
                self.beacons.push(*beacon);
            }
        }
    }

    fn get_translated_scanner(&self, other: &Scanner) -> Option<(Scanner, Point)> {
        self.beacons
            .iter()
            .cartesian_product(&other.beacons)
            .find_map(|(p1, p2)| {
                let offset = *p1 - *p2;

                let other_beacons = other.beacons.iter().map(|p| *p + offset);

                let num_matching = self
                    .beacons
                    .iter()
                    .filter(|&point| other_beacons.clone().any(|p| p == *point))
                    .count();

                (num_matching >= 12).then(|| (other.translate(offset), offset))
            })
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        tag("\n\n"),
        map(
            separated_pair(
                delimited(tag("--- scanner "), i32, tag(" ---")),
                newline,
                separated_list1(
                    newline,
                    map(separated_list1(tag(","), i32), |x| Point(x[0], x[1], x[2])),
                ),
            ),
            |(id, beacons)| Scanner::new(id, beacons),
        ),
    ).parse(input);

    result.unwrap().1
}

fn merge_scanners(input: &Input) -> (Scanner, Vec<Point>) {
    let mut prime = input.first().unwrap().clone();
    let mut coords = vec![Point(0, 0, 0)];

    // precompute all the rotations
    let mut scanners: VecDeque<Scanner> = input
        .clone()
        .into_iter()
        .skip(1)
        .flat_map(|x| (0..24).map(move |r| x.rotate(r)))
        .collect();

    let mut merged: BTreeSet<i32> = BTreeSet::new();

    while let Some(s1) = scanners.pop_front() {
        // if we already merged one of these (because we precomputed all the rotations) we can just skip this one
        if merged.contains(&s1.id) {
            continue;
        }

        // get the translation vector
        if let Some((s1, offset)) = prime.get_translated_scanner(&s1) {
            println!("Merging {} into prime with offset {offset:?}", s1.id);

            // merge all the stuff together
            prime.merge(&s1);
            merged.insert(s1.id);

            // keep track of where the sensor is
            coords.push(offset);
        } else {
            scanners.push_back(s1);
        }
    }

    (prime, coords)
}

fn problem(input: &Input) -> (usize, u32) {
    let (result, scanner_coords) = merge_scanners(input);

    let answer1 = result.beacons.len();

    let answer2 = scanner_coords
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1.manhattan(p2))
        .max()
        .unwrap();

    (answer1, answer2)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (result1, result2) = problem(&input);
        assert_eq!(result1, 79);
        assert_eq!(result2, 3621);
    }
}
