use itertools::Itertools;
use std::collections::{BTreeSet, VecDeque};

use nalgebra::MatrixXx3;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
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

    fn translation_vector(&self, other: &Self) -> (i32, i32, i32) {
        let Point(x1, y1, z1) = self;
        let Point(x2, y2, z2) = *other;

        (x1 - x2, y1 - y2, z1 - z2)
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    id: i32,
    beacons: BTreeSet<Point>,
    fingerprint: BTreeSet<u32>,
}
impl Scanner {
    fn new(id: i32, beacons: Vec<Point>) -> Self {
        let beacons = beacons.into_iter().collect();
        let fingerprint = Self::fingerprint(&beacons);
        Scanner {
            id,
            beacons,
            fingerprint,
        }
    }

    // 12 choose 2 gets us to 66 so we can validate that a point cloud matches another point cloud
    const MIN_MATCH: usize = 66;

    fn fingerprint_match(&self, other: &Scanner) -> bool {
        self.fingerprint.intersection(&other.fingerprint).count() >= Self::MIN_MATCH
    }

    fn fingerprint(beacons: &BTreeSet<Point>) -> BTreeSet<u32> {
        // Get the manhattan distances between all pairs of two points
        beacons
            .iter()
            .tuple_combinations()
            .map(|(b1, b2)| b1.manhattan(b2))
            .collect()
    }

    fn rotate(&self, rot: u8) -> Self {
        Self {
            id: self.id,
            beacons: self.beacons.iter().map(|x| x.rotate(rot)).collect(),
            fingerprint: self.fingerprint.clone(),
        }
    }

    fn translate(&self, (dx, dy, dz): (i32, i32, i32)) -> Self {
        Self {
            id: self.id,
            beacons: self
                .beacons
                .iter()
                .map(|x| {
                    let Point(x, y, z) = *x;
                    Point(x + dx, y + dy, z + dz)
                })
                .collect(),
            fingerprint: self.fingerprint.clone(),
        }
    }

    fn merge(&mut self, other: &Scanner) {
        self.beacons.extend(other.beacons.iter().copied());
        self.fingerprint = self
            .fingerprint
            .union(&other.fingerprint)
            .copied()
            .collect();
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
    )(input);

    result.unwrap().1
}

fn merge_scanners(input: &Input) -> Scanner {
    let mut scanners: VecDeque<Scanner> = input.clone().into_iter().collect();
    let mut merged: BTreeSet<i32> = BTreeSet::new();

    while let Some(mut s1) = scanners.pop_front() {
        if merged.contains(&s1.id) {
            continue;
        }

        // if this is the last scanner, we're done
        if scanners.is_empty() {
            return s1;
        }

        println!(
            "Checking sensor {}. Rest: {:?}",
            s1.id,
            scanners.iter().map(|x| x.id).collect::<Vec<i32>>()
        );

        let other_scanners: Vec<&Scanner> = scanners
            .iter()
            .filter(|s2| !merged.contains(&s2.id) && s1.id != s2.id && s1.fingerprint_match(s2))
            .collect();

        let reference_beacons = s1.beacons.clone();

        'merge: for s2 in other_scanners {
            println!("  Sensor {} neighbors sensor {}", s1.id, s2.id);
            for rotation in 0..24 {
                let s2r = s2.rotate(rotation);

                let translation_vectors: BTreeSet<(i32, i32, i32)> = reference_beacons
                    .iter()
                    .cartesian_product(&s2r.beacons)
                    .map(|(p1, p2)| p1.translation_vector(p2))
                    .collect();

                for v in translation_vectors {
                    let s2t = s2r.translate(v);
                    if s2t.beacons.intersection(&s1.beacons).count() >= 12 {
                        println!("   Merged {} into {}", s2t.id, s1.id);
                        s1.merge(&s2t);
                        merged.insert(s2t.id);
                        continue 'merge;
                    }
                }
            }
        }

        println!("Resuling scanner has {} beacons", s1.beacons.len());

        // now that we've merged into this one, push it back on
        scanners.push_back(s1);
    }

    unreachable!()
}

fn problem1(input: &Input) -> usize {
    let result = merge_scanners(input);

    result.beacons.len()
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
        assert_eq!(result, 79)
    }

    #[test]
    #[ignore]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
