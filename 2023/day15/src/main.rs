use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u8},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input<'a> = Vec<Step<'a>>;

fn parse(input: &str) -> Input<'_> {
    let result: IResult<&str, Input> = separated_list1(
        tag(","),
        alt((
            map(separated_pair(alpha1, tag("="), u8), |(label, value)| {
                Step::Insert(label, value)
            }),
            map(terminated(alpha1, tag("-")), Step::Remove),
        )),
    ).parse(input);

    result.unwrap().1
}

enum Step<'a> {
    Insert(&'a str, u8),
    Remove(&'a str),
}

impl<'a> Step<'a> {
    /** Hash just the label, for part 2 */
    fn hash_label(&self) -> u8 {
        let v = match self {
            Step::Insert(label, _) => label,
            Step::Remove(label) => label,
        };

        hash(v)
    }

    /** Hash the whole step, for part 1 */
    fn hash(&self) -> u8 {
        let v = match self {
            Step::Insert(label, value) => format!("{label}={value}"),
            Step::Remove(label) => format!("{label}-"),
        };

        hash(&v)
    }
}

fn hash(v: &str) -> u8 {
    v.chars()
        .fold(0, |acc, c| acc.wrapping_add(c as u8).wrapping_mul(17))
}

fn problem1(input: &Input) -> u32 {
    input.iter().map(|x| x.hash() as u32).sum()
}

fn problem2(input: &Input) -> usize {
    let mut boxes: BTreeMap<u8, Vec<(&str, u8)>> = BTreeMap::new();

    // follow all the instructions to construct the box map
    for x in input {
        let box_number = x.hash_label();
        let v = boxes.entry(box_number).or_default();

        match x {
            Step::Insert(label, value) => {
                let existing_label = v.iter_mut().find(|x| x.0 == *label);
                if let Some((_, old_value)) = existing_label {
                    // a lens with this label is already there
                    *old_value = *value;
                } else {
                    // just add the new lens
                    v.push((label, *value));
                }
            }
            Step::Remove(label) => {
                // if a lens with the label is there, remove it
                if let Some(idx) = v.iter().position(|x| x.0 == *label) {
                    v.remove(idx);
                };
            }
        }
    }

    // now that the lenses are in, calculate the focal length
    boxes
        .into_iter()
        .flat_map(|(box_num, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(move |(slot, (_label, length))| (box_num as usize, slot, length as usize))
        })
        .map(|(box_num, slot, length)| (box_num + 1) * (slot + 1) * length)
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
        assert_eq!(result, 1320)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 145)
    }
}
