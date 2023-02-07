use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64 as nom_i64, newline},
    combinator::map,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

#[derive(Debug, Clone, Copy)]
enum MonkeyValue<'a> {
    Literal(Option<i64>),
    Plus(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Times(&'a str, &'a str),
    Divides(&'a str, &'a str),
    Equals(&'a str, &'a str),
}

impl<'a> MonkeyValue<'a> {
    fn get_dependencies(&self) -> Option<(&'a str, &'a str)> {
        match self {
            MonkeyValue::Equals(l, r) => Some((*l, *r)),
            MonkeyValue::Plus(l, r) => Some((*l, *r)),
            MonkeyValue::Minus(l, r) => Some((*l, *r)),
            MonkeyValue::Times(l, r) => Some((*l, *r)),
            MonkeyValue::Divides(l, r) => Some((*l, *r)),
            MonkeyValue::Literal(_) => None,
        }
    }
}

type Input<'a> = Vec<(&'a str, MonkeyValue<'a>)>;

fn monkey_value(input: &str) -> IResult<&str, MonkeyValue> {
    alt((
        map(nom_i64, |x| MonkeyValue::Literal(Some(x))),
        map(separated_pair(alpha1, tag(" + "), alpha1), |(lhs, rhs)| {
            MonkeyValue::Plus(lhs, rhs)
        }),
        map(separated_pair(alpha1, tag(" - "), alpha1), |(lhs, rhs)| {
            MonkeyValue::Minus(lhs, rhs)
        }),
        map(separated_pair(alpha1, tag(" * "), alpha1), |(lhs, rhs)| {
            MonkeyValue::Times(lhs, rhs)
        }),
        map(separated_pair(alpha1, tag(" / "), alpha1), |(lhs, rhs)| {
            MonkeyValue::Divides(lhs, rhs)
        }),
    ))(input)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list0(newline, separated_pair(alpha1, tag(": "), monkey_value))(input);

    result.unwrap().1
}

struct Equation<'a> {
    map: HashMap<&'a str, MonkeyValue<'a>>,
}

impl<'a> Equation<'a> {
    fn new(input: &'a Input) -> Equation<'a> {
        let map = input.iter().copied().collect();
        Equation { map }
    }

    fn evaluate(&self, current: &str) -> Option<i64> {
        let current = self.map[current];

        if let MonkeyValue::Literal(x) = current {
            return x;
        }

        match current {
            MonkeyValue::Plus(l, r) => self
                .evaluate(l)
                .and_then(|l| self.evaluate(r).map(|r| l + r)),
            MonkeyValue::Minus(l, r) => self
                .evaluate(l)
                .and_then(|l| self.evaluate(r).map(|r| l - r)),
            MonkeyValue::Times(l, r) => self
                .evaluate(l)
                .and_then(|l| self.evaluate(r).map(|r| l * r)),
            MonkeyValue::Divides(l, r) => self
                .evaluate(l)
                .and_then(|l| self.evaluate(r).map(|r| l / r)),
            _ => unreachable!("Something went wrong"),
        }
    }

    fn solve(&self, id: &str, value_to_solve: Option<i64>) -> Option<i64> {
        // We're here, so just return the value to solve, we've already solved it
        if id == "humn" {
            return value_to_solve;
        }

        let monkey = self.map[id];
        let Some((l, r)) = monkey.get_dependencies() else { panic!(); };

        // one of these will be none and the other will be a solved monkey
        let left = self.evaluate(l);
        let right = self.evaluate(r);

        let (unknown_id, known) = match (left, right) {
            (None, Some(x)) => (l, x),
            (Some(x), None) => (r, x),
            _ => panic!("No idea what went wrong here"),
        };

        // this is the starting point, so go ahead and recurse on the unknown side of the equation
        if id == "root" {
            return self.solve(unknown_id, Some(known));
        }

        let value_to_solve = value_to_solve.map(|value| match monkey {
            MonkeyValue::Plus(_, _) => value - known,
            MonkeyValue::Minus(_, _) if l == unknown_id => value + known,
            MonkeyValue::Minus(_, _) if r == unknown_id => known - value,
            MonkeyValue::Divides(_, _) if l == unknown_id => value * known,
            MonkeyValue::Divides(_, _) if r == unknown_id => known / value,
            MonkeyValue::Times(_, _) => value / known,

            _ => unreachable!(),
        });

        self.solve(unknown_id, value_to_solve)
    }
}

fn problem1(input: &Input) -> i64 {
    let equation = Equation::new(input);
    equation.evaluate("root").unwrap()
}

fn problem2(input: &Input) -> i64 {
    let mut equation = Equation::new(input);

    // set root to an Equals Monkey
    let root = equation.map.get_mut("root").unwrap();
    let Some((l, r)) = root.get_dependencies() else { panic!()};
    *root = MonkeyValue::Equals(l, r);

    // set humn to None so the whole tree and any tree that contains it will eval to None
    let humn = equation.map.get_mut("humn").unwrap();
    *humn = MonkeyValue::Literal(None);

    // keep solving "the other side" of the tree so we can get a literal to factor out of the other side of the equality check
    equation.solve("root", None).unwrap()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 152)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 301)
    }
}
