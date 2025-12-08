use std::collections::{BTreeMap, BinaryHeap};

use common::{answer, read_input};
use itertools::Itertools;
use nom::{
    bytes::tag,
    character::complete::{i64, newline},
    combinator::map,
    multi::separated_list1,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input, 1000));
    answer!(problem2(&input));
}

type JunctionBox = (i64, i64, i64);
type Input = BTreeMap<JunctionBox, usize>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            map(separated_list1(tag(","), i64), |x| (x[0], x[1], x[2])),
        ),
        |x| BTreeMap::from_iter(x.iter().enumerate().map(|(id, x)| (*x, id))),
    )
    .parse(input);

    result.unwrap().1
}

#[derive(Eq, PartialEq)]
struct Pair(JunctionBox, JunctionBox);
impl Pair {
    fn distance(&self) -> i64 {
        let (ax, ay, az) = self.0;
        let (bx, by, bz) = self.1;
        ((ax - bx).pow(2) + (ay - by).pow(2) + (az - bz).pow(2)).isqrt()
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance().cmp(&self.distance())
    }
}

fn problem1(x: &Input, pairs: usize) -> u64 {
    let mut circuits = x.clone();
    let mut queue =
        BinaryHeap::from_iter(circuits.keys().combinations(2).map(|x| Pair(*x[0], *x[1])));

    let mut count = 0;
    while let Some(Pair(a, b)) = queue.pop() {
        count += 1;
        if count > pairs {
            break;
        }

        match (circuits.get(&a).cloned(), circuits.get(&b).cloned()) {
            (Some(a_id), Some(b_id)) if a_id != b_id => {
                circuits
                    .values_mut()
                    .filter(|x| **x == b_id)
                    .for_each(|x| *x = a_id);
            }
            _ => {}
        }
    }

    let grouped = circuits.values().counts();
    let largest: u64 = grouped
        .values()
        .sorted()
        .rev()
        .take(3)
        .map(|x| *x as u64)
        .product();

    largest
}

fn problem2(x: &Input) -> u64 {
    let mut circuits = x.clone();
    let mut queue =
        BinaryHeap::from_iter(circuits.keys().combinations(2).map(|x| Pair(*x[0], *x[1])));

    while let Some(Pair(a, b)) = queue.pop() {
        match (circuits.get(&a).cloned(), circuits.get(&b).cloned()) {
            (Some(a_id), Some(b_id)) if a_id != b_id => {
                circuits
                    .values_mut()
                    .filter(|x| **x == b_id)
                    .for_each(|x| *x = a_id);

                if circuits.values().unique().count() == 1 {
                    return (a.0 * b.0) as u64;
                }
            }
            _ => {}
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
        let result = problem1(&input, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 25272)
    }
}
