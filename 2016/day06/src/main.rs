use std::cmp::Ordering;

use common::get_raw_input;
use itertools::Itertools;
use nom::{
    character::complete::{alpha1, newline},
    combinator::map,
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(separated_list1(newline, alpha1), |input: Vec<&str>| {
        let width = input[0].len();
        // transpose the nested vec so we can examine each char index
        let mut i_t: Vec<Vec<char>> = vec![vec![]; width];
        (0..width).for_each(|x| {
            (0..input.len()).for_each(|y| i_t[x].push(input[y].chars().nth(x).unwrap()));
        });

        i_t
    })(input);

    result.unwrap().1
}

fn frequency<F>(input: &Input, cmp: F) -> String
where
    F: Fn(usize, usize) -> Ordering + Copy,
{
    input
        .iter()
        .map(|p| {
            p.iter()
                .counts_by(|c| c)
                .into_iter()
                .sorted_by(|x, y| cmp(x.1, y.1))
                .map(|x| x.0)
                .next()
                .unwrap()
        })
        .collect()
}

fn problem1(input: &Input) -> String {
    frequency(input, |x, y| y.cmp(&x))
}

fn problem2(input: &Input) -> String {
    frequency(input, |x, y| x.cmp(&y))
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, "easter")
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, "advent")
    }
}
