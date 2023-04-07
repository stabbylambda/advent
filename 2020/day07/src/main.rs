use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
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

type Input<'a> = HashMap<&'a str, Vec<(u32, &'a str)>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            separated_pair(
                take_until(" bag"),
                tag(" bags contain "),
                terminated(
                    alt((
                        map(tag("no other bags"), |_| vec![]),
                        separated_list1(
                            tag(", "),
                            separated_pair(
                                u32,
                                tag(" "),
                                terminated(take_until(" bag"), alt((tag(" bags"), tag(" bag")))),
                            ),
                        ),
                    )),
                    tag("."),
                ),
            ),
        ),
        |input| {
            let map: HashMap<&str, Vec<(u32, &str)>> =
                input.iter().map(|x| (x.0, x.1.clone())).collect();
            map
        },
    )(input);

    result.unwrap().1
}

fn contains_shiny_gold(map: &Input, bag: &str) -> bool {
    if bag == "shiny gold" {
        return true;
    }

    let inner: Vec<_> = map.get(bag).cloned().unwrap_or(vec![]);
    inner.iter().any(|x| contains_shiny_gold(map, x.1))
}

fn problem1(input: &Input) -> usize {
    input
        .keys()
        .filter(|outer| **outer != "shiny gold" && contains_shiny_gold(input, outer))
        .count()
}

fn bag_sum(map: &Input, amount: u32, bag: &str) -> u32 {
    map.get(bag)
        .map(|inner| {
            inner
                .iter()
                .fold(amount, |acc, (inner_amount, inner_type)| {
                    acc + amount * bag_sum(map, *inner_amount, inner_type)
                })
        })
        .unwrap_or(0)
}

fn problem2(input: &Input) -> u32 {
    // minus one because we don't count the shiny gold bag
    bag_sum(input, 1, "shiny gold") - 1
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 4)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 32)
    }

    #[test]
    fn second_bigger() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 126)
    }
}
