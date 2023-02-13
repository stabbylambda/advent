use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
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

#[derive(Debug)]
struct Input<'a> {
    molecule: &'a str,
    mappings: Vec<(&'a str, &'a str)>,
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_pair(
            separated_list1(newline, separated_pair(alpha1, tag(" => "), alpha1)),
            tag("\n\n"),
            alpha1,
        ),
        |(mappings, molecule)| Input { mappings, molecule },
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let mut set: HashSet<String> = HashSet::new();
    for (key, value) in input.mappings.iter() {
        for (x, _s) in input.molecule.match_indices(key) {
            let before = &input.molecule[0..x];
            let replacement = value;
            let after = &input.molecule[(x + key.len())..];
            let s = format!("{}{}{}", before, replacement, after);
            set.insert(s);
        }
    }

    set.len()
}

fn problem2(input: &Input) -> u32 {
    let mut n = 0;
    let mut s = input.molecule.to_string();

    // reverse the productions so they're reductions
    let mut reductions: Vec<(&str, &str)> = input.mappings.iter().map(|(a, b)| (*b, *a)).collect();
    // reverse sort them by length of production so we can be greedy
    reductions.sort_by(|(a_from, _a_to), (b_from, _b_to)| b_from.len().cmp(&a_from.len()));

    while s != "e" {
        // go through the reductions at each step and execute the first one that matches
        for (from, to) in &reductions[..] {
            if s.contains(from) {
                s = s.replacen(from, to, 1);
                n += 1;
                break;
            }
        }
    }
    n
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 7)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 6)
    }
}
