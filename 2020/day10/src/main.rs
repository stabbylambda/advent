use std::collections::HashMap;

use nom::{
    character::complete::{i64, newline},
    multi::separated_list1,
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<i64>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, i64).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> i64 {
    let mut input = input.clone();
    input.sort();

    let (one, three) = input.windows(2).fold((1, 1), |(mut one, mut three), pair| {
        one += (pair[0].abs_diff(pair[1]) == 1) as i64;
        three += (pair[0].abs_diff(pair[1]) == 3) as i64;

        (one, three)
    });

    one * three
}

fn problem2(input: &Input) -> i64 {
    let mut adapters = input.clone();
    adapters.sort();
    // add the device itself
    adapters.push(adapters.iter().max().unwrap() + 3);

    // prime the cache with 1
    let mut cache: HashMap<i64, i64> = HashMap::new();
    cache.insert(0, 1);

    // do some dynamic programming
    for adapter in adapters {
        // we always get all the possible combinations from this adapter with a max of three jolts back
        let a = cache.get(&(adapter - 1)).copied().unwrap_or(0);
        let b = cache.get(&(adapter - 2)).copied().unwrap_or(0);
        let c = cache.get(&(adapter - 3)).copied().unwrap_or(0);

        cache.insert(adapter, a + b + c);
    }

    cache.values().max().copied().unwrap()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 220)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 19208)
    }
}
