use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, char, newline, not_line_ending, u32 as nom_u32};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::{
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};
fn main() {
    let raw = include_str!("../input.txt");
    let mut input = Input::parse(raw);
    let mut input2 = input.clone();

    let score = problem1(&mut input);
    println!("problem 1 score: {score}");

    let score = problem2(&mut input2);
    println!("problem 2 score: {score}");
}

type Stack<'a> = Vec<&'a str>;

#[derive(Clone, Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn parse(s: &str) -> IResult<&str, Move> {
        map(
            tuple((
                preceded(tag("move "), nom_u32),
                preceded(tag(" from "), nom_u32),
                preceded(tag(" to "), nom_u32),
            )),
            |(count, from, to)| Move {
                count: count as usize,
                from: (from - 1) as usize,
                to: (to - 1) as usize,
            },
        )(s)
    }
}

#[derive(Clone, Debug)]
struct Input<'a> {
    stacks: Vec<Stack<'a>>,
    moves: Vec<Move>,
}

impl Input<'_> {
    fn parse_crate(s: &str) -> IResult<&str, Option<&str>> {
        alt((
            map(delimited(char('['), alpha0, char(']')), Some),
            map(tag("   "), |_| None),
        ))(s)
    }

    fn parse_row(s: &str) -> IResult<&str, Vec<Option<&str>>> {
        separated_list0(tag(" "), Input::parse_crate)(s)
    }

    fn invert_stacks(rows: Vec<Vec<Option<&str>>>) -> Vec<Stack> {
        let stack_count = rows.first().unwrap().len();
        (0..stack_count)
            .map(|n| rows.iter().rev().filter_map(|row| row[n]).collect())
            .collect()
    }

    fn parse_stacks(s: &str) -> IResult<&str, Vec<Stack>> {
        map(
            many0(terminated(Input::parse_row, newline)),
            Input::invert_stacks,
        )(s)
    }

    fn parse(raw: &str) -> Input {
        map(
            separated_pair(
                Input::parse_stacks,
                terminated(not_line_ending, tag("\n\n")),
                separated_list0(newline, Move::parse),
            ),
            |(stacks, moves)| Input { stacks, moves },
        )(raw)
        .unwrap()
        .1
    }

    fn print_tops(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.last().unwrap().to_owned())
            .collect()
    }
}

fn problem1(input: &mut Input) -> String {
    for m in &input.moves {
        let from_stack = input.stacks.get_mut(m.from).unwrap();

        let mut crane: Vec<&str> = from_stack.drain((from_stack.len() - m.count)..).collect();
        crane.reverse();

        let to_stack = input.stacks.get_mut(m.to).unwrap();
        to_stack.append(&mut crane);
    }

    input.print_tops()
}

fn problem2(input: &mut Input) -> String {
    for m in &input.moves {
        let from_stack = input.stacks.get_mut(m.from).unwrap();

        let mut crane = from_stack.drain((from_stack.len() - m.count)..).collect();

        let to_stack = input.stacks.get_mut(m.to).unwrap();
        to_stack.append(&mut crane);
    }

    input.print_tops()
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2, Input};
    #[test]
    fn first() {
        let raw = include_str!("../test.txt");
        let mut input = Input::parse(raw);
        let result = problem1(&mut input);
        assert_eq!(result, "CMZ")
    }

    #[test]
    fn second() {
        let raw = include_str!("../test.txt");
        let mut input = dbg!(Input::parse(raw));
        let result = problem2(&mut input);
        assert_eq!(result, "MCD")
    }
    #[test]
    fn testparse() {
        assert_eq!(Input::parse_crate("[D]").unwrap().1, Some("D"));
        assert_eq!(Input::parse_crate("   ").unwrap().1, None);

        let expected = vec![Some("Z"), Some("M"), Some("P")];
        assert_eq!(Input::parse_row("[Z] [M] [P]").unwrap().1, expected);

        let expected = vec![None, Some("D"), None];
        assert_eq!(Input::parse_row("    [D]    ").unwrap().1, expected);
    }
}
