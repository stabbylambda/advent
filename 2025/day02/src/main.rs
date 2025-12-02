use common::digits;
use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list1,
    sequence::separated_pair, IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<(u64, u64)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(tag(","), separated_pair(u64, tag("-"), u64)).parse(input);

    result.unwrap().1
}

fn problem1(x: &Input) -> u64 {
    fn invalid(x: u64) -> Option<u64> {
        let d = digits(x as usize);
        let (fst, snd) = d.split_at(d.len() / 2);
        fst.iter().eq(snd.iter()).then_some(x)
    }

    x.iter()
        .flat_map(|(a, b)| *a..=*b)
        .filter_map(invalid)
        .sum()
}

fn problem2(x: &Input) -> u64 {
    fn invalid(x: u64) -> Option<u64> {
        let d = digits(x as usize);

        let invalid = (1..=(d.len() / 2)).rev().find(|n| {
            let mut chunks = d.chunks(*n);
            let first = chunks.next().unwrap();
            chunks.all(|x| first.eq(x))
        });

        invalid.map(|_| x)
    }

    x.iter()
        .flat_map(|(a, b)| *a..=*b)
        .filter_map(invalid)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 4174379265)
    }
}
