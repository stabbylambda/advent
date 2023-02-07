use std::collections::HashMap;

use common::get_raw_input;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);
    let reference: HashMap<Thing, u32> = REFERENCE.into_iter().collect();

    let score = problem1(&input, &reference);
    println!("problem 1 score: {score}");

    let score = problem2(&input, &reference);
    println!("problem 2 score: {score}");
}

const REFERENCE: [(Thing, u32); 10] = [
    (Thing::Children, 3),
    (Thing::Cats, 7),
    (Thing::Samoyeds, 2),
    (Thing::Pomeranians, 3),
    (Thing::Akitas, 0),
    (Thing::Vizslas, 0),
    (Thing::Goldfish, 5),
    (Thing::Trees, 3),
    (Thing::Cars, 2),
    (Thing::Perfumes, 1),
];

#[derive(Debug)]
struct Aunt {
    number: u32,
    stuff: HashMap<Thing, u32>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Thing {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl From<&str> for Thing {
    fn from(name: &str) -> Self {
        match name {
            "children" => Thing::Children,
            "cats" => Thing::Cats,
            "samoyeds" => Thing::Samoyeds,
            "pomeranians" => Thing::Pomeranians,
            "akitas" => Thing::Akitas,
            "vizslas" => Thing::Vizslas,
            "goldfish" => Thing::Goldfish,
            "trees" => Thing::Trees,
            "cars" => Thing::Cars,
            "perfumes" => Thing::Perfumes,
            _ => unreachable!(),
        }
    }
}

type Input = Vec<Aunt>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            tuple((
                delimited(tag("Sue "), nom_u32, tag(": ")),
                map(
                    separated_list1(
                        tag(", "),
                        separated_pair(map(alpha0, |x: &str| x.into()), tag(": "), nom_u32),
                    ),
                    |things| things.into_iter().collect(),
                ),
            )),
            |(number, stuff)| Aunt { number, stuff },
        ),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input, reference: &HashMap<Thing, u32>) -> u32 {
    input
        .iter()
        .find_map(|aunt| {
            aunt.stuff
                .iter()
                .all(|(thing, count)| *count == reference[thing])
                .then_some(aunt.number)
        })
        .unwrap()
}

fn problem2(input: &Input, reference: &HashMap<Thing, u32>) -> u32 {
    input
        .iter()
        .find_map(|aunt| {
            aunt.stuff
                .iter()
                .all(|(thing, count)| match thing {
                    Thing::Cats => *count > reference[&Thing::Cats],
                    Thing::Trees => *count > reference[&Thing::Trees],
                    Thing::Pomeranians => *count < reference[&Thing::Pomeranians],
                    Thing::Goldfish => *count < reference[&Thing::Goldfish],
                    thing => *count == reference[thing],
                })
                .then_some(aunt.number)
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2, Thing, REFERENCE};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);

        let reference: HashMap<Thing, u32> = REFERENCE.into_iter().collect();
        let result = problem1(&input, &reference);
        assert_eq!(result, 103)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let reference: HashMap<Thing, u32> = REFERENCE.into_iter().collect();
        let result = problem2(&input, &reference);
        assert_eq!(result, 405)
    }
}
