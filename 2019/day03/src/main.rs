use common::extensions::PointExt;
use std::collections::{BTreeMap, BTreeSet};

use nom::{
    bytes::complete::tag,
    character::{
        complete::{newline, u32},
        streaming::one_of,
    },
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
type Input = (Vec<(Direction, u32)>, Vec<(Direction, u32)>);

fn parse(input: &str) -> Input {
    let wire = |s| {
        separated_list1(
            tag(","),
            map(tuple((one_of("UDLR"), u32)), |(dir, amount)| match dir {
                'U' => (Direction::Up, amount),
                'D' => (Direction::Down, amount),
                'L' => (Direction::Left, amount),
                'R' => (Direction::Right, amount),
                _ => unreachable!(),
            }),
        )(s)
    };
    let result: IResult<&str, Input> = separated_pair(wire, newline, wire)(input);

    result.unwrap().1
}

fn generate_points(directions: &[(Direction, u32)]) -> BTreeMap<(i64, i64), i64> {
    let mut points = BTreeMap::new();
    let (mut x, mut y) = (0i64, 0i64);
    let mut current = 0;
    for (direction, d) in directions {
        for _n in 0..*d {
            current += 1;
            (x, y) = match direction {
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
                Direction::Right => (x + 1, y),
            };
            points.entry((x, y)).or_insert(current);
        }
    }

    points
}

fn get_min_by<F>(input: &Input, f: F) -> i64
where
    F: Fn(&&(i64, i64), i64, i64) -> i64,
{
    let points1 = generate_points(&input.0);
    let points2 = generate_points(&input.1);

    let p1: BTreeSet<_> = points1.keys().collect();
    let p2: BTreeSet<_> = points2.keys().collect();

    p1.intersection(&p2)
        .map(|point| f(point, points1[point], points2[point]))
        .min()
        .unwrap()
}

fn problem1(input: &Input) -> i64 {
    get_min_by(input, |x, _v1, _v2| x.manhattan(&(0, 0)))
}

fn problem2(input: &Input) -> i64 {
    get_min_by(input, |_x, v1, v2| v1 + v2)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 6)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 30)
    }
}
