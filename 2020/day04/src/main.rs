use std::collections::{BTreeSet, HashMap};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, one_of, u32},
    combinator::map,
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair, terminated},
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

type Input = Vec<Passport>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        tag("\n\n"),
        map(
            separated_list1(
                alt((tag(" "), tag("\n"))),
                separated_pair(alpha1, tag(":"), take_till(|x: char| x.is_whitespace())),
            ),
            Passport::new,
        ),
    )(input);

    result.unwrap().1
}
enum Height {
    Cm(u32),
    In(u32),
}

fn parse_height(input: &str) -> IResult<&str, Height> {
    alt((
        map(terminated(u32, tag("cm")), Height::Cm),
        map(terminated(u32, tag("in")), Height::In),
    ))(input)
}

fn parse_color(input: &str) -> IResult<&str, Vec<char>> {
    preceded(tag("#"), count(one_of("0123456789abcdef"), 6))(input)
}

fn parse_pid(input: &str) -> IResult<&str, Vec<char>> {
    count(one_of("0123456789"), 9)(input)
}

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

impl Passport {
    fn new<'a>(v: Vec<(&'a str, &'a str)>) -> Self {
        Self {
            fields: v
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }

    fn is_valid_basic(&self) -> bool {
        let my_fields = self
            .fields
            .keys()
            .map(|x| x.as_str())
            .collect::<BTreeSet<_>>();

        let required_fields = FIELDS.into_iter().collect::<BTreeSet<_>>();

        my_fields.is_superset(&required_fields)
    }

    fn field_between(&self, field: &str, low: u32, high: u32) -> bool {
        self.fields
            .get(field)
            .and_then(|x| x.parse::<u32>().ok())
            .map(|x| (low..=high).contains(&x))
            .unwrap_or(false)
    }

    fn is_valid(&self) -> bool {
        if !self.is_valid_basic() {
            return false;
        }

        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        let byr_valid = self.field_between("byr", 1920, 2002);
        let iyr_valid = self.field_between("iyr", 2010, 2020);
        let eyr_valid = self.field_between("eyr", 2020, 2030);

        // hgt (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        let hgt_valid = self
            .fields
            .get("hgt")
            .and_then(|x| parse_height(x).ok())
            .map(|(left, x)| match x {
                Height::Cm(x) => (150..=193).contains(&x),
                Height::In(x) => (59..=76).contains(&x),
            } && left.is_empty())
            .unwrap_or(false);

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let hcl_valid = self
            .fields
            .get("hcl")
            .and_then(|x| parse_color(x).ok())
            .map(|x| x.0.is_empty() && x.1.len() == 6)
            .unwrap_or(false);

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let ecl_valid = self
            .fields
            .get("ecl")
            .map(|x| {
                matches!(
                    x.as_str(),
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                )
            })
            .unwrap_or(false);

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        let pid_valid = self
            .fields
            .get("pid")
            .and_then(|x| parse_pid(x).ok())
            .map(|x| x.0.is_empty())
            .unwrap_or(false);

        byr_valid && iyr_valid && eyr_valid && hgt_valid && ecl_valid && hcl_valid && pid_valid
    }
}

fn problem1(input: &Input) -> usize {
    input.iter().filter(|x| x.is_valid_basic()).count()
}

fn problem2(input: &Input) -> usize {
    input.iter().filter(|x| x.is_valid()).count()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 2)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 4)
    }
}
