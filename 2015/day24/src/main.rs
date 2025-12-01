use itertools::Itertools;
use nom::{
    character::complete::{newline, u64 as nom_u64},
    multi::separated_list1,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<u64>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, nom_u64).parse(input);

    result.unwrap().1
}

fn problem(input: &Input, groups: u64) -> u64 {
    let expected: u64 = input.iter().sum::<u64>() / groups;

    (0..input.len())
        .find_map(|n| {
            let combos = input
                .iter()
                .combinations(n)
                .filter(|group1| group1.iter().map(|x| x.to_owned()).sum::<u64>() == expected)
                .collect_vec();
            (!combos.is_empty()).then_some(combos)
        })
        .unwrap()
        .iter()
        .map(|v| v.iter().fold(1, |acc, x| acc * **x))
        .min()
        .unwrap()
}

fn problem1(input: &Input) -> u64 {
    problem(input, 3)
}

fn problem2(input: &Input) -> u64 {
    problem(input, 4)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 11846773891)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 80393059)
    }
}
