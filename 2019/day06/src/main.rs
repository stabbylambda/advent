use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::newline,
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
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

type Input<'a> = HashMap<&'a str, &'a str>;

fn parse(input: &str) -> Input<'_> {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            separated_pair(take_until(")"), tag(")"), take_until("\n")),
        ),
        |input| input.iter().map(|(k, v)| (*v, *k)).collect(),
    ).parse(input);

    result.unwrap().1
}

fn orbit_path<'a>(from: &'a str, map: &HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut current = from;
    let mut ancestors = vec![];

    while current != "COM" {
        current = map[current];
        ancestors.push(current);
    }

    ancestors
}

fn problem1(input: &Input) -> usize {
    let all: HashSet<&str> = input.iter().flat_map(|(k, v)| vec![*k, *v]).collect();

    let mut orbits = 0;
    for x in all {
        orbits += orbit_path(x, input).len();
    }

    orbits
}

fn problem2(input: &Input) -> usize {
    let you = orbit_path("YOU", input);
    let san = orbit_path("SAN", input);

    // find the first common ancestor on the path
    let you_depth = you.iter().position(|x| san.contains(x)).unwrap();
    let san_depth = san.iter().position(|x| you.contains(x)).unwrap();

    you_depth + san_depth
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 42)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 4)
    }
}
