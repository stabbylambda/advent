use std::{collections::BTreeMap, ops::Range};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline, u32},
    combinator::map,
    multi::separated_list1,
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

fn parse(input: &str) -> Input<'_> {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    fn parse(input: &str) -> IResult<&str, Rule<'_>> {
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

    fn evaluate_part(&self, part: &Part) -> Option<&str> {
        match self {
            Rule::Branch(xmas, comp, num, result) => {
                let value = part.attributes[xmas];
                let test = match comp {
                    Comparison::Greater => value > *num,
                    Comparison::Less => value < *num,
                };

                test.then_some(result)
            }
            Rule::Fallthrough(result) => Some(result),
        }
    }

    fn evaluate_ranged_part(&self, rp: &RangedPart) -> (RangedPart, RangedPart) {
        let mut accepted_part = rp.clone();
        let mut rejected_part = rp.clone();

        // we only care about branches, not the fallthrough
        if let Rule::Branch(xmas, comp, val, _target) = self {
            let attribute = &rp.attributes[xmas];
            let val = *val as usize;

            let (accepted_range, rejected_range) = match comp {
                Comparison::Greater => {
                    let accepted = val + 1..attribute.end;
                    let rejected = attribute.start..val + 1;

                    (accepted, rejected)
                }
                Comparison::Less => {
                    let accepted = attribute.start..val;
                    let rejected = val..attribute.end;

                    (accepted, rejected)
                }
            };

            accepted_part.attributes.insert(*xmas, accepted_range);
            rejected_part.attributes.insert(*xmas, rejected_range);
        }

        (accepted_part, rejected_part)
    }

    fn target(&self) -> &str {
        match self {
            Rule::Branch(_, _, _, t) => t,
            Rule::Fallthrough(t) => t,
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

    fn evalute_part(&self, rule: &str, p: &Part) -> bool {
        match rule {
            "A" => true,
            "R" => false,
            _ => {
                let rules = &self.map[&rule];
                let result = rules.iter().find_map(|x| x.evaluate_part(p)).unwrap();
                self.evalute_part(result, p)
            }
        }
    }

    /** Do a DFS on the workflows, going down each path and splitting the ranges based on the conditions */
    fn evaluate_ranged_part(&self, rule: &str, p: &RangedPart) -> usize {
        match rule {
            "A" => p.count(),
            "R" => 0,
            _ => {
                let rules = &self.map[rule];
                let (total, _) = rules.iter().fold((0, p.clone()), |(total, rejected), r| {
                    // split the ranges into two based on the rule
                    let (accepted, rejected) = r.evaluate_ranged_part(&rejected);
                    let total = total + self.evaluate_ranged_part(r.target(), &accepted);

                    (total, rejected)
                });
                total
            }
        }
    }
}

#[derive(Clone, Debug)]
struct RangedPart {
    attributes: BTreeMap<Xmas, Range<usize>>,
}

impl RangedPart {
    fn new() -> Self {
        let attributes = [
            (Xmas::X, 1..4001),
            (Xmas::M, 1..4001),
            (Xmas::A, 1..4001),
            (Xmas::S, 1..4001),
        ];
        Self {
            attributes: attributes.into_iter().collect(),
        }
    }

    fn count(&self) -> usize {
        self.attributes
            .values()
            .map(|x| x.clone().count())
            .product()
    }
}

fn problem1(input: &Input) -> u32 {
    let (workflows, parts) = input;

    parts
        .iter()
        .filter_map(|p| workflows.evalute_part("in", p).then_some(p.rating()))
        .sum()
}

fn problem2(input: &Input) -> usize {
    let (workflows, _) = input;

    workflows.evaluate_ranged_part("in", &RangedPart::new())
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
        assert_eq!(result, 167409079868000)
    }
}
