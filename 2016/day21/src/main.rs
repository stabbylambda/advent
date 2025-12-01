use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input, "abcdefgh");
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input, "fbgdceah");
    println!("problem 2 answer: {answer}");
}

#[derive(Debug)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOn(char, bool),
    ReversePositions(usize, usize),
    MovePosition(usize, usize),
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Instruction> {
        let swap_position = map(
            preceded(
                tag("swap position "),
                separated_pair(u32, tag(" with position "), u32),
            ),
            |(x, y)| Instruction::SwapPosition(x as usize, y as usize),
        );

        let swap_letter = map(
            preceded(
                tag("swap letter "),
                separated_pair(anychar, tag(" with letter "), anychar),
            ),
            |(x, y)| Instruction::SwapLetter(x, y),
        );

        let rotate_right = map(
            delimited(
                tag("rotate right "),
                u32,
                alt((tag(" steps"), tag(" step"))),
            ),
            |x| Instruction::RotateRight(x as usize),
        );

        let rotate_left = map(
            delimited(tag("rotate left "), u32, alt((tag(" steps"), tag(" step")))),
            |x| Instruction::RotateLeft(x as usize),
        );

        let rotate_based = map(
            preceded(tag("rotate based on position of letter "), anychar),
            |x| Instruction::RotateBasedOn(x, true),
        );

        let reverse_positions = map(
            preceded(
                tag("reverse positions "),
                separated_pair(u32, tag(" through "), u32),
            ),
            |(x, y)| Instruction::ReversePositions(x as usize, y as usize),
        );

        let move_position = map(
            preceded(
                tag("move position "),
                separated_pair(u32, tag(" to position "), u32),
            ),
            |(x, y)| Instruction::MovePosition(x as usize, y as usize),
        );

        alt((
            swap_position,
            swap_letter,
            rotate_right,
            rotate_left,
            rotate_based,
            reverse_positions,
            move_position,
        )).parse(input)
    }

    fn execute(&self, cs: &mut Vec<char>) {
        match *self {
            Self::SwapPosition(x, y) => {
                cs.swap(x, y);
            }
            Self::SwapLetter(a, b) => {
                let pa = cs.iter().position(|&c| c == a).unwrap();
                let pb = cs.iter().position(|&c| c == b).unwrap();

                cs.swap(pa, pb);
            }
            Self::RotateLeft(x) => {
                cs.rotate_left(x);
            }
            Self::RotateRight(x) => {
                cs.rotate_right(x);
            }
            Self::RotateBasedOn(x, true) => {
                let pos = cs.iter().position(|&c| c == x).unwrap();
                let skip = (pos + if pos >= 4 { 2 } else { 1 }) % cs.len();
                cs.rotate_right(skip);
            }
            Self::RotateBasedOn(x, false) => {
                let pos = cs.iter().position(|&c| c == x).unwrap();
                let skip = (pos / 2) + if pos % 2 == 1 || pos == 0 { 1 } else { 5 };
                cs.rotate_left(skip);
            }
            Self::ReversePositions(x, y) => {
                let sub: Vec<char> = cs[x..=y].iter().copied().rev().collect();
                cs[x..(sub.len() + x)].copy_from_slice(&sub[..]);
            }
            Self::MovePosition(x, y) => {
                let c = cs.remove(x);
                cs.insert(y, c);
            }
        }
    }

    fn inverse(&self) -> Instruction {
        match *self {
            Self::SwapPosition(x, y) => Self::SwapPosition(x, y),
            Self::SwapLetter(x, y) => Self::SwapLetter(y, x),
            Self::RotateLeft(x) => Self::RotateRight(x),
            Self::RotateRight(x) => Self::RotateLeft(x),
            Self::RotateBasedOn(x, encrypt) => Self::RotateBasedOn(x, !encrypt),
            Self::ReversePositions(x, y) => Self::ReversePositions(x, y),
            Self::MovePosition(x, y) => Self::MovePosition(y, x),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    let result: IResult<&str, Vec<Instruction>> =
        separated_list1(newline, Instruction::parse).parse(input);
    result.unwrap().1
}

fn execute_all(input: &[Instruction], password: &str) -> String {
    let mut cs: Vec<char> = password.chars().collect();
    for i in input {
        i.execute(&mut cs);
    }

    cs.iter().collect()
}
fn problem1(input: &[Instruction], password: &str) -> String {
    execute_all(input, password)
}

fn problem2(input: &[Instruction], password: &str) -> String {
    let instructions: Vec<Instruction> = input.iter().rev().map(|i| i.inverse()).collect();
    execute_all(&instructions, password)
}

#[test]
fn rot() {
    assert_eq!(problem1(&[Instruction::RotateRight(1)], "abcd"), "dabc");
    assert_eq!(problem1(&[Instruction::RotateLeft(1)], "abcd"), "bcda");
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input, "abcde");
        assert_eq!(result, "decab")
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let encrypted = problem1(&input, "abcde");
        let result = problem2(&input, &encrypted);
        assert_eq!(result, "abcde")
    }
}
