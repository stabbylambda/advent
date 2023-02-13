use std::collections::HashSet;

use nalgebra::MatrixXx3;
use nom::{
    bytes::complete::tag,
    character::complete::{i32 as nom_i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

pub mod kabsch;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Scanner>;

struct Scanner {
    id: i32,
    beacons: MatrixXx3<i32>,
}
impl Scanner {
    fn fingerprint(&self) -> HashSet<i32> {
        self.beacons
            .row_iter()
            .flat_map(|b1| {
                self.beacons
                    .row_iter()
                    // this is just the manhattan distance between two points
                    .map(move |b2| (b1 - b2).abs().sum())
            })
            .collect()
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        tag("\n\n"),
        map(
            separated_pair(
                delimited(tag("--- scanner "), nom_i32, tag(" ---")),
                newline,
                separated_list1(
                    newline,
                    map(
                        tuple((
                            terminated(nom_i32, tag(",")),
                            terminated(nom_i32, tag(",")),
                            nom_i32,
                        )),
                        |(x, y, z)| vec![x, y, z],
                    ),
                ),
            ),
            |(id, beacons)| {
                let count = beacons.len();
                let points = beacons.into_iter().flatten();
                let beacons = MatrixXx3::from_row_iterator(count, points);
                Scanner { id, beacons }
            },
        ),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    // 12 choose 2 gets us to 66 so we can validate that a point cloud matches another point cloud
    let min_match = 66;

    // fingerprint all the scanner point clouds
    let x: Vec<(&Scanner, HashSet<i32>)> = input.iter().map(|s| (s, s.fingerprint())).collect();
    let mut seen: HashSet<i32> = HashSet::new();
    let mut queue = vec![x.first().unwrap()];

    while let Some((s1, f1)) = queue.pop() {
        println!("Checking sensor {}", s1.id);
        seen.insert(s1.id);

        // Rotation3::from_euler_angles(roll, pitch, yaw);

        for e @ (s2, f2) in &x {
            let matches = !seen.contains(&s2.id)
                && s1.id != s2.id
                && f1.intersection(f2).count() >= min_match;

            if matches {
                println!("  Sensor {} neighbors sensor {}", s1.id, s2.id);
                queue.push(e);
            }
        }
    }

    todo!()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    #[ignore]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 0)
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
