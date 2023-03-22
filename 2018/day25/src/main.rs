use std::collections::BTreeSet;

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");
}

type Input = Vec<Point>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point((i32, i32, i32, i32));

impl Point {
    fn manhattan(&self, other: &Point) -> u32 {
        let (sx, sy, sz, sa) = self.0;
        let (ox, oy, oz, oa) = other.0;

        sx.abs_diff(ox) + sy.abs_diff(oy) + sz.abs_diff(oz) + sa.abs_diff(oa)
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(separated_list1(tag(","), i32), |x| {
            Point((x[0], x[1], x[2], x[3]))
        }),
    )(input);

    result.unwrap().1
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Constellation {
    points: BTreeSet<Point>,
}

impl Constellation {
    fn new(point: Point) -> Constellation {
        let mut points = BTreeSet::new();
        points.insert(point);

        Constellation { points }
    }

    fn distance_from(&self, other: &Constellation) -> u32 {
        let mut min_dist = u32::MAX;

        for point in &self.points {
            for other in &other.points {
                min_dist = point.manhattan(other).min(min_dist);
            }
        }

        min_dist
    }

    fn merge(&mut self, other: &Constellation) {
        self.points.extend(other.points.clone())
    }
}

fn problem1(input: &Input) -> usize {
    let mut constellations: Vec<Constellation> =
        input.iter().map(|x| Constellation::new(*x)).collect();

    loop {
        let merged: Vec<Constellation> = constellations.iter().fold(vec![], |mut result, c1| {
            let can_merge = result
                .iter_mut()
                .find(|constellation| constellation.distance_from(c1) <= 3);

            if let Some(constellation) = can_merge {
                constellation.merge(c1);
            } else {
                result.push(c1.clone())
            }

            result
        });

        if merged.len() == constellations.len() {
            break;
        }

        constellations = merged;
    }

    constellations.len()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let cases = [
            (include_str!("../test1.txt"), 2),
            (include_str!("../test2.txt"), 4),
            (include_str!("../test3.txt"), 3),
            (include_str!("../test4.txt"), 8),
        ];
        for (input, expected) in cases {
            let input = parse(input);
            let result = problem1(&input);
            assert_eq!(result, expected)
        }
    }
}
