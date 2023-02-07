use itertools::Itertools;
use nom::{
    character::complete::{newline, u32 as nom_u32},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input, 150);
    println!("problem 1 score: {score}");

    let score = problem2(&input, 150);
    println!("problem 2 score: {score}");
}

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, nom_u32)(input);

    result.unwrap().1
}

fn problem1(input: &Input, total: u32) -> usize {
    input
        .iter()
        .powerset()
        .filter(|x| x.iter().map(|y| **y).sum::<u32>() == total)
        .count()
}

fn problem2(input: &Input, total: u32) -> usize {
    input
        .iter()
        .powerset()
        .filter(|x| x.iter().map(|y| **y).sum::<u32>() == total)
        .min_set_by(|x, y| x.len().cmp(&y.len()))
        .len()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input, 25);
        assert_eq!(result, 4)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input, 25);
        assert_eq!(result, 3)
    }
}
