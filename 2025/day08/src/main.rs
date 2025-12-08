use std::collections::BTreeMap;

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
type Input = Vec<JunctionBox>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(separated_list1(tag(","), i64), |x| (x[0], x[1], x[2])),
    )
    .parse(input);

    result.unwrap().1
}

fn distance((ax, ay, az): JunctionBox, (bx, by, bz): JunctionBox) -> i64 {
    ((ax - bx).pow(2) + (ay - by).pow(2) + (az - bz).pow(2)).isqrt()
}

fn problem1(x: &Input, pairs: usize) -> u64 {
    let mut circuits: BTreeMap<JunctionBox, usize> =
        BTreeMap::from_iter(x.iter().enumerate().map(|(id, x)| (*x, id)));

    let combos = x
        .iter()
        .combinations(2)
        .map(|x| (*x[0], *x[1]))
        .sorted_by_key(|(a, b)| distance(*a, *b))
        .take(pairs);

    for (a, b) in combos {
        let ids = (circuits.get(&a).cloned(), circuits.get(&b).cloned());
        match ids {
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
    let mut circuits: BTreeMap<JunctionBox, usize> =
        BTreeMap::from_iter(x.iter().enumerate().map(|(id, x)| (*x, id)));

    let combos = x
        .iter()
        .combinations(2)
        .map(|x| (*x[0], *x[1]))
        .sorted_by_key(|(a, b)| distance(*a, *b));

    for (a, b) in combos {
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
