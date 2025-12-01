use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i32 as nom_i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let mut input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&mut input);
    println!("problem 2 answer: {answer}");
}

#[derive(Copy, Clone, Debug)]
enum Data<'a> {
    Constant(i32),
    Wire(&'a str),
}

#[derive(Copy, Clone, Debug)]
enum Gate<'a> {
    Constant(Data<'a>),
    And((Data<'a>, Data<'a>)),
    Or((Data<'a>, Data<'a>)),
    Not(Data<'a>),
    Lshift((Data<'a>, Data<'a>)),
    Rshift((Data<'a>, Data<'a>)),
}

type Input<'a> = HashMap<&'a str, Gate<'a>>;

fn data(input: &str) -> IResult<&str, Data<'_>> {
    alt((map(nom_i32, Data::Constant), map(alpha1, Data::Wire)))(input)
}

fn gate(input: &str) -> IResult<&str, Gate<'_>> {
    alt((
        map(separated_pair(data, tag(" AND "), data), Gate::And),
        map(separated_pair(data, tag(" OR "), data), Gate::Or),
        map(separated_pair(data, tag(" LSHIFT "), data), Gate::Lshift),
        map(separated_pair(data, tag(" RSHIFT "), data), Gate::Rshift),
        map(preceded(tag("NOT "), data), Gate::Not),
        map(data, Gate::Constant),
    ))(input)
}

fn parse(input: &str) -> Input<'_> {
    let result: IResult<&str, Vec<(&str, Gate)>> = separated_list1(
        newline,
        map(separated_pair(gate, tag(" -> "), alpha1), |(gate, wire)| {
            (wire, gate)
        }),
    )(input);

    result.unwrap().1.into_iter().collect()
}

struct Kit<'a> {
    circuits: &'a HashMap<&'a str, Gate<'a>>,
    cache: HashMap<&'a str, i32>,
}

impl<'a> Kit<'a> {
    fn new(circuits: &'a HashMap<&'a str, Gate<'a>>) -> Self {
        Kit {
            circuits,
            cache: HashMap::new(),
        }
    }

    fn evaluate_wire(&mut self, wire: &'a str) -> i32 {
        if let Some(v) = self.cache.get(wire) {
            return *v;
        }

        let result = match &self.circuits[wire] {
            Gate::Constant(a) => self.evaluate(a),
            Gate::And((a, b)) => {
                let a = self.evaluate(a);
                let b = self.evaluate(b);

                a & b
            }
            Gate::Or((a, b)) => {
                let a = self.evaluate(a);
                let b = self.evaluate(b);

                a | b
            }
            Gate::Not(a) => {
                let a = self.evaluate(a);
                !a
            }
            Gate::Lshift((a, b)) => {
                let a = self.evaluate(a);
                let b = self.evaluate(b);

                a << b
            }
            Gate::Rshift((a, b)) => {
                let a = self.evaluate(a);
                let b = self.evaluate(b);

                a >> b
            }
        };

        self.cache.insert(wire, result);

        result
    }

    fn evaluate(&mut self, data: &'a Data) -> i32 {
        match data {
            Data::Constant(x) => *x,
            Data::Wire(wire) => self.evaluate_wire(wire),
        }
    }
}

fn problem1(input: &Input) -> i32 {
    let mut kit = Kit::new(input);
    kit.evaluate(&Data::Wire("a"))
}

fn problem2(input: &mut Input) -> i32 {
    let mut kit = Kit::new(input);
    let a_value = kit.evaluate(&Data::Wire("a"));

    input
        .entry("b")
        .and_modify(|v| *v = Gate::Constant(Data::Constant(a_value)));

    let mut kit = Kit::new(input);
    kit.evaluate(&Data::Wire("a"))
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 114)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let mut input = parse(input);
        let result = problem2(&mut input);
        assert_eq!(result, 114)
    }
}
