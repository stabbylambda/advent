use common::{answer, read_input};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Equation = Vec<EquationPart>;
type Input = Vec<Equation>;

#[derive(Clone, Debug)]
enum EquationPart {
    Plus,
    Times,
    Literal(u64),
    SubEquation(Equation),
}

fn equation(s: &str) -> IResult<&str, Equation> {
    separated_list1(
        tag(" "),
        alt((
            map(char('+'), |_| EquationPart::Plus),
            map(char('*'), |_| EquationPart::Times),
            map(u64, EquationPart::Literal),
            map(
                delimited(tag("("), equation, tag(")")),
                EquationPart::SubEquation,
            ),
        )),
    ).parse(s)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, equation).parse(input);

    result.unwrap().1
}

fn solve(e: &Equation) -> u64 {
    if e.len() == 1 {
        if let EquationPart::Literal(x) = e[0] {
            return x;
        }
    }
    e.chunks(2)
        .fold((0, EquationPart::Plus), |(acc, op), x| {
            let new_op = x.get(1).unwrap_or(&EquationPart::Plus).clone();

            let rhs = match &x[0] {
                EquationPart::Literal(x) => *x,
                EquationPart::SubEquation(eq) => solve(eq),
                _ => unreachable!(),
            };

            let acc = match op {
                EquationPart::Plus => acc + rhs,
                EquationPart::Times => acc * rhs,
                _ => unreachable!(),
            };

            (acc, new_op)
        })
        .0
}

fn problem1(input: &[Equation]) -> u64 {
    input.iter().map(solve).sum()
}

fn problem2(input: &[Equation]) -> u64 {
    let input: Vec<Equation> = input.iter().map(|eq| advanced_process(eq)).collect();
    input.iter().map(solve).sum()
}

fn advanced_process(eq: &[EquationPart]) -> Vec<EquationPart> {
    let (mut acc, sub) = eq.iter().fold((vec![], vec![]), |(mut acc, mut sub), p| {
        match p {
            EquationPart::Times => {
                acc.push(EquationPart::SubEquation(sub.clone()));
                acc.push(p.clone());
                sub.clear();
            }
            EquationPart::SubEquation(s) => {
                let inner = advanced_process(s);
                sub.push(EquationPart::SubEquation(inner));
            }
            _ => {
                sub.push(p.clone());
            }
        }
        (acc, sub)
    });

    acc.push(EquationPart::SubEquation(sub));
    acc
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, [71, 51, 26, 437, 12240, 13632].iter().sum())
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, [231, 51, 46, 1445, 669060, 23340].iter().sum())
    }
}
