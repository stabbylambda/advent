use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline, u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input<'a> = (Workflows<'a>, Vec<Part>);

fn parse(input: &str) -> Input {
    let workflows = map(
        separated_list1(
            newline,
            tuple((
                alpha1,
                delimited(tag("{"), separated_list1(tag(","), Rule::parse), tag("}")),
            )),
        ),
        Workflows::new,
    );

    let parts = separated_list1(newline, Part::parse);

    let result: IResult<&str, Input> = separated_pair(workflows, tag("\n\n"), parts)(input);

    result.unwrap().1
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Xmas {
    X,
    M,
    A,
    S,
}

impl Xmas {
    fn parse(input: &str) -> IResult<&str, Xmas> {
        alt((
            map(char('x'), |_| Xmas::X),
            map(char('m'), |_| Xmas::M),
            map(char('a'), |_| Xmas::A),
            map(char('s'), |_| Xmas::S),
        ))(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Comparison {
    Greater,
    Less,
}

impl Comparison {
    fn parse(input: &str) -> IResult<&str, Comparison> {
        alt((
            map(char('>'), |_| Comparison::Greater),
            map(char('<'), |_| Comparison::Less),
        ))(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Rule<'a> {
    Branch(Xmas, Comparison, u32, &'a str),
    Fallthrough(&'a str),
}

impl<'a> Rule<'a> {
    fn parse(input: &str) -> IResult<&str, Rule> {
        alt((
            map(
                tuple((
                    Xmas::parse,
                    Comparison::parse,
                    u32,
                    preceded(tag(":"), alpha1),
                )),
                |(xmas, comp, num, result)| Rule::Branch(xmas, comp, num, result),
            ),
            map(alpha1, |x: &str| Rule::Fallthrough(x)),
        ))(input)
    }

    fn evaluate(&self, part: &Part) -> Option<&str> {
        match self {
            Rule::Branch(xmas, comp, num, result) => {
                let value = part.attributes[xmas];
                let test = match comp {
                    Comparison::Greater => value > *num,
                    Comparison::Less => value < *num,
                };

                if test {
                    Some(result)
                } else {
                    None
                }
            }
            Rule::Fallthrough(result) => Some(result),
        }
    }
}

#[derive(Debug)]
struct Part {
    attributes: BTreeMap<Xmas, u32>,
}

impl Part {
    fn new(attributes: Vec<(Xmas, u32)>) -> Part {
        Part {
            attributes: attributes.into_iter().collect(),
        }
    }

    fn parse(input: &str) -> IResult<&str, Part> {
        map(
            delimited(
                tag("{"),
                separated_list1(tag(","), separated_pair(Xmas::parse, tag("="), u32)),
                tag("}"),
            ),
            Part::new,
        )(input)
    }

    fn rating(&self) -> u32 {
        self.attributes.values().sum()
    }
}

#[derive(Debug)]
struct Workflows<'a> {
    map: BTreeMap<&'a str, Vec<Rule<'a>>>,
}

impl<'a> Workflows<'a> {
    fn new(workflows: Vec<(&'a str, Vec<Rule<'a>>)>) -> Self {
        Self {
            map: workflows.into_iter().collect(),
        }
    }

    fn evalute(&self, p: &Part) -> bool {
        // always start at "in"
        let mut current = "in";

        loop {
            let rules = &self.map[&current];
            let result = rules.iter().find_map(|x| x.evaluate(p)).unwrap();

            // bail if we hit A or R
            match result {
                "A" => return true,
                "R" => return false,
                _ => current = result,
            };
        }
    }
}

fn problem1(input: &Input) -> u32 {
    let (workflows, parts) = input;

    parts
        .iter()
        .filter_map(|p| workflows.evalute(p).then_some(p.rating()))
        .sum()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 19114)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
