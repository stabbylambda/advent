use std::collections::HashSet;

use common::{answer, read_input};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input<'a> = Vec<Vec<&'a str>>;

fn parse(input: &str) -> Input<'_> {
    let result: IResult<&str, Input> =
        separated_list1(newline, separated_list1(tag(" "), alpha1)).parse(input);

    result.unwrap().1
}

fn all_unique(phrase: &&Vec<&str>) -> bool {
    let set: HashSet<_> = HashSet::from_iter(phrase.iter());
    set.len() == phrase.len()
}

fn no_anagrams(phrase: &&Vec<&str>) -> bool {
    !phrase.iter().any(|word1| {
        let mut chars1: Vec<char> = word1.chars().collect();
        chars1.sort();

        phrase.iter().filter(|&word2| word1 != word2).any(|word2| {
            let mut chars2: Vec<char> = word2.chars().collect();
            chars2.sort();

            chars1 == chars2
        })
    })
}

fn problem1(input: &Input) -> usize {
    input.iter().filter(all_unique).count()
}

fn problem2(input: &Input) -> usize {
    input.iter().filter(all_unique).filter(no_anagrams).count()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test1.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 2)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 3)
    }
}
