use std::collections::HashMap;

use common::{answer, digits, nom::usize, read_input};
use nom::{bytes::complete::tag, sequence::separated_pair, IResult, Parser};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = (usize, usize);

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_pair(usize, tag("-"), usize).parse(input);

    result.unwrap().1
}

fn has_pair(n: &[u8]) -> bool {
    n.windows(2).any(|x| x[0] == x[1])
}

fn increasing(n: &[u8]) -> bool {
    n.windows(2).all(|x| x[1] >= x[0])
}

fn has_exact_pair(n: &[u8]) -> bool {
    let mut map: HashMap<u8, u8> = HashMap::new();
    for x in n {
        let e = map.entry(*x).or_default();
        *e += 1;
    }

    map.values().any(|x| *x == 2)
}

fn problem1(input: &Input) -> usize {
    (input.0..=input.1)
        .map(digits)
        .filter(|x| increasing(x))
        .filter(|x| has_pair(x))
        .count()
}

fn problem2(input: &Input) -> usize {
    (input.0..=input.1)
        .map(digits)
        .filter(|x| increasing(x))
        .filter(|x| has_exact_pair(x))
        .count()
}

#[cfg(test)]
mod test {
    use common::digits;

    use crate::{has_exact_pair, parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 1610)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1104)
    }

    #[test]
    fn has_exact_pair_test() {
        assert!(has_exact_pair(&digits(112233)));
        assert!(!has_exact_pair(&digits(123444)));
        assert!(has_exact_pair(&digits(111122)));
    }
}
